use std::{cell::RefCell, rc::Rc};

use crate::optimizer::CachedImage;

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

/// Context to contain all possible images.
#[derive(Clone, Default, Debug)]
pub(crate) struct IntrospectImageContext(pub(crate) Rc<RefCell<Vec<CachedImage>>>);

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

                    let context = IntrospectImageContext::default();
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
