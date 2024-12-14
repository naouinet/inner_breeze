use dioxus::prelude::*;
macro_rules! define_icons {
    ($(($name:ident, $asset_path:literal)),* $(,)?) => {
        // Define the constant array of icon names
        const ICON_NAMES: &[&str] = &[$(stringify!($name)),*];

        // Define the asset constants
        $(const $name: Asset = asset!($asset_path);)*

        // Generate the match arms for get_icon_asset using uppercase names
        fn get_icon_asset(icon: &str) -> Asset {
            let icon_name = icon.to_uppercase().replace('-', "_");

            match icon_name.as_str() {
                $(stringify!($name) => $name,)*
                _ => DEFAULT
            }
        }
    };
}
// Define all icons in one place with full paths
define_icons!(
    (YOGA, "/assets/icons/yoga.svg"),
    (MEDITATION, "/assets/icons/meditation.svg"),
    (BREATHING, "/assets/icons/breathing.svg"),
    (CLOCK, "/assets/icons/clock.svg"),
    (EXERCISE, "/assets/icons/exercise.svg"),
    (MOON, "/assets/icons/moon.svg"),
    (SUN, "/assets/icons/sun.svg"),
    (LANGUAGE, "/assets/icons/language.svg"),
    (STATS, "/assets/icons/stats.svg"),
    (SETTINGS, "/assets/icons/settings.svg"),
    (TREKKING, "/assets/icons/trekking.svg"),
    (DEFAULT, "/assets/icons/default.svg"),
    (STOP, "/assets/icons/stop.svg"),
    (SKIP_NEXT, "/assets/icons/skip_next.svg"),
    (SKIP_PREV, "/assets/icons/skip_prev.svg"),
    (PLAY, "/assets/icons/play.svg"),
    (PAUSE, "/assets/icons/pause.svg")
);

#[derive(Clone, Props, PartialEq)]
pub struct IconProps {
    name: String,
    #[props(default)]
    alt: Option<String>,
    #[props(default)]
    class: Option<String>,
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            alt: None,
            class: None,
        }
    }
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let icon_src = get_icon_asset(&props.name);
    let alt_text = props.alt.unwrap_or_else(|| format!("{} icon", props.name));
    let class = props.class.unwrap_or_else(|| "icon".to_string());

    #[cfg(target_arch = "wasm32")]
    {
        rsx! {
            div {
                class: class,
                dangerous_inner_html: icon_src,
                aria_label: alt_text
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            img {
                class: class,
                src: icon_src,
                alt: alt_text
            }
        }
    }
}

pub fn is_valid_icon_name(name: &str) -> bool {
    let sanitized = name.to_lowercase().replace('-', "_");
    ICON_NAMES.contains(&sanitized.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_icon_names() {
        assert!(is_valid_icon_name("yoga"));
        assert!(is_valid_icon_name("YOGA"));
        assert!(is_valid_icon_name("skip-next"));
        assert!(!is_valid_icon_name("invalid"));
    }
}
