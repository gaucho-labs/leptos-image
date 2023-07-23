use crate::optimizer::*;
use crate::provider::ImageCacheContext;

use leptos::*;
use leptos_meta::Link;

/**
 */

/// Image component for rendering optimized static images.
/// Images MUST be static. Will not work with dynamic images.
#[component]
pub fn Image(
    cx: Scope,
    /// Image source. Should be path relative to root.
    #[prop(into)]
    src: String,
    /// Resize image height, but will still maintain the same aspect ratio.
    height: u32,
    /// Resize image width, but will still maintain the same aspect ratio.
    width: u32,
    /// Image quality. 0-100.
    #[prop(default = 75_u8)]
    quality: u8,
    /// Will add blur image to head if true.
    #[prop(default = false)]
    blur: bool,
    /// Will add preload link to head if true.
    #[prop(default = false)]
    priority: bool,
    /// Lazy load image.
    #[prop(default = true)]
    lazy: bool,
    /// Image alt text.
    #[prop(into, optional)]
    alt: String,
    /// Style class for image.
    #[prop(into, optional)]
    class: Option<AttributeValue>,
) -> impl IntoView {
    if src.starts_with("http") {
        debug_warn!("Image component only supports static images.");
        let loading = if lazy { "lazy" } else { "eager" };
        return view! { cx, <img src=src alt=alt class=class loading=loading/> }.into_view(cx);
    }

    let blur_image = {
        CachedImage {
            src: src.clone(),
            option: CachedImageOption::Blur(Blur {
                width: 20,
                height: 20,
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
    // Happens on server start.
    #[cfg(feature = "ssr")]
    if let Some(context) = use_context::<crate::introspect::IntrospectImageContext>(cx) {
        let mut images = context.0.borrow_mut();
        images.push(opt_image.clone());
        if blur {
            images.push(blur_image.clone());
        }
    }

    // Check to see if we have svg literal already loaded in memory
    // We can send over data on initial ssr load, instead of waiting for client to hydrate.
    let placeholder_svg = {
        use_context::<ImageCacheContext>(cx)
            .map(|context| context.0)
            .and_then(|map| {
                let maybe = map.get(&blur_image);
                maybe.map(|link| link.clone())
            })
    };

    let opt_image = opt_image.get_url_encoded();

    if blur {
        let svg = {
            if let Some(svg_data) = placeholder_svg {
                SvgImage::InMemory(svg_data)
            } else {
                let blur_image = blur_image.get_url_encoded();
                SvgImage::Request(blur_image)
            }
        };
        view! { cx, <CacheImage lazy svg opt_image alt class=class priority/> }.into_view(cx)
    } else {
        let loading = if lazy { "lazy" } else { "eager" };
        view! { cx, <img alt=alt class=class decoding="async" loading=loading src=opt_image /> }
            .into_view(cx)
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
    class: Option<AttributeValue>,
    priority: bool,
    lazy: bool,
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
        "color:transparent;background-size:cover;background-position:50% 50%;background-repeat:no-repeat;background-image:{background_image};",
        );

        style
    };

    let loading = if lazy { "lazy" } else { "eager" };

    view! { cx,
        {if priority {
            view! { cx, <Link rel="preload" as_="image" href=opt_image.clone()/> }
                .into_view(cx)
        } else {
            view! { cx,  }
                .into_view(cx)
        }}
        <img
            alt=alt.clone()
            class=class
            decoding="async"
            loading=loading
            src=opt_image
            style=style
        />
    }
}
