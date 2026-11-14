use leptos::html;
use leptos::prelude::*;
//use leptos::tachys::reactive_graph::bind::GetValue;
//use leptos_router::components::A; // for making <A> work
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path; // for the path!() macro

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=DefaultPage/>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
fn DefaultPage() -> impl IntoView {
    let (data, _ /*data_set*/) = signal("Example Data".to_string());

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    //let display_ref: NodeRef<html::Div> = NodeRef::new(); //unused warning
    view! {
        <table>
            <tr>
                <td>
                    <br/>   <br/>   <br/>
                    <h2> "Memo" </h2>
                    <div class="default-container">
                        <hr/>
                        <input node_ref={input_ref} type="text" placeholder="Type here..."/>
                        <hr/>

                        <div>
                            /*<button on:click=move |_| {
                                let value = input_ref.get().unwrap().value();
                                let Some(html_div) = display_ref.get() {
                                    html_div.set_inner_html(&value);
                                };
                                //display_ref.get().unwrap_or_default().set_inner_html(&value);
                            }
                            >"Click button to store input"</button>*/
                            <hr/>
                        </div>
                        <div>
                            <button
                                on:click=move |_|  {
                                    //data_set.set(input_ref.get().unwrap_or("".to_string()).value());
                                }
                            >"Click button to show stored data"</button>
                            <br/>
                            <div>"Output: " {data}</div>
                            <hr/>
                        </div>
                    </div>
                </td>
                <td>
                    "Data stored: " {
                        /*let html_div = ;*/
                        /*if let Some(html_div) = display_ref.get() {
                            return html_div.inner_html();
                        } else {
                            return "".to_string();
                        }*/
                        //"test".to_string()
                    }
                </td>
            </tr>
        </table>
    }
}
