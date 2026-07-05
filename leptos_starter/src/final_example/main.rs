use leptos::logging::log; //logging
use leptos::prelude::*;
use leptos_starter::javascript_take_the_wheel;
//use leptos_use::UseTextareaAutosizeReturn; //text area
//use leptos_use::use_textarea_autosize; // text area

use anyhow::{Result, anyhow};

//use wasm_bindgen::JsCast; //for macro?
//use leptos::prelude::{on_cleanup, window_event_listener_untyped};
//use wasm_bindgen::{JsCast, JsValue};

use leptos_starter::final_example::js_stuff;
use leptos_starter::final_example::js_value_parsing;

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
                /*for item in list.get().iter() {
                    log!("{}", item.text.get())
                }*/
            }
            Err(e) => log!("{}", e), //console.log error
        }
    });
    //js handle
    view! {
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
    //
    view! {
        <li class="text-container" data-id={move || index.get()}>
            <div class="drag-handle">"⠿"</div>
            <div class="text-input-container">
                <textarea
                    id={move || index.get()}
                    class="textarea"
                    //prop:value=content
                    on:input=move |ev| {
                        text.set(event_target_value(&ev));
                    }
                    placeholder="Type something..."
                ></textarea>
            </div>
        </li>
    }
}
