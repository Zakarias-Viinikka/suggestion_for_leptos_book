use leptos::logging::log; //for console.log
use std::panic;

use leptos::prelude::set_timeout; //for going to sleep

use leptos::prelude::*;
use rand::seq::IndexedRandom; //rng stuff
/*
use rand::prelude::IndexedRandom; //for getting a random item from array easily.*/
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone, PartialEq)] // necessary
struct Fruit {
    name: String,
    id: usize,
}

#[component]
fn App() -> impl IntoView {
    let list: [Fruit; 1000] = std::array::from_fn(|i| Fruit {
        name: "Placeholder".to_string(),
        id: i,
    });
    let (list, list_set) = signal(list);
    view! {
        <div class="container">
            <ForEnumerate
                each=move || list.get() //signal.get() normal stuff
                key=|fruit| fruit.id // fruit is the equivalent of 'for fruit in list' // fruit.id is how leptos tracks stuff properly. without it stuff just breaks.
                let(index, _fruit) // index is fruit.id and _fruit is fruit
            >
                //view! { // with the let syntax view! shouldn't be used
                    <button on:click=move |_| {
                        list_set.update(|list| list[index.get()].name = get_random_fruit());
                        log!("index {}", index.get());
                    }>
                        {move || list.get()[index.get()].name.clone()} //without the move it's not reactive
                    </button>
                    //}
            </ForEnumerate>
        </div>
    }
}

/**/

fn get_random_fruit() -> String {
    let fruits: [&str; 5] = ["apple", "banana", "orange", "grape", "mango"];

    let mut rng = rand::rng();

    if let Some(random_fruit) = fruits.choose(&mut rng) {
        random_fruit.to_string()
    } else {
        panic!();
    }
}
