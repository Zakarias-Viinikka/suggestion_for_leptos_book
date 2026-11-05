use std::sync::OnceLock;

use leptos::prelude::Get;
use leptos::reactive::signal::RwSignal;

pub struct GlobalSignal(OnceLock<RwSignal<String>>);

impl GlobalSignal {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub fn set(&self, signal: RwSignal<String>) -> Result<(), RwSignal<String>> {
        self.0.set(signal)
    }

    pub fn get(&self) -> String {
        self.0
            .get()
            .expect("this should only panic if there is problem in logic")
            .get()
    }
}

pub static ANYONE_CAN_ACCESS_ME: GlobalSignal = GlobalSignal::new();