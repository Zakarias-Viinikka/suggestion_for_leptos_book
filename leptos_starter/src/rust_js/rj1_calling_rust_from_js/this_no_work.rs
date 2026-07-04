use leptos::prelude::*;

use leptos_meta::Script; //for this it doesn't matter if using <script src...></script> or <Script src.../>

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

/*
 * this example is for showing how to make a js library usable for another js library.
 *
 * if you want to make a js library usable by rust you have to import the functions you wanan call with wasm-bindgen stuff
 * link: https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html
 */

/* IMPORTANT */
/* csr (trunk) and ssr (actix, axum or whatever you use)
 * are going to work slightly differently for what this code is trying to achieve.
 *
 * "Trunk looks for assets in a public or static folder by default. You place your script.js there"
 */

#[component]
fn App() -> impl IntoView {
    view! {
        /*
         * IMPORTANT
         * trunk serve --filehash false
         * will get this to work during development, but it turns off caching so it's not a good solution.
         *
         */
        <Script src="/js.js"/>
        // trunk uses "public" folder in root
    }
}

/*
 * trunk serve --no-spa
 *
 * run the above ^ in console:
 * if trunk doesn't find a file it does some random bullshit otherwise.
 * makes debugging this difficult if not turned off.
 *
 * if the path is wrong, the browser console gives a much more usefull error message now
 */
