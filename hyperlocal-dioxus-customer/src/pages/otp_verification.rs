use dioxus::prelude::*;

#[component]
pub fn OtpVerification() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 flex items-center justify-center",
            div {
                class: "max-w-md w-full space-y-8",
                h2 {
                    class: "text-2xl font-bold text-center",
                    "OTP Verification"
                }
                p {
                    class: "text-gray-600 text-center",
                    "OTP verification page"
                }
            }
        }
    }
}
