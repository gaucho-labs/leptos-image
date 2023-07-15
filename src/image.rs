use crate::get_app_images::*;
use crate::image_service::*;
use crate::provider::ImageCacheContext;

use leptos::*;
use leptos_meta::Link;

/**
 * Image component for rendering optimized static images.
 */

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

    let placeholder_svg = {
        use_context::<ImageCacheContext>(cx)
            .map(|context| context.0)
            .and_then(|map| {
                let maybe = map.get(&blur_image);
                maybe.map(|link| link.clone())
            })
    };

    let blur_image = blur_image.get_url_encoded();
    let opt_image = opt_image.get_url_encoded();

    if blur {
        let svg = {
            if let Some(svg_data) = placeholder_svg {
                SvgImage::InMemory(svg_data)
            } else {
                SvgImage::Request(blur_image)
            }
        };
        view! { cx, <CacheImage svg opt_image alt class priority/> }.into_view(cx)
    } else {
        view! { cx, <img src=opt_image alt=alt class=class/> }.into_view(cx)
    }
}

enum SvgImage {
    InMemory(String),
    Request(String),
}

#[component]
fn CacheImage(
    cx: Scope,
    svg: SvgImage,
    #[prop(into)] opt_image: String,
    #[prop(into, optional)] alt: String,
    #[prop(into, optional)] class: String,
    priority: bool,
) -> impl IntoView {
    use base64::{engine::general_purpose, Engine as _};

    let style = {
        let background_image = match svg {
            SvgImage::InMemory(svg_data) => {
                let svg_encoded = general_purpose::STANDARD.encode(svg_data.as_bytes());
                format!("url('data:image/svg+xml;base64,{svg_encoded}')")
            }
            SvgImage::Request(svg_url) => {
                format!("url('{}')", svg_url)
            }
        };
        let style= format!(
        "color:transparent;max-width:100%;height:auto;background-size:cover;background-position:50% 50%;background-repeat:no-repeat;background-image:{background_image}');",
        );

        style
    };

    // let (image, set_image) = create_signal(cx, blur_image);
    let (style, set_style) = create_signal(cx, style);

    view! { cx,
        {if priority {
            view! { cx, <Link rel="preload" as_="image" href=opt_image.clone()/> }
                .into_view(cx)
        } else {
            view! { cx,  }
                .into_view(cx)
        }}
        <img
            src=opt_image
            alt=alt.clone()
            class=class.clone()
            style=move || style.get()
            on:load=move |_| set_style.set("".into())
        />
    }
}
