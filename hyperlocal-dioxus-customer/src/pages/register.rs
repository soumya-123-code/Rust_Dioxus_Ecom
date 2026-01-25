use dioxus::prelude::*;

use crate::types::RegisterRequest;
use crate::api::auth::AuthApi;
use crate::services::storage::StorageService;
use crate::config::AppConfig;
use crate::constants::AppConstants;
use crate::app::Route;

#[component]
pub fn Register() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut mobile = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let navigator = use_navigator();

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8",
            div {
                class: "max-w-md w-full space-y-8",
                    div {
                        class: "text-center",
                        h2 {
                            class: "text-3xl font-extrabold text-gray-900",
                            "Create your account"
                        }
                    }
                    form {
                        class: "mt-8 space-y-6",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            if password() != confirm_password() {
                                error.set("Passwords do not match".to_string());
                                return;
                            }
                            let auth_api = AuthApi::new(AppConfig::api_base_url());
                            spawn(async move {
                                loading.set(true);
                                error.set(String::new());

                                let request = RegisterRequest {
                                    name: name(),
                                    email: email(),
                                    mobile: mobile(),
                                    country: AppConstants::DEFAULT_COUNTRY.to_string(),
                                    iso2: AppConstants::DEFAULT_ISO2.to_string(),
                                    password: password(),
                                    password_confirmation: confirm_password(),
                                    fcm_token: None,
                                    device_type: Some(AppConstants::DEVICE_TYPE.to_string()),
                                };

                                match auth_api.register(request).await {
                                    Ok(auth_model) => {
                                        if let Some(token) = auth_model.access_token {
                                            let _ = StorageService::save_token(&token);
                                            if let Some(user_data) = auth_model.data {
                                                let _ = StorageService::save_user_data(&user_data);
                                            }
                                            let _ = navigator.push(crate::app::Route::Home {});
                                        } else {
                                            error.set("Registration failed: No token received".to_string());
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
                            class: "space-y-4",
                            input {
                                class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                r#type: "text",
                                placeholder: "Full Name",
                                value: "{name()}",
                                oninput: move |evt| name.set(evt.value())
                            }
                            input {
                                class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                r#type: "email",
                                placeholder: "Email",
                                value: "{email()}",
                                oninput: move |evt| email.set(evt.value())
                            }
                            input {
                                class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                r#type: "tel",
                                placeholder: "Mobile Number",
                                value: "{mobile()}",
                                oninput: move |evt| mobile.set(evt.value())
                            }
                            input {
                                class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                r#type: "password",
                                placeholder: "Password",
                                value: "{password()}",
                                oninput: move |evt| password.set(evt.value())
                            }
                            input {
                                class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                r#type: "password",
                                placeholder: "Confirm Password",
                                value: "{confirm_password()}",
                                oninput: move |evt| confirm_password.set(evt.value())
                            }
                        }
                        if !error().is_empty() {
                            div {
                                class: "text-red-600 text-sm",
                                "{error()}"
                            }
                        }
                        div {
                            button {
                                class: "group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50",
                                disabled: loading(),
                                if loading() {
                                    "Creating account..."
                                } else {
                                    "Sign up"
                                }
                            }
                        }
                        div {
                            class: "text-center",
                            a {
                                class: "font-medium text-indigo-600 hover:text-indigo-500",
                                href: "#",
                                onclick: move |_| {
                                    let _ = navigator.push(Route::Login {});
                                },
                                "Already have an account? Sign in"
                            }
                        }
                    }
                }
            }
        }
    }
