// app.rs
use crate::components::splash_screen::SplashScreen;
use crate::components::theme_provider::ThemeProvider;
use crate::i18n::set_language;
use crate::models::practice::ModuleContext;
use crate::routes::Route;
use dioxus::prelude::*;

pub fn change_language(new_language: &str) {
    if let Err(e) = set_language(new_language) {
        eprintln!("Failed to set language: {}", e);
    }
}

pub fn App() -> Element {
    let mut show_splash = use_signal(|| false);
    let module_context = use_signal(|| ModuleContext {
        current_module: "whm_basic".to_string(),
    });
    use_context_provider(|| module_context);

    rsx! {
        ThemeProvider {
            if show_splash() {
                SplashScreen {
                    on_complete: move |_| show_splash.set(false)
                }
            } else {
                Router::<Route> {}
            }
        }
    }
}
