#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{get, post},
        Router,
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_image::ImageOptimizer;
    use leptos_image::*;
    use start_axum::app::*;
    use start_axum::fileserv::file_and_error_handler;
    use tokio::net::TcpListener;

    // simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
    #[derive(Clone, axum::extract::FromRef)]
    struct AppState {
        leptos_options: leptos::LeptosOptions,
        optimizer: leptos_image::ImageOptimizer,
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let root = leptos_options.site_root.clone();

    let state = AppState {
        leptos_options: leptos_options.clone(),
        optimizer: ImageOptimizer::new(root, 1),
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // Add a handler for serving the cached images.
        .route("/cache/image", get(image_cache_handler))
        // Provide the optimizer to leptos context.
        .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), App)
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
