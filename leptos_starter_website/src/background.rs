use leptos::prelude::*;
use ndarray::Array2;

#[component]
pub fn Background() -> impl IntoView {
    let grid_settings = GridSettings::new();
    let grid = new_2d_grid(grid_settings.dimensions);
    let (bounds, bounds_set) = create_signal(MaxXAndY::new(
        grid_settings.dimensions.0,
        grid_settings.dimensions.1,
    ));
    let (dots, dots_set) = create_signal(grid);
    view! {
        <div class="dot-background">
        <table>
        {
            let mut rows = Vec::new();
            //let mut views: Vec<View> = Vec::new();
            for y in 0..dots.get().nrows() {
                let mut td_view_list = Vec::new();
                for x in 0..dots.get().ncols() {
                    //let value = dots.get()[[y, x]];
                    td_view_list.push(view! {
                        <td class={get_dot_class_name(y, x)}>/*{format!("{} {}", x, y)}*/"●"</td>
                    });
                }
                rows.push(view! {
                    <tr>{td_view_list}</tr>
                });
            }
            rows
        }
        </table>
        </div>
    }
}

//this one is a todo for when user changes resolution of screen. the bounds of what my animation is working with should then update
fn change_animation_bounds() {}

struct MaxXAndY {
    x: u8,
    y: u8,
}

impl MaxXAndY {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

fn get_dot_class_name(y: usize, x: usize) -> String {
    format!("dot-{}-{} happy_dot_on_the_grid", y, x)
}

struct GridSettings {
    dimensions: (u8, u8),
    spacing: u8,
}

impl GridSettings {
    fn new() -> Self {
        Self {
            dimensions: (30u8, 20u8),
            spacing: 10u8,
        }
    }
}

fn new_2d_grid(dimensions: (u8, u8)) -> Array2<bool> {
    let dimensions_as_usize = (dimensions.0 as usize, dimensions.1 as usize);
    Array2::from_elem(dimensions_as_usize, false)
}
