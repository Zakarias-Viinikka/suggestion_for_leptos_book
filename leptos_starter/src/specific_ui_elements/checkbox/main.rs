// checkbox examples

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (truth, _) = signal(true);
    //A prop makes it so a value updates from more than just user input. it also lets you set an init value.
    // the checkbox with prop example could be a localstorage deciding if a checkbox should start on or off.
    // or it could be a timer that turns it on or off. without the prop the signal would say one thing, but vizually
    // the website would lie
    //
    // checked also works to set the box. but that theoretically it wouldn't update if something else changed the box
    view! {
        <div>
        //checkbox without prop
        <span>"Checkbox without prop"</span>
        <input type="checkbox"> </input>

        <br/>
        <span>"Checkbox with prop"</span>
        <input type="checkbox" prop:checked=truth> </input>
        </div>
    }
}
