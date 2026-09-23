use leptos::logging::error;
use leptos::prelude::*;

pub const SIZE: f64 = 4000.0;
pub const GAP: f64 = 30.0;

pub const BUTTONS: [(f64, f64, &str); 5] = [
    (50.0, 50.0, "Center"),
    (25.0, 25.0, "Top Left"),
    (75.0, 25.0, "Top Right"),
    (25.0, 75.0, "Bottom Left"),
    (75.0, 75.0, "Bottom Right"),
];

#[derive(Clone, Copy)]
pub struct BoxController {
    pub cx: RwSignal<f64>,
    pub cy: RwSignal<f64>,
    pub delay1: RwSignal<u32>,
    pub delay2: RwSignal<u32>,
    pub spread: RwSignal<f64>,
}

impl BoxController {
    pub fn new(cx: f64, cy: f64, spread: f64) -> Self {
        Self {
            cx: RwSignal::new(cx),
            cy: RwSignal::new(cy),
            delay1: RwSignal::new(0),
            delay2: RwSignal::new(0),
            spread: RwSignal::new(spread),
        }
    }

    pub fn move_to(&self, target_cx: f64, target_cy: f64) {
        let (vw, vh) = match window_size() {
            Ok(v) => v,
            Err(e) => {
                error!("{e}");
                return;
            }
        };

        let offset = (SIZE + GAP) / (2.0 * std::f64::consts::SQRT_2);

        let cur_cx = self.cx.get_untracked();
        let cur_cy = self.cy.get_untracked();

        let box1_x = cur_cx / 100.0 * vw - offset;
        let box1_y = cur_cy / 100.0 * vh + offset;
        let box2_x = cur_cx / 100.0 * vw + offset;
        let box2_y = cur_cy / 100.0 * vh - offset;

        let target_x = target_cx / 100.0 * vw;
        let target_y = target_cy / 100.0 * vh;

        let dist1 = ((box1_x - target_x).powi(2) + (box1_y - target_y).powi(2)).sqrt();
        let dist2 = ((box2_x - target_x).powi(2) + (box2_y - target_y).powi(2)).sqrt();

        self.delay1.set(if dist1 <= dist2 { 0 } else { 200 });
        self.delay2.set(if dist2 < dist1 { 0 } else { 200 });

        self.cx.set(target_cx);
        self.cy.set(target_cy);
    }
}

#[component]
pub fn Theme7Boxes() -> impl IntoView {
    let c = expect_context::<BoxController>();
    let offset = (SIZE + GAP) / (2.0 * std::f64::consts::SQRT_2);

    let timing = move || {
        if c.spread.get() > 0.0 {
            ("3s", "cubic-bezier(0.25, 0.1, 0.25, 1)")
        } else {
            ("0.7s", "cubic-bezier(0.0, 0.0, 0.2, 1)")
        }
    };

    let box1_style = move || {
        let (dur, curve) = timing();
        format!(
            "left: calc({}% - {}px - {}vmax); \
             top: calc({}% + {}px + {}vmax); \
             transition: left {dur} {curve}, top {dur} {curve}; \
             transition-delay: {}ms;",
            c.cx.get(), offset, c.spread.get(),
            c.cy.get(), offset, c.spread.get(),
            c.delay1.get(),
        )
    };
    let box2_style = move || {
        let (dur, curve) = timing();
        format!(
            "left: calc({}% + {}px + {}vmax); \
             top: calc({}% - {}px - {}vmax); \
             transition: left {dur} {curve}, top {dur} {curve}; \
             transition-delay: {}ms;",
            c.cx.get(), offset, c.spread.get(),
            c.cy.get(), offset, c.spread.get(),
            c.delay2.get(),
        )
    };

    view! {
        <div class="box box1" style=box1_style />
        <div class="box box2" style=box2_style />
    }
}

fn window_size() -> Result<(f64, f64), String> {
    let win = web_sys::window().ok_or_else(|| "no window".to_string())?;
    let w = win
        .inner_width()
        .map_err(|e| format!("inner_width: {:?}", e))?
        .as_f64()
        .ok_or_else(|| "inner_width not a number".to_string())?;
    let h = win
        .inner_height()
        .map_err(|e| format!("inner_height: {:?}", e))?
        .as_f64()
        .ok_or_else(|| "inner_height not a number".to_string())?;
    Ok((w, h))
}