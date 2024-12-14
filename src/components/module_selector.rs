// src/components/module_selector.rs
use crate::data::practice_loader::{get_practice_by_id, load_practices};
use crate::i18n::translate;
use crate::models::practice::ModuleContext;
use crate::theme::set_theme;
use dioxus::prelude::*;
use tracing::*;

const MODULE_SELECTOR_CSS: Asset = asset!("/assets/styles/module_selector.css");

#[component]
pub fn ModuleSelector() -> Element {
    let mut module_context = use_context::<Signal<ModuleContext>>();
    let mut show_selector = use_signal(|| false);
    let practices = use_memo(load_practices);

    let toggle_selector = move |_| {
        show_selector.set(!show_selector());
    };

    let current_module_name = use_memo(move || {
        let name = get_practice_by_id(&module_context.read().current_module)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| module_context.read().current_module.clone());
        debug!("Current module name: {}", name);
        name
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MODULE_SELECTOR_CSS }

        div {
            class: "module-selector",
            div {
                class: if show_selector() { "module-title open" } else { "module-title" },
                onclick: toggle_selector,
                "{translate(&current_module_name())}"
            }
            { if show_selector() {
                rsx! {
                    div {
                        class: "module-options open",
                        { practices.iter().map(|practice| {
                            let practice_id = practice.id.clone();
                            rsx! {
                                button {
                                    key: "{practice.id}",
                                    onclick: move |_| {
                                        info!("Changing module to: {}", practice_id);
                                        module_context.write().current_module = practice_id.clone();
                                        set_theme(&practice_id);
                                        show_selector.set(false);
                                    },
                                    "{translate(&practice.name)}"
                                }
                            }
                        })}
                    }
                }
            } else {
                rsx! {}
            }}
        }
    }
}
