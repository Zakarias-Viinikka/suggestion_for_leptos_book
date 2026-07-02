use leptos::logging::log;
use leptos::prelude::*;
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
    //static LIST: OnceLock<RwSignal<Vec<TextBlocks>>> = OnceLock::new();
    let mut list: Vec<TextBlocks> = Vec::new();
    make_text_blocks(&mut list);

    let (list, list_set) = signal(list);
    view! {
        <div class="finale-container">
        /*

        <ul id="sortable-container">

         */
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
                             <input
                                 id=index
                                 type="text"
                                 value=text_blocks.text.get()
                                 on:input:target=move |ev| {
                                     text_blocks.text.set(ev.target().value());
                                 }
                             />
                         </div>
                     </DraggableItem>
                 </ForEnumerate>
             </DraggableZone>
         </Draggable>
        </div>
    }
}

/*#[component]
fn TextBlock(text: RwSignal<String>) -> impl IntoView {
    view! {

    }
}*/

fn make_text_blocks(list: &mut Vec<TextBlocks>) {
    for _ in 0..5 {
        list.push(TextBlocks::new(list.len()));
    }
}
/*
fn drag_visuals_rust(id_of_text_block: usize) {
    drag_visuals_js(id_of_text_block);
}

#[wasm_bindgen]
extern "C" {
    fn drag_visuals_js(id_of_text_block: usize);
}

#[wasm_bindgen]
pub fn on_drag_end(old_index: JsValue, new_index: JsValue) {
    let old_result = js_value_to_usize(&old_index);
    let new_result = js_value_to_usize(&new_index);

    match (old_result, new_result) {
        (Ok(old_id), Ok(new_id)) => {
            // do the swap
        }
        (Err(e), _) | (_, Err(e)) => {
            log!("{e}");
        }
    }

    /*if let (Ok(old_id), Ok(new_id)) = (old_result, new_result) {
        // TODO: swap old_id and new_id in the list
    } else {
        log!("swap failed: old={:?}, new={:?}", old_result, new_result);
    }*/
}

fn js_value_to_usize(value: &JsValue) -> anyhow::Result<usize> {
    Ok(value
        .as_string()
        .ok_or_else(|| anyhow!("JsValue is not a string"))?
        .parse::<usize>()
        .map_err(|_| anyhow!("JsValue was a string but couldn't be converted to usize"))?)
    //.parse::<usize>()
    //.map_err(|e| anyhow!("failed to parse '{}' as usize: {}", e))
}

*/
