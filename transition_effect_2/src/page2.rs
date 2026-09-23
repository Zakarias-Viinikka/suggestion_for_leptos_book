use leptos::prelude::*;

#[component]
pub fn Page2() -> impl IntoView {
    view! {
        <div class="margin-page">
            <main class="sheet">
                <h1
                    class="title"
                    contenteditable="plaintext-only"
                    spellcheck="false"
                    data-title=""
                    data-placeholder="Untitled"
                >
                    "Weekly review"
                </h1>
                <div class="blocks">
                    <div class="row">
                        <span class="handle" aria-hidden="true">"⋮⋮"</span>
                        <div
                            class="block"
                            contenteditable="plaintext-only"
                            data-block=""
                            data-placeholder="Write something…"
                        >
                            "Finished the draft of the budget sheet and sent it off. Waiting on two numbers from the landlord."
                        </div>
                    </div>
                    <div class="row">
                        <span class="handle" aria-hidden="true">"⋮⋮"</span>
                        <div
                            class="block"
                            contenteditable="plaintext-only"
                            data-block=""
                            data-placeholder="Write something…"
                        >
                            "Missed the gym twice. Moving the sessions to the morning, before anything else can get in the way."
                        </div>
                    </div>
                    <div class="row">
                        <span class="handle" aria-hidden="true">"⋮⋮"</span>
                        <div
                            class="block"
                            contenteditable="plaintext-only"
                            data-block=""
                            data-placeholder="Write something…"
                        >
                            "Next week: dentist on Tuesday, call the bank, and clear the inbox down to zero by Friday."
                        </div>
                    </div>
                    <div class="row">
                        <span class="handle" aria-hidden="true">"⋮⋮"</span>
                        <div
                            class="block"
                            contenteditable="plaintext-only"
                            data-block=""
                            data-placeholder="Write something…"
                        ></div>
                    </div>
                </div>
            </main>
        </div>
    }
}