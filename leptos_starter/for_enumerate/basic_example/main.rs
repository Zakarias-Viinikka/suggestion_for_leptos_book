use leptos::prelude::*;
use rand::seq::IndexedRandom; //rng stuff

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
        name: RwSignal::new("Placeholder".to_string()), //need to use a signal
        id: i,
    });
    let (list, _) = signal(list);
    view! {
        <div class="container">
        /*important */
        // fruit.id is how leptos tracks stuff properly, makes it 'reactive' and whatnot. without it stuff just breaks.
        /*important */
            <ForEnumerate
                each=move || list.get() //signal.get() normal stuff

                key=|fruit| fruit.id // fruit is the equivalent of 'for fruit in list'
                let(index, fruit) // index is fruit.id and fruit is fruit
            >
                //if you don't put anything here the entire view! gets underlined with red, so if you're debugging to make stuff work just put something like:
                // enumerate stuff...>
                // "hello"
                // </forEnumerate>

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
