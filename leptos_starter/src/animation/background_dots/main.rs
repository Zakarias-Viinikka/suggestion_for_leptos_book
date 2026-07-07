use leptos::prelude::*;
use wasm_bindgen::prelude::*; //for importing js

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
        </div>
    }
}

//change name of the macro.
// test it out inside of the extern thing.
// create a setinterval or settimeout whichever one is relevant that starts a thing that does an animation
// make it stop when "finish anim" is called.
// finish anim should update a signal so that leptos knows "ok js finished now" but that can be saved for the later example
macro_rules! animation_js_bindings {
    ($($variant:ident),+ $(,)?) => {
        $(
            fn [<set_anim_state_ $variant:lower>](element_id: String);
            fn [<finish_anim_state_ $variant:lower>](element_id: String);
        )+
    };
}

#[wasm_bindgen]
extern "C" {
    /*fn set_anim_state_bounce(String);
    fn set_anim_state_finish_bounce(String);*/
}

enum AnimationState {
    Bounce,
}

impl AnimationState {
    fn set_anim_state(&self) {}

    fn finish_animation(&self) {}
}
