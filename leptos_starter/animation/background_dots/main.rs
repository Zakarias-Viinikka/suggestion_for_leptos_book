use leptos::prelude::*;
use wasm_bindgen::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let grid_x_size = 3;
    let grid_y_size = 3;

    let grid_x_size_copy = grid_x_size.clone() as f64;
    let grid_y_size_copy = grid_y_size.clone() as f64;

    Effect::new(move |_| {
        start_anim(grid_x_size_copy, grid_y_size_copy);
    });

    view! {
        <div class="grid-container">
        {
            let mut vec = Vec::new();
            for y in 0..grid_y_size {
                for x in 0..grid_x_size {
                    vec.push(
                        view! {
                            <div class="grid-dot" id=create_dot_id(x, y)>
                                "●"
                            </div>
                        }
                    );
                }
            }
            vec
        }
        </div>
    }
}

fn create_dot_id(x: usize, y: usize) -> String {
    format!("dot-{}-{}", x, y)
}

// js/animations/background_dots.js
#[wasm_bindgen]
extern "C" {
    fn start_anim(grid_x_size: f64, grid_y_size: f64);
}
