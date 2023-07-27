#[allow(warnings)]
pub mod Canvas;
pub mod life;
use leptos::*;
pub use life::*;
use Canvas::Canvas;
fn main() {
    leptos::mount_to_body(|cx| view! {cx, <App/>});
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx, <Canvas />}
}
