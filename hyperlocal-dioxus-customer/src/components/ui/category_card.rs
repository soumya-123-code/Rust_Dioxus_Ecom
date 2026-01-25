use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CategoryCardProps {
    pub category: crate::types::Category,
}

#[component]
pub fn CategoryCard(props: CategoryCardProps) -> Element {
    let category = &props.category;

    rsx! {
        div {
            class: "bg-white rounded-lg shadow p-4 cursor-pointer hover:shadow-md transition-shadow text-center",
            if let Some(image) = &category.image {
                img {
                    class: "w-full h-24 object-cover rounded mb-2",
                    src: image.clone(),
                    alt: category.name.as_deref().unwrap_or("Category")
                }
            }
            if let Some(name) = &category.name {
                p {
                    class: "text-sm font-medium text-gray-900",
                    { name.clone() }
                }
            }
        }
    }
}
