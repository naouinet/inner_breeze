// src/pages/home_page.rs
use crate::components::bottom_nav::BottomNav;
use crate::components::module_selector::ModuleSelector;
use crate::models::practice::ModuleContext;
use crate::routes::Route;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styles/home.css");
const LOGO_SVG: Asset = asset!("/assets/images/logo.svg");

#[component]
pub fn HomePage() -> Element {
    let module_context = use_context::<Signal<ModuleContext>>();

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div {
            class: "container",
            div {
                class: "content",
                img {
                    class: "logo",
                    src: LOGO_SVG
                }
                ModuleSelector {}
                Link {
                    class: "btn btn-large",
                    to: Route::PracticePage { id: module_context.read().current_module.clone() },
                    "Start"
                }
            }
            BottomNav { active_route: Route::HomePage {} }
        }
    }
}
