use dioxus::prelude::*;

use crate::components::common::{Button, Input, ButtonVariant};
use crate::types::LoginRequest;
use crate::api::auth::AuthApi;
use crate::services::storage::StorageService;
use crate::config::AppConfig;
use crate::constants::AppConstants;
use crate::app::Route;

#[component]
pub fn LoginForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut phone = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let navigator = use_navigator();

    rsx! {
        div {
            style: "
                width: 100%;
                max-width: 28rem;
                margin: 0 auto;
                padding: 1.5rem;
            ",
            div {
                style: "
                    background: white;
                    border-radius: 0.75rem;
                    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);
                    padding: 2rem;
                ",
                div {
                    style: "
                        text-align: center;
                        margin-bottom: 2rem;
                    ",
                    h2 {
                        style: "
                            font-size: 1.875rem;
                            font-weight: 700;
                            color: #111827;
                            margin-bottom: 0.5rem;
                        ",
                        "Welcome Back"
                    }
                    p {
                        style: "
                            color: #6b7280;
                            font-size: 0.875rem;
                        ",
                        "Sign in to your account to continue"
                    }
                }

                form {
                    style: "
                        display: flex;
                        flex-direction: column;
                        gap: 1.25rem;
                    ",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        let auth_api = AuthApi::new(AppConfig::api_base_url());
                        spawn(async move {
                            loading.set(true);
                            error.set(String::new());

                            let request = LoginRequest {
                                email: if email().is_empty() { None } else { Some(email()) },
                                mobile: if phone().is_empty() { None } else { Some(phone()) },
                                password: password(),
                                fcm_token: None,
                                device_type: Some(AppConstants::DEVICE_TYPE.to_string()),
                            };

                            match auth_api.login(request).await {
                                Ok(auth_model) => {
                                    if let Some(token) = auth_model.access_token {
                                        let _ = StorageService::save_token(&token);
                                        if let Some(user_data) = auth_model.data {
                                            let _ = StorageService::save_user_data(&user_data);
                                        }
                                        let _ = navigator.push(Route::Home {});
                                    } else {
                                        error.set("Login failed: No token received".to_string());
                                    }
                                }
                                Err(e) => {
                                    error.set(e);
                                }
                            }
                            loading.set(false);
                        });
                    },
                    
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            gap: 0.375rem;
                        ",
                        label {
                            style: "
                                font-size: 0.875rem;
                                font-weight: 500;
                                color: #374151;
                                margin-bottom: 0.25rem;
                            ",
                            "Email or Phone"
                        }
                        Input {
                            r#type: "text",
                            placeholder: "Enter your email or phone number",
                            value: email(),
                            oninput: move |evt: FormEvent| {
                                let val = evt.value();
                                if val.contains('@') {
                                    email.set(val);
                                    phone.set(String::new());
                                } else {
                                    phone.set(val);
                                    email.set(String::new());
                                }
                            }
                        }
                    }

                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            gap: 0.375rem;
                        ",
                        div {
                            style: "
                                display: flex;
                                justify-content: space-between;
                                align-items: center;
                                margin-bottom: 0.25rem;
                            ",
                            label {
                                style: "
                                    font-size: 0.875rem;
                                    font-weight: 500;
                                    color: #374151;
                                ",
                                "Password"
                            }
                            a {
                                href: "#",
                                style: "
                                    font-size: 0.875rem;
                                    color: #2563eb;
                                    text-decoration: none;
                                    font-weight: 500;
                                ",
                                onmouseover: move |_| {},
                                "Forgot password?"
                            }
                        }
                        Input {
                            r#type: "password",
                            placeholder: "Enter your password",
                            value: password(),
                            oninput: move |evt: FormEvent| password.set(evt.value())
                        }
                    }

                    if !error().is_empty() {
                        div {
                            style: "
                                background-color: #fef2f2;
                                border: 1px solid #fecaca;
                                border-radius: 0.5rem;
                                padding: 0.75rem 1rem;
                                display: flex;
                                align-items: center;
                                gap: 0.5rem;
                            ",
                            span {
                                style: "
                                    color: #dc2626;
                                    font-size: 1.25rem;
                                ",
                                "⚠"
                            }
                            span {
                                style: "
                                    color: #991b1b;
                                    font-size: 0.875rem;
                                ",
                                {error()}
                            }
                        }
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: loading(),
                        loading: loading(),
                        r#type: "submit",
                        "Sign in"
                    }

                    div {
                        style: "
                            text-align: center;
                            margin-top: 1rem;
                        ",
                        span {
                            style: "
                                color: #6b7280;
                                font-size: 0.875rem;
                            ",
                            "Don't have an account? "
                        }
                        a {
                            href: "#",
                            style: "
                                color: #2563eb;
                                font-weight: 500;
                                text-decoration: none;
                                font-size: 0.875rem;
                            ",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}