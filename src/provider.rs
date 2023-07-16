use crate::image_service::CachedImage;
use lazy_static::lazy_static;
use leptos::*;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

// CacheImage -> Blur Image SVG data (literal, not file_path).
#[derive(Clone, Debug)]
pub(crate) struct ImageCacheContext(pub(crate) Rc<HashMap<CachedImage, String>>);

pub(crate) fn add_image_cache(images: Vec<(CachedImage, String)>) {
    let mut cache = IMAGE_CACHE.write().unwrap();
    for (image, path) in images {
        cache.insert(image, path);
    }
}

lazy_static! {
    pub(crate) static ref IMAGE_CACHE: Arc<RwLock<HashMap<CachedImage, String>>> = {
        let options = leptos_config::get_config_from_env().unwrap();
        let root = options.leptos_options.site_root.clone();

        log!("Initializing image cache with root: {}", &root);

        let path = format!("{root}/cache/image/**/*");
        let files = glob::glob(&path)
            .expect("Failed to read image files")
            .filter_map(|file| file.ok())
            .collect::<Vec<_>>();

        log!("Found image files: {:?}", files);
        let images = files
            .into_iter()
            .filter_map(|file| {
                let path = file.to_str().unwrap().to_string();
                CachedImage::from_file_path(&path).map(|img| (img, path))
            })
            .filter_map(|(img, path)| std::fs::read_to_string(path).ok().map(|svg| (img, svg)))
            .collect::<HashMap<_, _>>();
        Arc::new(RwLock::new(images))
    };
}

#[allow(unused_braces)]
#[component]
pub fn ImageProvider(cx: Scope, children: Children) -> impl IntoView {
    let context = IMAGE_CACHE.read();

    if let Ok(context) = context {
        provide_context(cx, ImageCacheContext(Rc::new(context.clone())));
    } else {
        error!("Failed to read image cache");
    }

    view! {cx,
        {children(cx)}
    }
}
