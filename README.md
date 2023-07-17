# Leptos `<Image/>` (Batteries included)

> Crafted with inspiration from Next.js

Images make a substantial impact on the size and performance of a website, so why not get them right?

Enter Leptos ` <Image/>`, a component that enhances the standard HTML `<img>` element with automatic image optimization features.

- **Size Optimization**: Resize images, and use modern `.webp` format for optimal size and quality.
- **Low-Quality Image Placeholders (LQIP)**: With this feature, the Image component embeds SVGs, extracted from the original images, into the initial SSR HTML. This placeholder is shown while the optimized version is loading.
- **Faster Page Load**: Prioritize key images, such as those contributing to the Largest Contentful Paint (LCP) with the `priority` prop. The component adds a preload `<link>` to the document head, improving load times and enhancing your site's performance.

## Quick Start

**REQUIRES SSR + AXUM**

In the base of your App, wrap everything (including Router) in `<ImageProvider/>`

```rust
// In your main App.
use leptos::*;
use leptos_meta::*;
use leptos_image::{Image, ImageProvider};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Title text="Welcome to Leptos"/>
        // Wrap the base of your App with ImageProvider
        <ImageProvider>
            <MyPage/>
            // The rest of your App...
        </ImageProvider>
    }
}

// Now you can use the Image Component anywhere in your app!

#[component]
pub fn MyPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="This Rust thing is pretty great (100 reasons to hate python)"/>
        <Image
            src="/cute_ferris.png"
            blur=true
            width=750
            height=500
            quality=85
        />
    }
}

```

Next go to your SSR Main Function in `main.rs`

Before you create your router, call the `cache_app_images` function with the project root. This will cache all the images in your app, and generate the LQIPs

```rust

use leptos::*;

let conf = get_configuration(None).await.unwrap();
let leptos_options = conf.leptos_options;
let root = leptos_options.site_root.clone();

use leptos_image::cache::cache_app_images;

cache_app_images(root, |cx: Scope| view! {cx, <App/>})
    .await
    .expect("Failed to cache images");

```

Next add an endpoint to your router that serves the cached images. For now, the endpoint path must be `/cache/image` and is not configurable

````rust

```rust
use axum::{routing::{get, post}, Router};

let router = ...

router.route("/cache/image", get(image_cache_handler));

````

The final router should look something like this!

```rust

let router = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| {
            view! { cx,
                   <App/>
            }
        })
        // Here's the new route!.
        .route("/cache/image", get(image_cache_handler))
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

```

And that's it. You're all set to use the Image Component.

## Caveats:

- Images will only be retrieved from routes that are non-dynamic (meaning not `api/post/:id` in Route path).
- Images can take a few seconds to optimize, meaning first startup of server will be slower.
- Actix Support is coming soon!
