use dioxus::prelude::*;
use crate::components::common::{Button, Input, ButtonVariant, ButtonSize, InputSize, InputVariant};
use crate::types::LoginRequest;
use crate::api::auth::AuthApi;
use crate::services::storage::StorageService;
use crate::config::AppConfig;
use crate::constants::AppConstants;
use crate::app::Route;
use crate::context::use_theme;

#[component]
pub fn Login() -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    let navigator = use_navigator();
    
    let mut email = use_signal(|| String::new());
    let mut phone = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_password = use_signal(|| false);

    let handle_login = move |evt: Event<FormData>| {
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
    };

    rsx! {
        div {
            class: "min-h-screen flex flex-col",
            style: "background-color: {colors.background};",
            
            // Top Section with Back Button
            div {
                class: "px-6 pt-12 pb-6",
                button {
                    class: "w-12 h-12 rounded-full flex items-center justify-center transition-all duration-300 hover:scale-110 active:scale-95",
                    style: "background-color: {colors.surface}; border: 1px solid {colors.border};",
                    onclick: move |_| {
                        let _ = navigator.go_back();
                    },
                    svg {
                        class: "w-6 h-6",
                        style: "color: {colors.text_primary};",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M19 12H5M12 19l-7-7 7-7" }
                    }
                }
            }
            
            // Main Content
            div {
                class: "flex-1 px-6 pb-6 flex flex-col",
                
                // Header
                div {
                    class: "mb-10",
                    h1 {
                        class: "text-4xl font-bold mb-3",
                        style: "color: {colors.text_primary};",
                        "Welcome"
                    }
                    p {
                        class: "text-base",
                        style: "color: {colors.text_secondary};",
                        "Please login or sign up to continue our app"
                    }
                }
                
                // Login Form
                form {
                    class: "flex-1 flex flex-col",
                    onsubmit: handle_login,
                    
                    div {
                        class: "space-y-5 mb-6",
                        
                        // Email/Phone Input
                        Input {
                            label: "Email or Username".to_string(),
                            placeholder: "Enter your email or phone".to_string(),
                            value: email(),
                            size: InputSize::Large,
                            variant: InputVariant::Filled,
                            oninput: move |evt: FormEvent| {
                                let val = evt.value();
                                if val.contains('@') {
                                    email.set(val);
                                    phone.set(String::new());
                                } else {
                                    phone.set(val);
                                    email.set(String::new());
                                }
                            },
                            icon_left: rsx! {
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                                }
                            },
                        }
                        
                        // Password Input
                        div {
                            Input {
                                label: "Password".to_string(),
                                r#type: if show_password() { "text" } else { "password" },
                                placeholder: "Enter your password".to_string(),
                                value: password(),
                                size: InputSize::Large,
                                variant: InputVariant::Filled,
                                oninput: move |evt: FormEvent| password.set(evt.value()),
                                icon_left: rsx! {
                                    svg {
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path { d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" }
                                    }
                                },
                                icon_right: rsx! {
                                    button {
                                        r#type: "button",
                                        class: "pointer-events-auto",
                                        onclick: move |evt| {
                                            evt.stop_propagation();
                                            show_password.set(!show_password());
                                        },
                                        svg {
                                            class: "w-5 h-5",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2",
                                            if show_password() {
                                                path { d: "M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" }
                                            } else {
                                                path { d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                                                path { d: "M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" }
                                            }
                                        }
                                    }
                                },
                            }
                            
                            // Forgot Password Link
                            div {
                                class: "text-right mt-2",
                                a {
                                    class: "text-sm font-semibold transition-colors duration-300",
                                    style: "color: {colors.error};",
                                    href: "#",
                                    "Forgot Password?"
                                }
                            }
                        }
                    }
                    
                    // Error Message
                    if !error().is_empty() {
                        div {
                            class: "mb-6 p-4 rounded-xl flex items-center gap-3",
                            style: "background-color: #FEE; border: 1px solid {colors.error};",
                            svg {
                                class: "w-5 h-5 flex-shrink-0",
                                style: "color: {colors.error};",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                    clip_rule: "evenodd",
                                }
                            }
                            p {
                                class: "text-sm font-medium",
                                style: "color: {colors.error};",
                                {error()}
                            }
                        }
                    }
                    
                    // Remember Me (Optional)
                    div {
                        class: "flex items-center mb-8",
                        input {
                            r#type: "checkbox",
                            id: "remember",
                            class: "w-5 h-5 rounded border-2 transition-colors",
                            style: "border-color: {colors.border}; accent-color: {colors.primary};",
                        }
                        label {
                            r#for: "remember",
                            class: "ml-3 text-sm font-medium",
                            style: "color: {colors.text_primary};",
                            "Remember me"
                        }
                    }
                    
                    // Login Button
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Large,
                        full_width: true,
                        disabled: loading(),
                        loading: loading(),
                        r#type: "submit",
                        "Login"
                    }
                    
                    // Divider
                    div {
                        class: "flex items-center my-8",
                        div {
                            class: "flex-1 h-px",
                            style: "background-color: {colors.divider};",
                        }
                        span {
                            class: "px-4 text-sm",
                            style: "color: {colors.text_tertiary};",
                            "OR"
                        }
                        div {
                            class: "flex-1 h-px",
                            style: "background-color: {colors.divider};",
                        }
                    }
                    
                    // Social Login Buttons
                    div {
                        class: "flex gap-4 mb-8",
                        
                        // Google Login
                        button {
                            r#type: "button",
                            class: "flex-1 h-14 rounded-xl flex items-center justify-center gap-3 transition-all duration-300 hover:scale-105 active:scale-95",
                            style: "background-color: {colors.surface}; border: 1px solid {colors.border};",
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                path {
                                    fill: "#4285F4",
                                    d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z",
                                }
                                path {
                                    fill: "#34A853",
                                    d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z",
                                }
                                path {
                                    fill: "#FBBC05",
                                    d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z",
                                }
                                path {
                                    fill: "#EA4335",
                                    d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z",
                                }
                            }
                        }
                        
                        // Facebook Login
                        button {
                            r#type: "button",
                            class: "flex-1 h-14 rounded-xl flex items-center justify-center gap-3 transition-all duration-300 hover:scale-105 active:scale-95",
                            style: "background-color: {colors.surface}; border: 1px solid {colors.border};",
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                fill: "#1877F2",
                                path { d: "M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z" }
                            }
                        }
                    }
                    
                    // Sign Up Link
                    div {
                        class: "text-center",
                        span {
                            class: "text-base",
                            style: "color: {colors.text_secondary};",
                            "Don't have an account? "
                        }
                        a {
                            class: "text-base font-bold transition-colors duration-300",
                            style: "color: {colors.text_primary};",
                            href: "#",
                            onclick: move |evt| {
                                evt.prevent_default();
                                let _ = navigator.push(Route::Register {});
                            },
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}
