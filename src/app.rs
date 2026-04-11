use leptos::{logging, prelude::*};
use leptos_meta::{Script, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    WildcardSegment,
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::{
    models::{api_responses::ApiResponse, user::UserOnClient},
    pages::{
        add_mosques_of_region::AddMosquesOfRegion,
        auth::{Login, Register},
        discord_callback::DiscordCallback,
        events::Events,
        google_callback::GoogleCallback,
        home::Home,
        layout::AppLayout,
        learn::Learn,
        microsoft_callback::MicrosoftCallback,
    },
    server_functions::auth::fetch_me,
};

#[derive(Clone, Debug, Store)]
pub struct AppState {
    user: Option<UserOnClient>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let app_state = Store::new(AppState { user: None });
    let auth_user = OnceResource::new(async { fetch_me().await });

    provide_context(app_state);

    Effect::new(move |_| {
        auth_user.map(|auth_user_response_result| match auth_user_response_result {
            Ok(auth_user_response) => {
                let auth_error = auth_user_response.error.clone();
                app_state.user().set(auth_user_response.data.clone());

                if let Some(error) = auth_error {
                    logging::log!("No authenticated user found. error: {:?}", error);
                }
            }
            Err(error) => {
                logging::log!("Error fetching user data: {:?}", error);
            }
        });
    });

    view! {
        <Script src="https://cdn.tailwindcss.com"/>
        <Script>
            "tailwind.config = { corePlugins: { preflight: false, transform: false, translate: false }, optimizeUniversalDefaults: true }"
        </Script>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/merzah.css"/>

        // sets the document title
        <Title text="Merzah"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <ParentRoute path=path!("/") view=AppLayout>
                        <Route path=path!("home") view=Home/>
                        <Route path=path!("events") view=Events/>
                        <Route path=path!("learn") view=Learn/>
                    </ParentRoute>
                    <Route path=path!("/register") view=Register/>
                    <Route path=path!("/login") view=Login/>
                    <Route path=path!("/add-mosques") view=AddMosquesOfRegion/>
                    <Route path=path!("/auth/callback/google") view=GoogleCallback/>
                    <Route path=path!("/auth/callback/discord") view=DiscordCallback/>
                    <Route path=path!("/auth/callback/microsoft") view=MicrosoftCallback/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button class = "text-white bg-blue-400" on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
