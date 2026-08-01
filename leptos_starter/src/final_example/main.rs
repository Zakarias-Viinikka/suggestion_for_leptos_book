use leptos::logging::log; //logging
use leptos::prelude::*;
use leptos_meta::*;
use leptos_starter::final_example::js_stuff;
use leptos_starter::final_example::js_value_parsing;
use leptos_starter::javascript_take_the_wheel;

use leptos::task::spawn_local_scoped;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)] //necessary
struct TextBlocks {
    text: RwSignal<String>,
    id: usize,
}

impl TextBlocks {
    fn new(id: usize) -> Self {
        Self {
            text: RwSignal::new(String::new()),
            id,
        }
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let list = RwSignal::new(Vec::new());
    //make text blocks
    list.update(|l| {
        for _ in 0..5 {
            l.push(TextBlocks::new(l.len()));
        }
    });
    //make text blocks

    //js handle //the macro is from rust_js/rj3_cleaner_code/handle_macro.rs
    javascript_take_the_wheel!("update_list_order", |js_value| {
        match js_value_parsing::js_value_to_usize_tuple(js_value) {
            Ok((old_index, new_index)) => {
                list.update(|v| {
                    let item = v.remove(old_index);
                    v.insert(new_index, item);
                });
            }
            Err(e) => log!("{}", e), //console.log error
        }
    });
    //js handle

    //js.js should load after Sortable.js otherwise javascript gets all uppity
    let (sortablejs_has_loaded, set_sortablejs_has_loaded) = signal(false);

    // Trigger the wait loop exactly once
    Effect::new(move |_| {
        if !sortablejs_has_loaded.get_untracked() {
            spawn_local_scoped(wait_for_sortable(set_sortablejs_has_loaded));
        }
    });

    view! {
        <Stylesheet href="/public/finale/finale.css"/>
        <Script src="/public/finale/Sortable.js"/>
        //insert the script once sortablejs has loaded
        {move || {
            if sortablejs_has_loaded.get() {
                view! { <Script src="/public/finale/js.js"/> }.into_any()
            } else {
                view! { "" }.into_any()
            }
        }}

        <div class="finale-container">
            <ul id="sortable-container">
                 <ForEnumerate
                     each=move || list.get()
                     key=|text_blocks| text_blocks.id
                     let(index, text_blocks)
                >
                    <TextArea
                        index=index
                        text=text_blocks.text
                    />
                 </ForEnumerate>
             </ul>
             <div>
             "this is all of the textblocks combined:"
             <ForEnumerate
                 each=move || list.get()
                 key=|text_blocks| text_blocks.id
                 let(_, text_blocks)
             >
                <span>
                    {move ||
                        text_blocks.text.get()
                    }
                    <br/>
                </span>
             </ForEnumerate>
             </div>
        </div>
        /*
         * https://github.com/leptos-rs/leptos/discussions/1471
         */
        <js_stuff::JsStuff />
    }
}

#[component]
fn TextArea(index: ReadSignal<usize>, text: RwSignal<String>) -> impl IntoView {
    view! {
        <li class="text-container" data-id={move || index.get()}>
            <div class="drag-handle">"⠿"</div>
            <div class="text-input-container">
                <textarea
                    id={move || index.get()}
                    class="textarea"
                    on:input=move |ev| {
                        text.set(event_target_value(&ev));
                    }
                    placeholder="Type something..."
                ></textarea>
            </div>
        </li>
    }
}

// Function that loops until Sortable is defined, then flips the signal
async fn wait_for_sortable(setter: WriteSignal<bool>) {
    loop {
        let ok = web_sys::window()
            .and_then(|w| w.get("Sortable"))
            .map(|_| true)
            .unwrap_or(false);
        if ok {
            setter.set(true);
            break;
        }
        // Wait 50ms before next check (uses gloo-timers)
        gloo_timers::future::TimeoutFuture::new(50).await;
    }
}
