use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_image::Image;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }
                .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
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
                    width=750
                    height=500
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
