use crate::optimizer::CachedImage;
use dashmap::DashMap;
use lazy_static::lazy_static;
use leptos::*;
use std::sync::Arc;

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
            IMAGE_CACHE
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect::<Vec<_>>()
        },
    );

    leptos::provide_context(resource);
}

type ImageResource = Resource<(), Vec<(CachedImage, String)>>;

pub(crate) fn use_image_cache_resource() -> ImageResource {
    use_context::<ImageResource>().expect("Missing Image Resource")
}

#[cfg(feature = "ssr")]
pub(crate) async fn add_image_cache<S, I>(root: S, images: I)
where
    S: AsRef<str>,
    I: IntoIterator<Item = CachedImage>,
{
    images
        .into_iter()
        .filter(|image| image.option == crate::optimizer::CachedImageOption::Blur)
        .filter(|image| IMAGE_CACHE.get(&image).is_none())
        .for_each(|image| {
            let path = image.get_file_path_from_root(root);
            if let Some(data) = tokio::fs::read_to_string(path).await.ok() {
                IMAGE_CACHE.insert(image, data);
            } else {
                tracing::error!("Failed to read image: {:?}", image.path);
            }
        });
}

lazy_static! {
    // CacheImage -> Blur Image SVG data (literally the svg data, not a file_path).
    pub(crate) static ref IMAGE_CACHE: Arc<DashMap<CachedImage, String>> =
        Arc::new(DashMap::new());
}
