use leptos::{
    ev::{KeyboardEvent, keydown},
    logging::log,
    prelude::*,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ctr = RwSignal::new(0);
    window_event_listener(keydown, move |ev| handle_hotkey(ev, ctr));

    view! {
        <div class="container">
            "Counter goes up when you press 'ctrl + e': " {move || ctr.get()}
        </div>
    }
}

fn handle_hotkey(ev: KeyboardEvent, ctr: RwSignal<i64>) {
    let ctrl_held_down = ev.ctrl_key();
    let key = ev.key();
    if ctrl_held_down && key == "e" {
        //these two prevent default browser hotkeys from overriding
        ev.prevent_default();
        ev.stop_propagation();
        //these two prevent default browser hotkeys from overriding

        ctr.update(|c| *c += 1);
        log!("ctrl + e pressed");
    }
    log!("{}, {}", ctrl_held_down, key);
}
