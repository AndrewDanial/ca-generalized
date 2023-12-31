//#[allow(warnings)]
pub mod canvas;
pub mod life;
pub mod settings;
pub mod states;
use canvas::Canvas;
use leptos::*;
extern crate console_error_panic_hook;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    leptos::mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    view! { <Canvas/> }
}
