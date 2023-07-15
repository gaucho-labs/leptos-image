use crate::get_app_images::*;
use crate::image_service::*;

use leptos::html::Canvas;
use leptos::*;
use leptos_meta::Link;
use wasm_bindgen::JsCast;

/**
 * Image component for rendering optimized static images.
 */

// enum ImageOptions {
//     Fixed { width: u32, height: u32 },
//     Single(u32, u32),
//     Multi(Vec<(u32, u32)>),
// }

#[component]
pub fn Image(
    cx: Scope,
    #[prop(into)] src: String,
    width: u32,
    height: u32,
    #[prop(default = 75_u8)] quality: u8,
    #[prop(default = false)] blur: bool,
    // Will add preload link to head if true.
    #[prop(default = false)] priority: bool,
    #[prop(into, optional)] alt: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let blur_image = {
        CachedImage {
            src: src.clone(),
            option: CachedImageOption::Blur(Blur {
                width: 25,
                height: 25,
                svg_width: 100,
                svg_height: 100,
                sigma: 15,
            }),
        }
    };

    let opt_image = {
        CachedImage {
            src: src.clone(),
            option: CachedImageOption::Resize(Resize {
                quality,
                width,
                height,
            }),
        }
    };

    // Load images into context for blur generation.
    #[cfg(feature = "ssr")]
    if let Some(context) = use_context::<ImageContext>(cx) {
        let mut images = context.0.borrow_mut();
        images.push(opt_image.clone());
        images.push(blur_image.clone());
    }

    let blur_image = blur_image.get_url_encoded();
    let opt_image = opt_image.get_url_encoded();

    if blur {
        view! { cx, <CacheImage blur_image opt_image alt class priority/> }.into_view(cx)
    } else {
        view! { cx, <img src=opt_image alt=alt class=class/> }.into_view(cx)
    }
}

#[component]
fn CacheImage(
    cx: Scope,
    #[prop(into)] blur_image: String,
    #[prop(into)] opt_image: String,
    #[prop(into, optional)] alt: String,
    #[prop(into, optional)] class: String,
    priority: bool,
) -> impl IntoView {
    #[allow(unused_variables)]
    let (image, set_image) = create_signal(cx, blur_image);

    // let canvas_ref = create_node_ref::<Canvas>(cx);

    // use web_sys::HtmlCanvasElement;
    // use web_sys::ImageData;
    // {
    //     let canvas = canvas_ref.get().unwrap();
    //     canvas.set_width(100);
    //     canvas.set_height(100);
    //     let context = canvas
    //         .get_context("2d")
    //         .ok()
    //         .flatten()
    //         .expect("canvas to have context")
    //         .unchecked_into::<web_sys::CanvasRenderingContext2d>();
    //     // let image_data = ImageData::new_with_u8_clamped_array();

    //     let new_data = canvas.to_data_url();
    // };

    #[cfg(feature = "hydrate")]
    create_effect(cx, {
        let opt_image = opt_image.clone();
        move |_| {
            use wasm_bindgen::prelude::Closure;
            use wasm_bindgen::JsCast;
            use web_sys::HtmlImageElement;

            let image_element = HtmlImageElement::new().expect("Failed to create image element");
            image_element.set_src(&opt_image);

            if image_element.complete() {
                set_image.set(opt_image.clone());
            } else {
                let update_image = Closure::<dyn FnMut()>::new({
                    let opt_image = opt_image.clone();
                    move || {
                        set_image.set(opt_image.clone());
                    }
                });
                let as_js_func = update_image.as_ref().unchecked_ref();

                image_element
                    .add_event_listener_with_callback("load", as_js_func)
                    .unwrap_or_else(|e| {
                        error!("Failed to set image load listener {:?}", e);
                    });
                update_image.forget();
            }
        }
    });

    // No script fallback being on the bottom should take precedence over the blur image.
    view! { cx,
        {if priority {
            view! { cx, <Link rel="preload" as_="image" href=opt_image.clone()/> }
                .into_view(cx)
        } else {
            view! { cx,  }
                .into_view(cx)
        }}
        <img src=move || image.get() alt=alt.clone() class=class.clone()/>
    }
}
