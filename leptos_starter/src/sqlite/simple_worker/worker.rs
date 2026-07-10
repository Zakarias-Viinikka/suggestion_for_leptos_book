use gloo_worker::{HandlerId, Worker, WorkerScope};

pub struct Pinger;

impl Worker for Pinger {
    type Input = (); // no meaningful input needed
    type Message = (); // no internal messages
    type Output = &'static str;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, _msg: Self::Input, id: HandlerId) {
        // Ignore the input and just send back a message
        scope.respond(id, "I'm alive!");
    }
}
