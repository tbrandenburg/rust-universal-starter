use app_core::{Action, AppState, update};
use leptos::{ev::KeyboardEvent, html, prelude::*, wasm_bindgen::JsCast, web_sys::Element};

use super::{navigation::Navigation, panel::Panel};

#[component]
pub fn Shell() -> impl IntoView {
    let state = RwSignal::new(AppState::default());
    let screen = Memo::new(move |_| state.with(AppState::screen));
    let count = Memo::new(move |_| state.with(AppState::count));
    let on_action = Callback::new(move |action| state.update(|state| update(state, action)));
    let shell_ref = NodeRef::<html::Main>::new();

    let on_keydown = move |event: KeyboardEvent| {
        if event.default_prevented()
            || event.alt_key()
            || event.ctrl_key()
            || event.meta_key()
            || event.is_composing()
            || event.repeat()
            || is_editing(&event)
        {
            return;
        }

        let key = event.key();
        if key.len() == 1
            && let Some(action) = key.chars().next().and_then(Action::from_shortcut)
        {
            event.prevent_default();
            if matches!(action, Action::Navigate(_))
                && let Some(shell) = shell_ref.get()
            {
                let _ = shell.focus();
            }
            on_action.run(action);
        }
    };

    view! {
        <main class="shell" tabindex="0" node_ref=shell_ref on:keydown=on_keydown>
            <header class="shell-header">
                <h1>"Rust Universal Starter"</h1>
            </header>
            <div class="shell-content">
                <Navigation screen on_action/>
                <Panel screen count on_action/>
            </div>
        </main>
    }
}

// Leave native editing and selection shortcuts available to controls added to the shell.
fn is_editing(event: &KeyboardEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|element| {
            element
                .closest(
                    "input, textarea, select, [contenteditable]:not([contenteditable='false'])",
                )
                .ok()
                .flatten()
        })
        .is_some()
}
