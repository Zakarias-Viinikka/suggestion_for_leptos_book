use std::sync::LazyLock;

use leptos::reactive::signal::RwSignal;

pub static ANYONE_CAN_ACCESS_ME: LazyLock<RwSignal<String>> =
    LazyLock::new(|| RwSignal::new("hello".into()));
