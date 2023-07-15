use crate::{get_app_images, image_service::CachedImage};
use leptos::*;
use std::{collections::HashMap, rc::Rc};

pub fn provide_images<IV>(
    root: String,
    app_fn: impl Fn(leptos::Scope) -> IV + 'static,
) -> ImageContext
where
    IV: leptos::IntoView + 'static,
{
    let app_fn = Rc::new(app_fn);

    let make_app = {
        let app_fn = app_fn.clone();
        move || {
            let app_fn = app_fn.clone();
            move |cx: Scope| app_fn(cx)
        }
    };

    let images = get_app_images::get_app_images(make_app);

    let image_data: HashMap<CachedImage, String> = images
        .into_iter()
        .filter_map(|img| match img.option {
            crate::image_service::CachedImageOption::Blur(_) => {
                let path = img.get_file_path_from_root(&root);
                Some((img, path))
            }
            _ => None,
        })
        // Read all svg files into memory.
        .filter_map(|(img, path)| std::fs::read_to_string(path).ok().map(|svg| (img, svg)))
        .collect();

    ImageContext(Rc::new(image_data))
}

#[derive(Clone, Debug)]
pub struct ImageContext(pub(crate) Rc<HashMap<CachedImage, String>>);
