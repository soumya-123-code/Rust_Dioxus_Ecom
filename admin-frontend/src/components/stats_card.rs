use dioxus::prelude::*;

#[component]
pub fn StatsCard(
    title: String,
    value: String,
    icon: String,
    color: String,
) -> Element {
    let bg_color = match color.as_str() {
        "blue" => "bg-blue-500",
        "green" => "bg-green-500",
        "yellow" => "bg-yellow-500",
        "red" => "bg-red-500",
        "purple" => "bg-purple-500",
        "indigo" => "bg-indigo-500",
        _ => "bg-gray-500",
    };

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6",
            div { class: "flex items-center",
                div { class: "flex-shrink-0 {bg_color} rounded-md p-3",
                    span { class: "text-2xl text-white", "{icon}" }
                }
                div { class: "ml-5 w-0 flex-1",
                    dl {
                        dt { class: "text-sm font-medium text-gray-500 truncate", "{title}" }
                        dd { class: "text-2xl font-semibold text-gray-900", "{value}" }
                    }
                }
            }
        }
    }
}
