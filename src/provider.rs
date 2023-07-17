use crate::optimizer::CachedImage;
use lazy_static::lazy_static;
use leptos::*;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

///
/// Example
/// ```
/// #[component]
/// pub fn MyApp(cx: Scope) -> impl IntoView {
/// view! { cx,
///     <ImageProvider>
///         // The rest of your app (router, stylesheet, etc.)...
///         <MyAppInner/>
///     </ImageProvider>
/// }
/// ```
#[allow(unused_braces)]
#[component]
pub fn ImageProvider(cx: Scope, children: ChildrenFn) -> impl IntoView {
    let resource = create_blocking_resource(
        cx,
        || "images",
        |_| async {
            IMAGE_CACHE
                .read()
                .map(|c| c.clone())
                .map(|c| c.into_iter().collect::<Vec<_>>())
                .unwrap_or(vec![])
        },
    );

    let children = store_value(cx, children);

    view! { cx,
        <Transition fallback= move || children.with_value(|c| c(cx))>
            {move || {
                resource
                    .read(cx)
                    .map(move |cache| {
                        let cache = cache.into_iter().collect::<HashMap<_, _>>();
                        provide_context(cx, ImageCacheContext(Rc::new(cache)));
                        { children.with_value(|children| children(cx)) }
                    })
            }}
        </Transition>
    }
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
