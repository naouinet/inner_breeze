use crate::components::breathing_circle::BreathingCircle;
use crate::components::icon::Icon;
use crate::data::practice_loader::get_practice_by_id;
use crate::i18n::translate;
use crate::models::practice::{CustomizationOptionValue, PracticeState};
use crate::routes::Route;
use dioxus::prelude::*;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

const PRACTICE_CSS: Asset = asset!("/assets/styles/practice.css");

const DEFAULT_COUNTDOWN: i32 = 5;

#[component]
pub fn PracticePage(id: String) -> Element {
    let practice = use_signal(|| get_practice_by_id(&id));
    let sequences = use_memo(move || practice().unwrap().practice_structure.sequences.clone());

    // State management
    let practice_state = use_signal(|| PracticeState::CountingDown);
    let mut current_round = use_signal(|| 1);
    let mut current_sequence = use_signal(|| 0usize);
    let mut current_step = use_signal(|| 0usize);
    let mut breath_count = use_signal(|| 0);
    let mut countdown = use_signal(|| DEFAULT_COUNTDOWN);
    let mut circle_expanded = use_signal(|| false);

    let handle_next = move || {
        let seq_vec = sequences();
        if let Some(seq) = seq_vec.get(current_sequence()) {
            if current_step() < seq.steps.len() - 1 {
                current_step.set(current_step() + 1);
            } else if current_sequence() < seq_vec.len() - 1 {
                current_sequence.set(current_sequence() + 1);
                current_step.set(0);
            } else if current_round() < practice().unwrap().practice_structure.rounds.default as i32
            {
                current_round.set(current_round() + 1);
                current_sequence.set(0);
                current_step.set(0);
            }
        }
    };

    let animation_duration = use_memo(move || {
        let default_pace = practice()
            .map(|p| p.clone()) // Clone the entire practice
            .and_then(|p| {
                p.customization_options
                    .iter()
                    .find(|opt| opt.id.as_deref() == Some("breathing_pace"))
                    .and_then(|opt| match &opt.value {
                        CustomizationOptionValue::Numeric { default, .. } => Some(*default),
                        _ => None,
                    })
            })
            .unwrap_or(1.5);
        (default_pace * 1000.0) as u64
    });

    let mut handle_next = move || {
        let seq_vec = sequences();
        if let Some(seq) = seq_vec.get(current_sequence()) {
            if current_step() < seq.steps.len() - 1 {
                current_step.set(current_step() + 1);
            } else if current_sequence() < seq_vec.len() - 1 {
                current_sequence.set(current_sequence() + 1);
                current_step.set(0);
            } else if current_round() < practice().unwrap().practice_structure.rounds.default as i32
            {
                current_round.set(current_round() + 1);
                current_sequence.set(0);
                current_step.set(0);
            }
        }
    };

    let on_next_click = move |_: Event<MouseData>| {
        handle_next();
    };

    let on_stop_click = move |_: Event<MouseData>| {
        let nav = navigator();
        nav.push(Route::HomePage {});
    };
    use_future(move || {
        to_owned![
            countdown,
            practice_state,
            circle_expanded,
            breath_count,
            handle_next,
            animation_duration
        ];

        async move {
            // Handle countdown
            while countdown() > 0 {
                #[cfg(target_arch = "wasm32")]
                TimeoutFuture::new(1000).await;
                #[cfg(not(target_arch = "wasm32"))]
                tokio::time::sleep(Duration::from_secs(1)).await;

                countdown.set(countdown() - 1);
            }
            practice_state.set(PracticeState::Active);

            // Handle breathing animation
            loop {
                if *practice_state.read() == PracticeState::Active {
                    circle_expanded.set(true);

                    #[cfg(target_arch = "wasm32")]
                    TimeoutFuture::new(animation_duration()).await;
                    #[cfg(not(target_arch = "wasm32"))]
                    tokio::time::sleep(Duration::from_millis(animation_duration())).await;

                    circle_expanded.set(false);

                    let current_count = breath_count();
                    breath_count.set(current_count + 1);

                    if current_count + 1 == 30 {
                        handle_next();
                        breath_count.set(0);
                    }

                    #[cfg(target_arch = "wasm32")]
                    TimeoutFuture::new(animation_duration()).await;
                    #[cfg(not(target_arch = "wasm32"))]
                    tokio::time::sleep(Duration::from_millis(animation_duration())).await;
                } else {
                    #[cfg(target_arch = "wasm32")]
                    TimeoutFuture::new(100).await;
                    #[cfg(not(target_arch = "wasm32"))]
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    });

    rsx! {
           document::Link { rel: "stylesheet", href: PRACTICE_CSS }
           div {
               class: "practice-page",
               {practice.with(|practice| {
                   if let Some(practice) = practice {
                       rsx! {
                           div {
                               class: "practice-exercise",
                               h2 {
                                   class: "title",
                                   {match *practice_state.read() {
                                       PracticeState::CountingDown => translate("practice.get_ready"),
                                       PracticeState::Paused => translate("practice.paused"),
                                       _ => format!("{}: {}", translate("practice.round"), current_round())
                                   }}
                               }

                               {practice.practice_structure.sequences.iter().any(|seq| seq.id == "breathing_cycle").then(|| rsx!(
                                   BreathingCircle {
                                       circle_expanded: circle_expanded,
                                       countdown: countdown,
                                       breath_count: breath_count,
                                       practice_state: practice_state,
                                       animation_duration: *animation_duration.read()
                                   }
                               ))}
                            div { class: "controls",
                                button {
                                    class: "control-button",
                                    onclick: on_stop_click,
                                    Icon {
                                        name: "stop".to_string(),
                                        class: Some("nav-icon-img".to_string())
                                    }
                                }
                                button {
                                    class: "control-button disabled",
                                    disabled: true,
                                    Icon {
                                        name: "skip-prev".to_string(),
                                        class: Some("nav-icon-img".to_string())
                                    }
                                }
                                button {
                                    class: "control-button",
                                    onclick: |_| (),
                                    Icon {
                                        name: "pause".to_string(),
                                        class: Some("nav-icon-img".to_string())
                                    }
                                }
                                button {
                                    class: "control-button",
                                    onclick: on_next_click,
                                    Icon {
                                        name: "skip-next".to_string(),
                                        class: Some("nav-icon-img".to_string())
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {
                        h1 { "{translate(\"error.practice_not_found\")}" }
                        p { "{translate(\"error.practice_not_found_description\")}" }
                    }
                }
            })}
        }
    }
}
