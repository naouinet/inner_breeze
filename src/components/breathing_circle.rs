use crate::models::practice::PracticeState;
use dioxus::prelude::*;

#[component]
pub fn BreathingCircle(
    circle_expanded: Signal<bool>,
    countdown: Signal<i32>,
    breath_count: Signal<i32>,
    practice_state: Signal<PracticeState>,
    animation_duration: u64, // Add new prop
) -> Element {
    let should_show_animation = *practice_state.read() == PracticeState::Active;
    let inner_circle_class = if should_show_animation {
        if circle_expanded() {
            "breath-circle inner expanded"
        } else {
            "breath-circle inner collapsed"
        }
    } else {
        "breath-circle inner collapsed"
    };

    // Create the transition style string
    let transition_style = format!("transition-duration: {}ms", animation_duration);

    rsx! {
        div {
            class: "breath-circle-container",
            div {
                class: "breath-circle outer",
            }
            div {
                class: "{inner_circle_class}",
                style: "{transition_style}"
            }
            div {
                class: "breath-count",
                {if countdown() > 0 {
                    countdown().to_string()
                } else {
                    breath_count().to_string()
                }}
            }
        }
    }
}
