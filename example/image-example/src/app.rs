use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_image::{image::Image, provider::ImageProvider};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <ImageProvider>
            <Router
                fallback=|cx| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { cx, <ErrorTemplate outside_errors/> }
                        .into_view(cx)
                }
            >
                <main>
                    <Routes>
                        <Route
                            path=""
                            view=|cx| {
                                view! { cx, <HomePage/> }
                            }
                        />
                    </Routes>
                </main>
            </Router>
        </ImageProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div>
            <Image
                src="/cute_ferris.png"
                width=500
                height=500
                quality=85
                blur=true
                class="test-image"
            />
        </div>
    }
}
