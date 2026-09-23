use leptos::prelude::*;
use leptos_router::components::Router;
use leptos_router::hooks::use_location;

mod actual_background;
mod main_content;
mod page_b;
mod page_slot;
mod sliding_background;

use actual_background::ActualBackground;
use main_content::MainContent;
use page_slot::PageSlot;
use sliding_background::SlidingBackground;

fn main() {
    console_error_panic_hook::set_once();
    // trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let opened = RwSignal::new(false);
    let transition_done = RwSignal::new(false);

    view! {
        <Router>
            <NavigationWatcher opened=opened />
            <ActualBackground />
            <PageSlot />
            <SlidingBackground opened=opened />
            <MainContent opened=opened transition_done=transition_done />
        </Router>
    }
}

#[component]
fn NavigationWatcher(opened: RwSignal<bool>) -> impl IntoView {
    let location = use_location();
    Effect::new(move |_| {
        let path = location.pathname.get();
        opened.set(path == "/page-b");
    });
    ()
}