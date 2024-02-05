use crate::optimizer::CachedImage;
use leptos::*;

/// Provides Image Cache Context to the given scope.
/// This should go in the base of your Leptos <App/>.
///
///Example
///
/// ```
/// use leptos_image::*;
///
/// #[component]
/// pub fn App(cx: Scope) -> impl IntoView {
///     provide_image_context();
///
/// }
///
/// ```
pub fn provide_image_context() {
    let resource: ImageResource = create_blocking_resource(
        || (),
        |_| async {
            get_image_cache()
                .await
                .expect("Failed to retrieve image cache")
        },
    );

    leptos::provide_context(resource);
}

type ImageResource = Resource<(), Vec<(CachedImage, String)>>;

pub(crate) fn use_image_cache_resource() -> ImageResource {
    use_context::<ImageResource>().expect("Missing Image Resource")
}

#[cfg(feature = "ssr")]
pub(crate) fn use_optimizer() -> Result<crate::ImageOptimizer, ServerFnError> {
    use_context::<crate::ImageOptimizer>()
        .ok_or_else(|| ServerFnError::ServerError("Image Optimizer Missing.".into()))
}

#[cfg(feature = "ssr")]
pub(crate) async fn add_image_cache<I>(optimizer: &crate::optimizer::ImageOptimizer, images: I)
where
    I: IntoIterator<Item = CachedImage>,
{
    let images = images
        .into_iter()
        .filter(|image| matches!(image.option, crate::optimizer::CachedImageOption::Blur(_)))
        .filter(|image| optimizer.cache.get(&image).is_none());

    for image in images {
        let path = optimizer.get_file_path_from_root(&image);
        match tokio::fs::read_to_string(path).await {
            Ok(data) => {
                optimizer.cache.insert(image, data);
                tracing::info!("Added image to cache with size {}", optimizer.cache.len())
            }
            Err(e) => {
                tracing::error!("Failed to read image: {:?} with error: {:?}", image, e);
            }
        }
    }
}

#[server(GetImageCache)]
pub(crate) async fn get_image_cache() -> Result<Vec<(CachedImage, String)>, ServerFnError> {
    let optimizer = use_optimizer()?;

    Ok(optimizer
        .cache
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect())
}
