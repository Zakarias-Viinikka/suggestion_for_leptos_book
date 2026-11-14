use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (number, _) = signal(5);
    view! {
        <Component1 rs=number/>
    }
}

#[component]
fn Component1(rs: ReadSignal<i32>) -> impl IntoView {
    view! {
        <div class="container">
            {rs.get()}
        </div>
    }
}
