use app_core::{Action, Screen};
use leptos::prelude::*;

#[component]
pub fn Panel(screen: Memo<Screen>, count: Memo<u64>, on_action: Callback<Action>) -> impl IntoView {
    let content = move || match screen.get() {
        Screen::Home => view! {
            <div class="counter">
                <span id="counter-label">"Counter"</span>
                <output aria-labelledby="counter-label">{move || count.get()}</output>
            </div>
            <button
                type="button"
                class="primary-action"
                aria-keyshortcuts="I"
                on:click=move |_| on_action.run(Action::Increment)
            >
                "Increment counter" <kbd aria-hidden="true">"I"</kbd>
            </button>
        }
        .into_any(),
        Screen::Activity => view! {
            <ul class="panel-list">
                <li>
                    <span id="activity-counter-label">"Increments this session: "</span>
                    <output aria-labelledby="activity-counter-label">{move || count.get()}</output>
                </li>
            </ul>
        }
        .into_any(),
        Screen::Settings => view! {
            <ul class="panel-list">
                <li><kbd>"1"</kbd>"–"<kbd>"3"</kbd>" Navigate screens"</li>
                <li><kbd>"I"</kbd>" Increment counter"</li>
            </ul>
        }
        .into_any(),
    };

    view! {
        <section class="panel" aria-labelledby="screen-heading">
            <h2 id="screen-heading" aria-live="polite">{move || screen.get().label()}</h2>
            {content}
        </section>
    }
}
