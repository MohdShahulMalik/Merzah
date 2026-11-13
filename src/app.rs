use leptos::{logging, prelude::*};
use leptos_meta::{Script, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    WildcardSegment,
    components::{ParentRoute, ProtectedRoute, Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::{
    components::{favorites::FavoritesErrorToast, favorites::provide_favorites, location::LocationProvider},
    models::location::GeoPosition,
    models::user::UserOnClient,
    pages::{
        add_mosques_of_region::AddMosquesOfRegion,
        auth::{Login, Register},
        coming_soon::{AllCourses, AllLearningPaths, EventDetails, ForgotPassword},
        course::CourseDetailPage,
        discord_callback::DiscordCallback,
        events::Events,
        google_callback::GoogleCallback,
        home::Home,
        landing::Landing,
        layout::AppLayout,
        learn::Learn,
        learning_path::LearningPathDetailPage,
        lesson::LessonPage,
        microsoft_callback::MicrosoftCallback,
        mosques::Mosques,
    },
    server_functions::auth::fetch_me,
};

#[derive(Clone, Debug, Store)]
pub struct AppState {
    user: Option<UserOnClient>,
    coords: Option<GeoPosition>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let app_state = Store::new(AppState {
        user: None,
        coords: None,
    });
    let auth_user = OnceResource::new(async { fetch_me().await });
    let auth_condition = move || {
        auth_user.map(|auth_user_response_result| match auth_user_response_result {
            Ok(auth_user_response) => auth_user_response.data.is_some(),
            Err(_) => false,
        })
    };

    provide_context(app_state);
    provide_favorites();

    Effect::new(move |_| {
        auth_user.map(
            |auth_user_response_result| match auth_user_response_result {
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
            },
        );
    });

    Effect::new(move |_| {
        if let Some(coords) = app_state.coords().get() {
            logging::log!(
                "User coordinates: latitude {}, longitude {}",
                coords.latitude,
                coords.longitude
            );
        }
    });

    view! {
        <Script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/merzah.css"/>
        <Stylesheet id="amiri" href="https://fonts.googleapis.com/css2?family=Amiri:wght@400;700&display=swap"/>

        // sets the document title
        <Title text="Merzah"/>

        <Router>
            <main>
                <LocationProvider />

                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=Landing/>

                    <ParentRoute path=path!("/") view=AppLayout>
                        <ProtectedRoute path=path!("home") view=Home condition=auth_condition redirect_path=|| "/login"/>
                        <Route path=path!("mosques") view=Mosques/>
                        <ProtectedRoute path=path!("events") view=Events condition=auth_condition redirect_path=|| "/login"/>
                        <Route path=path!("learn") view=Learn/>
                        <Route path=path!("learn/courses") view=AllCourses/>
                        <Route path=path!("learn/paths") view=AllLearningPaths/>
                        <ProtectedRoute path=path!("events/:event_id") view=EventDetails condition=auth_condition redirect_path=|| "/login"/>
                        <Route path=path!("learn/courses/:course_id") view=CourseDetailPage/>
                        <Route path=path!("learn/lessons/:lesson_id") view=LessonPage/>
                        <Route path=path!("learn/roadmaps/:roadmap_id") view=LearningPathDetailPage/>
                    </ParentRoute>

                    <Route path=path!("/register") view=Register/>
                    <Route path=path!("/login") view=Login/>
                    <Route path=path!("/forgot-password") view=ForgotPassword/>
                    <Route path=path!("/add-mosques") view=AddMosquesOfRegion/>
                    <Route path=path!("/auth/callback/google") view=GoogleCallback/>
                    <Route path=path!("/auth/callback/discord") view=DiscordCallback/>
                    <Route path=path!("/auth/callback/microsoft") view=MicrosoftCallback/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
                <FavoritesErrorToast />
            </main>
        </Router>
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
        <div class="flex min-w-0 flex-col items-center justify-center gap-4 py-16 text-center sm:py-24">
            <p class="font-serif text-6xl font-bold text-foreground-900">"404"</p>
            <h1 class="font-serif text-2xl font-bold text-foreground-900 text-balance">"Page not found"</h1>
            <p class="max-w-md text-base leading-7 text-foreground-700">
                "The page you are looking for does not exist or may have been moved."
            </p>
            <a
                href="/home"
                class="inline-flex min-h-[44px] items-center rounded-lg bg-indigo-500 px-5 py-2.5 text-sm font-bold text-white transition hover:bg-indigo-600"
            >
                "Back to Home"
            </a>
        </div>
    }
}
