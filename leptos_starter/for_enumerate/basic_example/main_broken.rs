use leptos::logging::log; //for console.log
use std::panic;

use leptos::prelude::*;
use rand::seq::IndexedRandom; //rng stuff
/*
use rand::prelude::IndexedRandom; //for getting a random item from array easily.*/
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let list: [String; 5] = std::array::from_fn(|_| "Placeholder".to_string());
    let (list, list_set) = signal(list);
    view! {
        <div class="container">
            <ForEnumerate
                each=move || list.get()
                key=|fruit| fruit.clone()
                let(index, fruit)
            >
                <button on:click=move |_| {
                    list_set.update(|list| list[index.get()] = get_random_fruit());
                    log!("index {}", index.get());
                }>
                    {fruit}
                </button>
            </ForEnumerate>
        </div>
    }
}

fn get_random_fruit() -> String {
    let fruits: [&str; 5] = ["apple", "banana", "orange", "grape", "mango"];

    let mut rng = rand::rng();

    if let Some(random_fruit) = fruits.choose(&mut rng) {
        return random_fruit.to_string();
    } else {
        panic!();
    }
}
