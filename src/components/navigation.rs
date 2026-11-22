use app_core::{Action, Screen};
use leptos::prelude::*;

#[component]
pub fn Navigation(screen: Memo<Screen>, on_action: Callback<Action>) -> impl IntoView {
    let items = Screen::ALL
        .into_iter()
        .map(|item| {
            let shortcut = item.shortcut().to_string();
            let is_current = move || screen.get() == item;
            view! {
                <li>
                    <button
                        type="button"
                        aria-current=move || is_current().then_some("page")
                        aria-keyshortcuts=item.shortcut().to_string()
                        on:click=move |_| on_action.run(Action::Navigate(item))
                    >
                        {item.label()}
                        <kbd aria-hidden="true">{shortcut}</kbd>
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <nav class="navigation" aria-label="Screens">
            <ul>{items}</ul>
        </nav>
    }
}
