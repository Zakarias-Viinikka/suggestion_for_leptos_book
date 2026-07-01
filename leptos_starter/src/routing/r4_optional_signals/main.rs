use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Component1 /*rs=signal(5).0*//>
    }
}

#[component]
fn Component1(
    //another choice is to give the variable a default value if you don't want to put it in an option.
    #[prop(optional)] rs: Option<ReadSignal<i32>>,
) -> impl IntoView {
    let rs_exists = {
        match rs {
            Some(_) => true,
            None => false,
        }
    };

    view! {
        <div class="container">
            {rs} //rs.get() will work even if you don't check if rs was given or not.

            <br/><br/>

            "the optional signal was given: " {rs_exists}
        </div>
    }
}
