//#[allow(warnings)]
pub mod canvas;
pub mod life;
use canvas::Canvas;
use leptos::*;
pub use life::*;
fn main() {
    leptos::mount_to_body(|cx| view! {cx, <App/>});
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx, <Canvas />}
}
