use std::{cell::RefCell, rc::Rc};

use crate::image_service::CachedImage;

/// Context to contain all possible images.
#[derive(Clone, Default, Debug)]
pub(crate) struct ImageContext(pub(crate) Rc<RefCell<Vec<CachedImage>>>);

#[cfg(feature = "ssr")]
pub fn get_app_images<App, IV>(make_app_fn: impl Fn() -> App) -> Vec<CachedImage>
where
    App: Fn(leptos::Scope) -> IV + 'static,
    IV: leptos::IntoView + 'static,
{
    let routes = leptos_router::generate_route_list_inner(make_app_fn());
    let paths: Vec<String> = routes
        .into_iter()
        .map(|route| route.path().to_string())
        .collect();

    get_app_images_from_paths(paths, make_app_fn())
}

#[cfg(feature = "ssr")]
pub fn get_app_images_from_paths<IV>(
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
