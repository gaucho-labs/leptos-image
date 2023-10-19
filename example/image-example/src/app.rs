use leptos::*;
use leptos_image::{provide_image_context, Image};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_image_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|| {

            view! {  <NotFound /> }
                .into_view()
        }>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| {
                            view! {
                                <div
                                    style:display="flex"
                                    style:width="20rem"
                                    style:justify-content="space-between"
                                    style:margin-left="auto"
                                    style:margin-right="auto"
                                >
                                    <div>
                                        <a href="/1">"Example Medium"</a>
                                    </div>
                                    <div>
                                        <a href="/2">"Example Large"</a>
                                    </div>
                                </div>
                                <Outlet/>
                            }
                        }
                    >
                        <Route
                            path="/"
                            view=|| {
                                view! {  <h1>"Welcome to Leptos Image"</h1> }
                            }
                        />
                        <Route
                            path="/1"
                            view=|| {
                                view! {  <ImageComparison width=500 height=500/> }
                            }
                        />
                        <Route
                            path="/2"
                            view=|| {
                                view! {  <ImageComparison width=1000 height=1000/> }
                            }
                        />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ImageComparison(width: u32, height: u32) -> impl IntoView {
    view! {
        <div
            style:margin-left="auto"
            style:margin-right="auto"
            style:display="flex"
            style:justify-content="space-around"
            style:align-items="center"
            style:gap="1rem"
        >
            <div>
                <div>
                    <h1>"Optimized with blur preview"</h1>
                </div>
                <Image
                    src="/cute_ferris.png"
                    width
                    height
                    quality=85
                    blur=true
                    class="test-image"
                />
            </div>
            <div>
                <div>
                    <h1>"Normal Image"</h1>
                </div>
                <img src="/cute_ferris.png" class="test-image"/>
            </div>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
