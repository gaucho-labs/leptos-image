// Checks to see if image request is valid / already cached.
// If new request, it will cache the image and return the cached image.
// If cached, it will return the cached image.
#[cfg(feature = "ssr")]
pub mod handlers {
    use crate::image_service::CachedImage;
    use axum::response::Response as AxumResponse;
    use axum::{
        body::boxed,
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

    pub async fn image_cache_handler(
        State(options): State<LeptosOptions>,
        req: Request<Body>,
    ) -> AxumResponse {
        let root = options.site_root.clone();
        let cache_result = check_cache_image(req.uri().clone(), &root).await;

        if let Some(uri) = cache_result {
            println!("Cache result: {:?}", uri);
            let mut response = execute_file_handler(uri, &root).await.unwrap();
            let cache_control = format!("public, stale-while-revalidate, max-age={}", 60 * 60 * 24);
            let headers = response.headers_mut();
            headers.append(
                http::header::CACHE_CONTROL,
                http::HeaderValue::from_str(&cache_control).unwrap(),
            );

            response.map(boxed).into_response()
        } else {
            Response::builder()
                .status(404)
                .body("Invalid Image.".to_string())
                .unwrap()
                .map(boxed)
                .into_response()
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
        // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
        // This path is relative to the cargo root
        ServeDir::new(root).oneshot(req).await
    }

    async fn check_cache_image(uri: Uri, root: &str) -> Option<Uri> {
        let url = uri.to_string();
        let maybe_cache_image = CachedImage::from_url_encoded(&url).ok();

        // println!("Checking cache image: {url}, {:?}", maybe_cache_image);

        let maybe_created = {
            if let Some(img) = maybe_cache_image {
                Some(img.create_image(root).await)
            } else {
                None
            }
        };

        if let Some(Ok(file_path)) = maybe_created {
            let new_uri = ("/".to_string() + &file_path).parse::<Uri>().unwrap();
            Some(new_uri)
        } else {
            leptos::error!("FAILED TO CREATE CACHE IMAGE {} - {:?}", url, maybe_created);
            None
        }
    }
}
