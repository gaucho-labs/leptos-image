// Checks to see if image request is valid / already cached.
// If new request, it will cache the image and return the cached image.
// If cached, it will return the cached image.
#[cfg(feature = "ssr")]
pub mod handlers {
    use crate::add_image_cache;
    use crate::optimizer::{CachedImage, CachedImageOption, CreateImageError};
    use axum::response::Response as AxumResponse;
    use axum::{
        body::Body,
        extract::State,
        http::{self, Request, Response, Uri},
        response::IntoResponse,
    };
    use leptos::LeptosOptions;
    use std::convert::Infallible;
    use tower::ServiceExt;
    use tower_http::services::fs::ServeFileSystemResponseBody;
    use tower_http::services::ServeDir;

    /// Returns the cached image if it exists.
    ///
    /// Example:
    /// ```
    ///  // Create Axum Router.
    ///  let app = Router::new();
    ///  // Add route for image cache.
    ///  app.route("/cache/image", get(image_cache_handler))
    /// ```
    ///
    pub async fn image_cache_handler(
        State(options): State<LeptosOptions>,
        req: Request<Body>,
    ) -> AxumResponse {
        let root = options.site_root.clone();
        let cache_result = check_cache_image(req.uri().clone(), &root).await;

        match cache_result {
            Ok(Some(uri)) => {
                let mut response = execute_file_handler(uri, &root).await.unwrap();
                let cache_control =
                    format!("public, stale-while-revalidate, max-age={}", 60 * 60 * 24);
                let headers = response.headers_mut();
                headers.append(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_str(&cache_control).unwrap(),
                );
                response.into_response()
            }

            Ok(None) => Response::builder()
                .status(404)
                .body("Invalid Image.".to_string())
                .unwrap()
                .into_response(),

            Err(e) => {
                tracing::error!("Failed to create image: {:?}", e);
                Response::builder()
                    .status(500)
                    .body("Error creating image".to_string())
                    .unwrap()
                    .into_response()
            }
        }
    }

    async fn execute_file_handler(
        uri: Uri,
        root: &str,
    ) -> Result<Response<ServeFileSystemResponseBody>, Infallible> {
        let req = Request::builder()
            .uri(uri.clone())
            .body(Body::empty())
            .unwrap();
        ServeDir::new(root).oneshot(req).await
    }

    async fn check_cache_image(uri: Uri, root: &str) -> Result<Option<Uri>, CreateImageError> {
        let url = uri.to_string();
        let maybe_cache_image = CachedImage::from_url_encoded(&url).ok();

        let cache_image = {
            if let Some(img) = maybe_cache_image {
                let _ = img.create_image(root).await?;
                img
            } else {
                return Ok(None);
            }
        };

        let file_path = cache_image.get_file_path_from_root(root);

        add_file_to_cache(root, cache_image).await;
        let uri_string = "/".to_string() + &file_path;
        let maybe_uri = (uri_string).parse::<Uri>().ok();

        if let Some(uri) = maybe_uri {
            Ok(Some(uri))
        } else {
            tracing::error!("Failed to create uri: File path {file_path}");
            Ok(None)
        }
    }

    // When the image is created, it will be added to the cache.
    // Mostly helpful for dev server startup.
    async fn add_file_to_cache(root: &str, image: CachedImage) {
        if let CachedImageOption::Blur(_) = image.option {
            add_image_cache(root, vec![image]).await;
            return;
        }
    }
}
