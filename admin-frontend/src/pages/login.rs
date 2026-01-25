use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::api::{self, LoginRequest, LoginResponse};
use crate::state::AuthState;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let mut auth = use_context::<Signal<AuthState>>();
    let nav = use_navigator();
    
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let handle_submit = move |_evt: Event<FormData>| {
        let email_val = email.read().clone();
        let password_val = password.read().clone();
        
        spawn(async move {
            loading.set(true);
            error.set(None);
            
            let req = LoginRequest {
                email: email_val,
                password: password_val,
            };
            
            match api::post::<LoginResponse, _>("/login", &req).await {
                Ok(response) => {
                    auth.write().login(response.token, response.user);
                    nav.push(Route::Dashboard {});
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            
            loading.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-100",
            div { class: "max-w-md w-full bg-white rounded-lg shadow-lg p-8",
                div { class: "text-center mb-8",
                    h1 { class: "text-2xl font-bold text-gray-800", "HyperLocal Admin" }
                    p { class: "text-gray-600 mt-2", "Sign in to your account" }
                }
                
                if let Some(err) = error.read().as_ref() {
                    div { class: "bg-red-50 text-red-600 p-3 rounded mb-4 text-sm",
                        "{err}"
                    }
                }
                
                form { onsubmit: handle_submit,
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Email"
                        }
                        input {
                            r#type: "email",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "admin@example.com",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                            required: true,
                        }
                    }
                    
                    div { class: "mb-6",
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Password"
                        }
                        input {
                            r#type: "password",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            placeholder: "••••••••",
                            value: "{password}",
                            oninput: move |evt| password.set(evt.value()),
                            required: true,
                        }
                    }
                    
                    button {
                        r#type: "submit",
                        class: "w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50",
                        disabled: *loading.read(),
                        if *loading.read() { "Signing in..." } else { "Sign In" }
                    }
                }
            }
        }
    }
}
