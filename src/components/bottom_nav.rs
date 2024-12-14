use crate::components::icon::Icon;
use crate::routes::Route;
use dioxus::prelude::*;

const NAV_CSS: Asset = asset!("/assets/styles/nav.css");

#[component]
pub fn BottomNav(active_route: Route) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAV_CSS }

        nav { class: "bottom-nav",
            Link {
                to: Route::ProgressPage {},
                class: if matches!(active_route, Route::ProgressPage {}) { "nav-icon active" } else { "nav-icon" },
                Icon {
                    name: "stats".to_string(),
                    alt: Some("Progress".to_string()),
                    class: Some("nav-icon-img".to_string())
                }
                span { "Progress" }
            }
            Link {
                to: Route::HomePage {},
                class: if matches!(active_route, Route::HomePage {}) { "nav-icon active" } else { "nav-icon" },
                Icon {
                    name: "sun".to_string(),
                    alt: Some("Home".to_string()),
                    class: Some("nav-icon-img".to_string())
                }
                span { "Home" }
            }
            Link {
                to: Route::SettingsPage {},
                class: if matches!(active_route, Route::SettingsPage {}) { "nav-icon active" } else { "nav-icon" },
                Icon {
                    name: "settings".to_string(),
                    alt: Some("Settings".to_string()),
                    class: Some("nav-icon-img".to_string())
                }
                span { "Settings" }
            }
        }
    }
}
