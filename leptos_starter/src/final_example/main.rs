use leptos::logging::log; //logging
use leptos::prelude::*;
use leptos_use::UseTextareaAutosizeReturn; //text area
use leptos_use::use_textarea_autosize; // text area

//use leptos::html::Textarea; //for using type NodeRef::textarea outside of component
/*for the drag reorder visuals */
/*
 * https://rust-ui.com/docs/components/drag-and-drop
 *
 * cargo install ui-cli --force
 * ui add drag_and_drop
 * ui add drag_and_drop
 */
//use crate::components::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};
use leptos_starter::components::ui::drag_and_drop::{Draggable, DraggableItem, DraggableZone};

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
    let mut list: Vec<TextBlocks> = Vec::new();

    //make text blocks
    for _ in 0..5 {
        list.push(TextBlocks::new(list.len()));
    }
    //make text blocks

    let (list, list_set) = signal(list);
    view! {
        <div class="finale-container">
         <Draggable>
             <DraggableZone>
                 <ForEnumerate
                     each=move || list.get()
                     key=|text_blocks| text_blocks.id
                     let(index, text_blocks)
                 >
                     <DraggableItem /* ... */ >
                         <div class="drag-handle">"⠿"</div>
                         <div class="text-input-container">
                            <TextArea
                                index=index.get()
                                text=text_blocks.text
                            />
                         </div>
                     </DraggableItem>
                 </ForEnumerate>
             </DraggableZone>
         </Draggable>
        </div>
    }
}

#[component]
fn TextArea(index: usize, text: RwSignal<String>) -> impl IntoView {
    //textarea
    let node_ref = NodeRef::new();
    let UseTextareaAutosizeReturn {
        content,
        set_content,
        trigger_resize,
    } = use_textarea_autosize(node_ref);
    //textarea
    //
    view! {
        <textarea
            id=index
            class="textarea resize-none"
            node_ref=node_ref
            prop:value=content
            on:input=move |ev| {
                set_content.set(event_target_value(&ev));//so the resize stuff works
                text.set(event_target_value(&ev)); //updates the signal
            }
            placeholder="Type something..."
        ></textarea>
    }
}
