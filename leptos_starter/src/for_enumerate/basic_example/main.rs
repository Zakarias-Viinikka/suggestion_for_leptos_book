use leptos::prelude::*;
use rand::seq::IndexedRandom; //rng stuff
/*
use rand::prelude::IndexedRandom; //for getting a random item from array easily.*/
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)] //necessary
struct Fruit {
    name: RwSignal<String>,
    id: usize,
}

#[component]
fn App() -> impl IntoView {
    let list: [Fruit; 1000] = std::array::from_fn(|i| Fruit {
        name: RwSignal::new("Placeholder".to_string()),
        id: i,
    });
    let (list, _) = signal(list);
    view! {
        <div class="container">
            <ForEnumerate
                each=move || list.get() //signal.get() normal stuff
                key=|fruit| fruit.id // fruit is the equivalent of 'for fruit in list' // fruit.id is how leptos tracks stuff properly. without it stuff just breaks.
                let(index, fruit) // index is fruit.id and fruit is fruit
            >
                //view! { // with the let syntax view! isn't needed
                    <button on:click=move |_| {
                        fruit.name.set(get_random_fruit());
                    }>
                        {list.get()[index.get()].name.clone()}
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
        unreachable!();
    }
}
