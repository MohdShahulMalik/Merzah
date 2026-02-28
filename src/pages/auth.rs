use garde::Validate;
use leptos::{html, prelude::*, reactive::spawn_local};
use leptos_router::components::A;

use crate::components::text_input::TextInput;
use crate::models::{
    auth::{LoginFormData, Platform, RegistrationFormData},
    user::Identifier,
};
use crate::server_functions::auth::{
    get_discord_oauth_url, get_google_oauth_url, get_microsoft_oauth_url, login, register,
};

#[component]
pub fn Register() -> impl IntoView {
    let (error, set_error) = signal("".to_string());
    let (success, set_success) = signal("".to_string());
    let (name_error, set_name_error) = signal(String::new());
    let (password_error, set_password_error) = signal(String::new());
    let (identifier_error, set_identifier_error) = signal(String::new());

    let name_input: NodeRef<html::Input> = NodeRef::new();
    let email_or_mobile_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // Clear previous errors
        set_name_error.set(String::new());
        set_identifier_error.set(String::new());
        set_password_error.set(String::new());
        set_error.set(String::new());

        let name_value = name_input.get().expect("<input> should be mounted").value();
        let email_or_mobile_value = email_or_mobile_input
            .get()
            .expect("<input> should be mounted")
            .value();
        let password_value = password_input
            .get()
            .expect("<input> should be mounted")
            .value();

        let identifier = if email_or_mobile_value.contains('@') {
            Identifier::Email(email_or_mobile_value)
        } else {
            Identifier::Mobile(email_or_mobile_value)
        };

        let registration_form = RegistrationFormData {
            name: name_value,
            identifier,
            password: password_value,
            platform: Platform::Web,
        };

        if let Err(report) = registration_form.validate() {
            for (field, error) in report.iter() {
                let field_str = field.to_string();
                let error_msg = error.to_string();

                if field_str.starts_with("name") {
                    set_name_error.set(error_msg);
                } else if field_str.starts_with("identifier") {
                    set_identifier_error.set(error_msg);
                } else if field_str.starts_with("password") {
                    set_password_error.set(error_msg);
                }
            }
            return;
        }

        spawn_local(async move {
            match register(registration_form).await {
                Ok(_) => set_success.set("Successful".to_string()),

                Err(e) => {
                    set_error.set(format!("Registration Error: {}", e));
                }
            }
        });
    };

    view! {
        <form on:submit = on_submit>

            <h1>Create an Account</h1>
            <p>"Sign up to get started"</p>

            <div class = "form-group">
                <label for = "name">"Full Name"</label>
                <input
                    type = "text"
                    name = "name"
                    placeholder = "Armaan Ali"
                    node_ref = name_input
                    required
                />
                <Show when = move || !name_error.get().is_empty()>
                    <p>{name_error.get()}</p>
                </Show>
            </div>

            <div class = "form-group">
                <label for = "contact">"Email or Mobile"</label>
                <input
                    type = "text"
                    name = "contact"
                    placeholder = "email@example.com or +91923XXXXX90"
                    node_ref = email_or_mobile_input
                    required
                />
                <Show when = move || !identifier_error.get().is_empty()>
                    <p>{identifier_error.get()}</p>
                </Show>
                <Show when = move || identifier_error.get().is_empty()>
                    <p>"Enter a valid email or mobile number"</p>
                </Show>
            </div>

            <div class = "form-group">
                <label for = "password">"Password"</label>
                <input
                    type = "password"
                    name = "password"
                    node_ref = password_input
                    required
                />
                <Show when = move || !password_error.get().is_empty()>
                    <p>{password_error.get()}</p>
                </Show>
                <Show when = move || password_error.get().is_empty()>
                    <p>"Password must contain 8 characters"</p>
                </Show>
            </div>

            <button
                class = "border-2 cursor-pointer bg-primary"
                type = "submit">Create Account</button>

        </form>

        <Show
            when = move || !error.get().is_empty()
            fallback = view! {<p></p>}
        >
            <p>{error.get()}</p>
        </Show>

        <Show
            when = move || !success.get().is_empty()
            fallback = view! {<p></p>}
        >
            <p>{success.get()}</p>
        </Show>

    }
}

#[component]
pub fn Login() -> impl IntoView {
    let (error, set_error) = signal("".to_string());
    let (success, set_success) = signal("".to_string());
    let (identifier_error, set_identifier_error) = signal(String::new());
    let (password_error, set_password_error) = signal(String::new());

    let email_or_mobile_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();

    let start_google_login = move |_| {
        spawn_local(async move {
            match get_google_oauth_url().await {
                Ok(response) => {
                    if let Some(url) = response.data {
                        window().location().set_href(&url).ok();
                    }
                }
                Err(e) => {
                    set_error.set(format!("Failed to start Google login: {}", e));
                }
            }
        });
    };

    let start_discord_login = move |_| {
        spawn_local(async move {
            match get_discord_oauth_url().await {
                Ok(response) => {
                    if let Some(url) = response.data {
                        window().location().set_href(&url).ok();
                    }
                }
                Err(e) => {
                    set_error.set(format!("Failed to start Discord login: {}", e));
                }
            }
        });
    };

    let start_microsoft_login = move |_| {
        spawn_local(async move {
            match get_microsoft_oauth_url().await {
                Ok(response) => {
                    if let Some(url) = response.data {
                        window().location().set_href(&url).ok();
                    }
                }
                Err(e) => {
                    set_error.set(format!("Failed to start Microsoft login: {}", e));
                }
            }
        });
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // Clear previous errors
        set_identifier_error.set(String::new());
        set_password_error.set(String::new());
        set_error.set(String::new());

        let email_or_mobile_value = email_or_mobile_input
            .get()
            .expect("failed to get the the email or mobile input node unfortunately")
            .value();

        let password_value = password_input
            .get()
            .expect("Failed to get the password input node unfortunately")
            .value();

        let is_mobile = email_or_mobile_value.chars().all(|c| c.is_digit(10) || c == '+' || c.is_whitespace() || c == '-' || c == '.' || c == '(' || c == ')');

        let identifier = if is_mobile {
            Identifier::Mobile(email_or_mobile_value)
        } else {
            Identifier::Email(email_or_mobile_value)
        };
        let login_form = LoginFormData {
            identifier,
            password: password_value,
            platform: Platform::Web,
        };

        if let Err(report) = login_form.validate() {
            for (field, error) in report.iter() {
                let field_str = field.to_string();
                let error_msg = error.to_string();

                if field_str.starts_with("identifier") {
                    set_identifier_error.set("A valid email or mobile number is required".to_string());
                } else if field_str.starts_with("password") {
                    set_password_error.set(error_msg);
                }
            }
            return;
        }

        spawn_local(async move {
            match login(login_form).await {
                Ok(_) => set_success.set("Successful".to_string()),

                Err(e) => set_error.set(format!("Error: {}", e)),
            }
        });
    };

    view! {
        <main class = "flex gap-1 h-svh bg-surface-900">
            <section class = "felx-[2] content-center grid gap-16 pl-24">

                <div class = "flex gap-2">
                    <img class = "w-auto h-16 rounded-full" src = "/assets/logo.png" />

                    <div class = "w-full">
                        <img class = "w-auto h-12" src="/assets/logo-text.png" alt="Merzah <logo>" />
                        <span class = "text-foreground-600">Your Mosque, Your Community</span>
                    </div>

                </div>

                <div class = "w-[45%] grid gap-16">
                    <p class = "text-4xl font-bold text-foreground-900">
                        "Welcome back to connect with your mosque, stay informed on events, and grow in deen and dunya"
                    </p>

                    <p class = "text-foreground-600">
                        "Rooted in Islamic values: connecting Muslims with their masajid and empowering everyone through holistic, ethical learning."
                    </p>
                </div>

            </section>

            <section class = "flex-1 bg-surface-700 fixed right-[8rem] top-[50%] -translate-y-1/2 w-[30%] px-10 py-8 rounded-3xl text-foreground-900">
            //TODO: Place this button at an appropriate place and style it properly


                <form on:submit = on_submit class = "grid gap-4 mb-3">
                    <div>
                        <h1 class = "text-2xl font-bold">"Login"</h1>
                        <h2 class = "text-foreground-400">"Welcome back. please enter your details."</h2>
                    </div>

                    <TextInput
                        label = "Email or Mobile"
                        name = "contact"
                        placeholder = "email@example.com or +91923XXXXX90"
                        input_type = "text"
                        node_ref = email_or_mobile_input
                        error_signal = identifier_error
                        hint = "Enter a valid email or mobile number"
                    />

                    <div>
                        <TextInput
                            label = "Password"
                            name = "password"
                            placeholder = "Enter your password"
                            input_type = "password"
                            node_ref = password_input
                            error_signal = password_error
                            hint = "Password must contain 8 characters"
                        />
                        <div>
                            <A href = "/forgot-password" attr:class = "text-indigo-400 font-bold">"Forgot password?"</A>
                        </div>
                    </div>


                    <button
                        class = "bg-indigo-400 hover:bg-indigo-500 transition-colors duration-300 cursor-pointer font-bold text-white py-2 rounded-2xl "
                        type = "submit">Login
                    </button>


                </form>

                <div class="flex items-center gap-4 mb-6 mt-6">
                    <div class="flex-1 h-px bg-gray-300"></div>
                    <span class="text-[.8rem] text-foreground-600">Or continue with</span>
                    <div class="flex-1 h-px bg-gray-300"></div>
                </div>

                <div class="flex gap-2 mb-8">
                    <button
                        on:click = start_google_login
                        class = "flex-1 flex items-center justify-center gap-2 bg-white text-gray-700 font-semibold py-2 px-2 rounded-2xl border border-gray-300 hover:bg-gray-50 transition-colors"
                    >
                        <img src="https://www.google.com/favicon.ico" alt="Google" class="w-5 h-5" />
                    </button>

                    <button
                        on:click = start_discord_login
                        class = "flex-1 flex items-center justify-center gap-2 bg-[#5865F2] text-white font-semibold py-2 px-2 rounded-2xl border border-[#5865F2] hover:bg-[#4752C4] transition-colors"
                    >
                        <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
                        </svg>
                    </button>

                    <button
                        on:click = start_microsoft_login
                        class = "flex-1 flex items-center justify-center gap-2 bg-[#00A4EF] text-white font-semibold py-2 px-2 rounded-2xl border border-[#00A4EF] hover:bg-[#0088CC] transition-colors"
                    >
                        <svg class="w-5 h-5 text-white" viewBox="0 0 23 23">
                            <path fill="white" d="M1 1h10v10H1z"/>
                            <path fill="white" d="M1 12h10v10H1z"/>
                            <path fill="white" d="M12 1h10v10H12z"/>
                            <path fill="white" d="M12 12h10v10H12z"/>
                        </svg>
                    </button>
                </div>

                <Show
                    when = move || !error.get().is_empty()
                    fallback = view! {<p></p>}
                >
                    <p>{error.get()}</p>
                </Show>

                <Show
                    when = move || !success.get().is_empty()
                    fallback = view! {<p></p>}
                >
                    <p>{success.get()}</p>
                </Show>

                <p class = "text-[0.90rem] text-foreground-600 text-center mb-2">"Don't have an account?"</p>
                <A href = "/register">
                    <button
                    class = "bg-transparent w-[100%] border-indigo-400 border-2 hover:bg-indigo-300/30 transition-colors duration-300 cursor-pointer font-bold text-indigo-400 py-2 rounded-2xl"
                    type = "submit">Register
                    </button>
                </A>

            </section>

        </main>
    }
}
