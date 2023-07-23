use crate::optimizer::CachedImage;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

/// Provides Image Cache Context to the given scope.
/// This should go in your SSR main function's Router.
///
/// Leptos + Axum Example
///
/// ```
///let app = Router::new()
///     .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
///     .leptos_routes(&leptos_options, routes, |cx| {
///         provide_image_context(cx);
///         view! { cx,
///             <App/>
///         }
///     })
/// ```
#[cfg(feature = "ssr")]
pub fn provide_image_context(cx: leptos::Scope) {
    let cache = IMAGE_CACHE
        .read()
        .map(|c| c.clone())
        .unwrap_or(HashMap::new());

    leptos::provide_context(cx, ImageCacheContext(Rc::new(cache)));
}

// CacheImage -> Blur Image SVG data (literally the svg data, not a file_path).
#[derive(Clone, Debug)]
pub(crate) struct ImageCacheContext(pub(crate) Rc<HashMap<CachedImage, String>>);

#[cfg(feature = "ssr")]
pub(crate) fn add_image_cache<I>(images: I)
where
    I: IntoIterator<Item = (CachedImage, String)>,
{
    let mut cache = IMAGE_CACHE.write().unwrap();
    for (image, svg) in images.into_iter() {
        cache.insert(image, svg);
    }
}

lazy_static! {
    pub(crate) static ref IMAGE_CACHE: Arc<RwLock<HashMap<CachedImage, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
