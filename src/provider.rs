use crate::image_service::CachedImage;
use lazy_static::lazy_static;
use leptos::*;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub(crate) struct ImageCacheContext(pub(crate) Rc<HashMap<CachedImage, String>>);

pub(crate) fn set_image_cache(image: CachedImage, path: String) {
    IMAGE_CACHE.write().unwrap().insert(image, path);
}

lazy_static! {
    pub(crate) static ref IMAGE_CACHE: Arc<RwLock<HashMap<CachedImage, String>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

#[allow(unused_braces)]
#[component]
pub fn ImageProvider(cx: Scope, children: Children) -> impl IntoView {
    let context = IMAGE_CACHE.read().unwrap().clone();
    provide_context(cx, ImageCacheContext(Rc::new(context)));
    view! {cx,
        {children(cx)}
    }
}

// pub fn provide_images<IV>(
//     root: String,
//     app_fn: impl Fn(leptos::Scope) -> IV + 'static,
// ) -> ImageCacheContext
// where
//     IV: leptos::IntoView + 'static,
// {
//     let app_fn = Rc::new(app_fn);

//     let make_app = {
//         let app_fn = app_fn.clone();
//         move || {
//             let app_fn = app_fn.clone();
//             move |cx: Scope| app_fn(cx)
//         }
//     };

//     let images = get_app_images::get_app_images(make_app);

//     let image_data: HashMap<CachedImage, String> = images
//         .into_iter()
//         .filter_map(|img| match img.option {
//             crate::image_service::CachedImageOption::Blur(_) => {
//                 let path = img.get_file_path_from_root(&root);
//                 Some((img, path))
//             }
//             _ => None,
//         })
//         // Read all svg files into memory.
//         .filter_map(|(img, path)| std::fs::read_to_string(path).ok().map(|svg| (img, svg)))
//         .collect();

//     ImageCacheContext(Rc::new(image_data))
// }
