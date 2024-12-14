// components/theme_provider.rs
use crate::data::practice_loader::get_practice_by_id;
use crate::models::practice::ModuleContext;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/styles/main.css");

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let module_context = use_context::<Signal<ModuleContext>>();

    // Get theme colors from the current module
    let theme_style = use_memo(move || {
        if let Some(practice) = get_practice_by_id(&module_context.read().current_module) {
            format!(
                "background-color: {}; \
                 --primary-color: {}; \
                 --secondary-color: {}; \
                 --tertiary-color: {}; \
                 --accent-color: {}; \
                 --text-primary: {}; \
                 --text-secondary: {}; \
                 --shadow-color: {};",
                practice.visual.colors.background_color,
                practice.visual.colors.primary_color,
                practice.visual.colors.secondary_color,
                practice.visual.colors.tertiary_color,
                practice.visual.colors.accent_color,
                practice.visual.colors.text_primary,
                practice.visual.colors.text_secondary,
                practice.visual.colors.shadow_color
            )
        } else {
            // Default theme if no practice is found
            "background-color: #0A0C11; \
             --primary-color: #004d4d; \
             --secondary-color: #006666; \
             --tertiary-color: #008080; \
             --accent-color: #00cccc; \
             --text-primary: #e6f3f3; \
             --text-secondary: #001a1a; \
             --shadow-color: rgba(0, 204, 204, 0.2);"
                .to_string()
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            style: "{theme_style}",
            {children}
        }
    }
}
