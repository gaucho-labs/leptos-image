// Checks to see if image request is valid / already cached.
// If new request, it will cache the image and return the cached image.
// If cached, it will return the cached image.
#[cfg(feature = "ssr")]
pub mod handlers {
    use crate::add_image_cache;
    use crate::optimizer::{CachedImage, CachedImageOption, CreateImageError};
    use actix_files::NamedFile;
    use actix_web::body::{BoxBody, MessageBody};
    use actix_web::http::header::HeaderValue;

    use actix_web::http::{self, StatusCode, Uri};
    use actix_web::HttpRequest as ActixRequest;
    use actix_web::{Error, HttpResponse as ActixResponse};

    use std::path::PathBuf;

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
    pub async fn image_cache_handler(root: String, req: ActixRequest) -> ActixResponse {
        let root = root.clone();
        let cache_result = check_cache_image(req.uri().clone(), &root).await;

        match cache_result {
            Ok(Some(uri)) => {
                let file = execute_file_handler(uri, &root, req)
                    .await
                    .expect("couldn't get file");
                let cache_control =
                    format!("public, stale-while-revalidate, max-age={}", 60 * 60 * 24);

                let mut response = file;
                response.headers_mut().insert(
                    http::header::CACHE_CONTROL,
                    HeaderValue::from_str(&cache_control).unwrap(),
                );
                response
            }

            Ok(None) => {
                let body = BoxBody::new(MessageBody::boxed("Invalid Image.".to_string()));
                ActixResponse::with_body(StatusCode::NOT_FOUND, body)
            }

            Err(e) => {
                log::error!("Failed to create image: {:?}", e);
                let body = BoxBody::new(MessageBody::boxed("Error creating image.".to_string()));
                ActixResponse::with_body(StatusCode::NOT_FOUND, body)
            }
        }
    }

    async fn execute_file_handler(
        uri: Uri,
        root: &str,
        req: ActixRequest,
    ) -> Result<ActixResponse, Error> {
        let file_path = PathBuf::from(format!("{}/{}", root, uri));
        let named_file = NamedFile::open(file_path)?;

        // Convert NamedFile into HttpResponse
        let response = named_file.into_response(&req);
        Ok(response)
    }

    async fn check_cache_image(uri: Uri, root: &str) -> Result<Option<Uri>, CreateImageError> {
        let url = uri.to_string();
        let maybe_cache_image = CachedImage::from_url_encoded(&url).ok();

        let maybe_created = {
            if let Some(ref img) = maybe_cache_image {
                Some(img.create_image(root).await)
            } else {
                None
            }
        };

        match maybe_created {
            Some(Ok((file_path, created))) => {
                if created && maybe_cache_image.is_some() {
                    add_file_to_cache(root, maybe_cache_image.unwrap()).await;
                }
                let new_uri = ("/".to_string() + &file_path).parse::<Uri>().unwrap();
                Ok(Some(new_uri))
            }
            Some(Err(err)) => Err(err),
            None => Ok(None),
        }
    }

    // When the image is created, it will be added to the cache.
    // Mostly helpful for dev server startup.
    async fn add_file_to_cache(root: &str, image: CachedImage) {
        if let CachedImageOption::Blur(_) = image.option {
            let path = image.get_file_path_from_root(root);
            let created = tokio::fs::read_to_string(path).await.ok();
            if let Some(created) = created {
                add_image_cache([(image, created)]);
            }
            return;
        }
    }
}
