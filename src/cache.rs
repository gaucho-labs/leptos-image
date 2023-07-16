use std::{cell::RefCell, rc::Rc};

use crate::image_service::CachedImage;

/// Context to contain all possible images.
#[derive(Clone, Default, Debug)]
pub(crate) struct ImageContext(pub(crate) Rc<RefCell<Vec<CachedImage>>>);

#[cfg(feature = "ssr")]
pub async fn cache_app_images<IV>(
    root: String,
    app_fn: impl Fn(leptos::Scope) -> IV + 'static,
) -> Result<(), crate::image_service::CreateImageError>
where
    IV: leptos::IntoView + 'static,
{
    let images = find_app_images(app_fn);
    let all_images: Vec<_> = images
        .clone()
        .into_iter()
        .map(|img| {
            let root = root.clone();
            tokio::spawn(async move { img.create_image(&root).await })
        })
        .collect();

    println!("Waiting for images to be created...");

    let result: Result<Vec<_>, crate::image_service::CreateImageError> =
        futures::future::join_all(all_images)
            .await
            .into_iter()
            .map(|res| res.unwrap())
            .collect();

    let _ = result?;

    println!("Images created.");

    let image_data = images
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
        .collect::<Vec<_>>();

    crate::provider::add_image_cache(image_data);
    Ok(())
}

#[cfg(feature = "ssr")]
pub fn find_app_images<IV>(app_fn: impl Fn(leptos::Scope) -> IV + 'static) -> Vec<CachedImage>
where
    IV: leptos::IntoView + 'static,
{
    let app_fn = Rc::new(app_fn);

    let app = {
        let app_fn = app_fn.clone();
        move |cx: leptos::Scope| app_fn(cx)
    };

    let routes = leptos_router::generate_route_list_inner(app);
    let paths: Vec<String> = routes
        .into_iter()
        .map(|route| route.path().to_string())
        .collect();

    let app = {
        let app_fn = app_fn.clone();
        move |cx: leptos::Scope| app_fn(cx)
    };

    find_app_images_from_paths(paths, app)
}

#[cfg(feature = "ssr")]
pub fn find_app_images_from_paths<IV>(
    paths: Vec<String>,
    app_fn: impl Fn(leptos::Scope) -> IV + 'static,
) -> Vec<CachedImage>
where
    IV: leptos::IntoView + 'static,
{
    use leptos::*;
    let runtime = leptos::create_runtime();
    let app_fn = Rc::new(app_fn);

    let images = paths
        .into_iter()
        .map(|path| format!("http://leptos.dev{}", path))
        .map(|path| {
            run_scope(runtime, {
                let app_fn = app_fn.clone();
                move |cx| {
                    let integration = leptos_router::ServerIntegration { path };

                    provide_context(
                        cx,
                        leptos_router::RouterIntegrationContext::new(integration),
                    );

                    let context = ImageContext::default();
                    provide_context(cx, context.clone());

                    leptos::suppress_resource_load(true);
                    _ = app_fn(cx).into_view(cx);
                    leptos::suppress_resource_load(false);

                    let images = context.0.borrow();
                    images.clone()
                }
            })
        })
        .flatten()
        .collect();

    runtime.dispose();

    images
}
