use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)] //necessary
struct Computers {
    computer_input: RwSignal<String>,
    id: usize,
}

impl Computers {
    fn new(id: usize) -> Self {
        Self {
            computer_input: RwSignal::new(String::new()),
            id,
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let mut list: Vec<Computers> = Vec::new();
    make_computers(&mut list);
    let (list, _) = signal(list);
    view! {
        <div class="container">
            <ForEnumerate
                each=move || list.get()
                key=|computer| computer.id
                let(_, computer)
            >
                <input type="text"
                    on:input:target= move |ev| {
                        //fruit.name.set(get_random_fruit());
                        computer.computer_input.set(ev.target().value());
                    }
                />
                <br/>
                //"test: " {move || computer.computer_input.get()} //do not ask me about 'move ||'
                <Computer command=computer.computer_input/>
            </ForEnumerate>
        </div>
    }
}

#[component]
fn Computer(command: RwSignal<String>) -> impl IntoView {
    view! {
        <div style="border: 1px red solid;">
            <span>"A great day to be a computer"</span><hr/>
            <span>"Computer data: "{move || command.get()}</span> //I repeat. do not ask me about 'move ||' code no workerino otherise. leptos told me how to fix it in the browser console.log
        </div>
    }
}

fn make_computers(list: &mut Vec<Computers>) {
    for _ in 0..5 {
        list.push(Computers::new(list.len()));
    }
}
