use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

// *** IMPORTANT
//<For> component
// requires struct to have id field
// and struct needs to derive Clone
macro_rules! for_leptos {
    ($list:expr, $item:ident => $body:expr) => {
        view! {
            <For
                each=move || $list.get()
                key=|$item| $item.id
                children=move |$item| $body
            />
        }
    };
}

#[derive(Clone)]
struct item {
    text: String,
    id: usize,
}

#[component]
fn App() -> impl IntoView {
    let (list, list_set) = signal(Vec::new());
    for i in 0..5 {
        list_set.update(|list| {
            list.push(item {
                text: format!("item {}", i),
                id: i,
            })
        });
    }
    view! {
        <div class="container">
        //<For> macro usage
        {for_leptos!(list, list_item => {
            view! {
                <div>
                    "text: " {list_item.text.clone()}
                    <br/>
                    "id: " {list_item.id}
                    <br/>
                    <br/>
                </div>
            }
        })}
        </div>
    }
}
