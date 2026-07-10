use gloo_worker::{HandlerId, Worker, WorkerScope};

pub struct SharedThing;

impl Worker for SharedThing {
    type Input = String; // what you send to the worker
    type Message = (); // internal messages (unused)
    type Output = String; // what the worker sends back

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, _msg: Self::Input, id: HandlerId) {
        // Ignore the input, just send back a fixed string.
        scope.respond(id, "Hardcoded response".to_string());
    }
}
