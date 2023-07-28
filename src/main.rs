//#[allow(warnings)]
pub mod canvas;
pub mod life;
use canvas::Canvas;
use leptos::*;
pub use life::*;
use std::panic;
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    leptos::mount_to_body(|cx| view! {cx, <App/>});
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx, <Canvas />}
}
