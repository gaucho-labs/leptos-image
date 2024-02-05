# Leptos Image

[![Crates.io](https://img.shields.io/crates/v/leptos_image.svg)](https://crates.io/crates/leptos_image)
[![docs.rs](https://docs.rs/leptos_image/badge.svg)](https://docs.rs/leptos_image)

> Crafted with inspiration from Next.js

Images make a substantial impact on the size and performance of a website, so why not get them right?

Enter Leptos `<Image/>`, a component that enhances the standard HTML `<img>` element with automatic image optimization features.

## Features

- **Size Optimization**: Automatically resizes images and converts them to the modern `.webp` format for an optimal balance of size and quality.
- **Low-Quality Image Placeholders (LQIP)**: Embeds SVG placeholders extracted from original images into server-side rendered HTML, improving perceived performance during image loading.
- **Faster Page Load**: Prioritizes critical images, impacting Largest Contentful Paint (LCP), with the `priority` prop by adding a preload `<link>` to the document head, accelerating load times.

## Version compatibility for Leptos and Leptos Image

The table below shows the compatible versions of `leptos_image` for each `leptos` version. Ensure you are using compatible versions to avoid potential issues.

| `leptos` version | `leptos_image` version |
|------------------|------------------------|
| 0.6.*            | 0.2.*                  |


## Installation

To add `leptos_image` to your project, use cargo:

```bash
cargo add leptos_image --optional
```

Enable the SSR feature in your `Cargo.toml`:

```toml
[features]
ssr = [
    "leptos_image/ssr",
    # other dependencies...
]

hydrate = [
    "leptos_image/hydrate", 
    # other dependencies...
]
```

## Quick Start

> This requires SSR + Leptos Axum integration

1. **Provide Image Context**: Initialize your Leptos application with `leptos_image::provide_image_context` to grant it read access to the image cache.

    ```rust
    use leptos::*;

    #[component]
    fn App() -> impl IntoView {
        leptos_image::provide_image_context();
        // Your app content here
    }
    ```

2. **Axum State Configuration**: Incorporate `ImageOptimizer` into your app's Axum state.

    ```rust
    // Composite App State with the optimizer and leptos options.
    #[derive(Clone, axum::extract::FromRef)]
    struct AppState {
        leptos_options: leptos::LeptosOptions,
        optimizer: leptos_image::ImageOptimizer,
    }

    ```

3. **Integrate with Router**: Ensure your `ImageOptimizer` is available during SSR of your Leptos App.
    - Add Image Cache Route: Use `image_cache_route` to serve cached images.
    - Add `ImageOptimizer` to your App state.
    - Add `ImageOptimizer` to Leptos Context: Provide the optimizer to Leptos context using `leptos_routes_with_context`.

    ```rust
    use leptos::*;
    use leptos_axum::*;
    use leptos_image::*;

    async fn main() {
        // Get Leptos options from configuration.
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let root = leptos_options.site_root.clone();

        // Create App State with ImageOptimizer.
        let state = AppState {
            leptos_options,
            optimizer: ImageOptimizer::new("/__cache/image", root, 1),
        };

        // Create your router
        let app = Router::new()
            .route("/api/*fn_name", post(handle_server_fns))
             // Add a handler for serving the cached images.
            .image_cache_route(&state)
            // Provide the optimizer to leptos context.
            .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), App)
            .fallback(fallback_handler)
            // Provide the state to the app.
            .with_state(state);
    }
    ```


A full working example is available in the [examples](./example/start-axum) directory.

Now you can use the Image Component anywhere in your app!

```rust
#[component]
pub fn MyImage() -> impl IntoView {
    view! {
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

This setup ensures your Leptos application is fully equipped to deliver optimized images, enhancing the performance and user experience of your web projects.
