use leptos::prelude::*;
use wasm_bindgen::prelude::*; //for importing js

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)]
enum BounceState {
    NotMoving,
    Moving,
}

#[component]
fn App() -> impl IntoView {
    let (bounce_state, set_bounce_state) = signal(BounceState::NotMoving);
    view! {
        <div class="container">
            <div class="box-container">
                <div id="bounce-wrapper">
                    <div id="box-trying-to-become-square"></div>
                </div>
            </div>
            <hr/>
            <ul>
                <li><button on:click=move |_| {
                    match bounce_state.get() {
                        BounceState::NotMoving => {
                            set_anim_state_bounce();
                            set_bounce_state.set(BounceState::Moving);
                        }
                        BounceState::Moving => {
                            set_anim_state_finish_bounce();
                            set_bounce_state.set(BounceState::NotMoving);
                        }
                    }
                }>
                    "bounce"
                </button></li>
                <li><button on:click=move |_| {
                    set_anim_state_backflip();
                }>
                    "backflip"
                </button></li>
            </ul>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    fn set_anim_state_bounce();
    fn set_anim_state_finish_bounce();
    fn set_anim_state_backflip();
}
/*
enum AnimationState {
    Bounce,
}

impl AnimationState {
    fn set_anim_state() {}

    fn finish_animation() {}
}*/
