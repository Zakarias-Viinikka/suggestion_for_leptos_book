use gloo_worker::Registrable;
use leptos_starter::sqlite::gloo_worker::shared::SharedThing;

fn main() {
    console_error_panic_hook::set_once();

    SharedThing::registrar().register();
}
