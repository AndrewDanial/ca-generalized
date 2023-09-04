use crate::life::*;
use js_sys::Math::random;
use leptos::html::Canvas;
use leptos::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[component]
pub fn Settings(
    canvas_ref: NodeRef<html::Canvas>,
    width: ReadSignal<i32>,
    height: ReadSignal<i32>,
    r_cell_size: ReadSignal<i32>,
    w_cell_size: WriteSignal<i32>,
    r_board: ReadSignal<Board>,
    w_board: WriteSignal<Board>,
    render_grid: fn(HtmlElement<Canvas>, i32, i32, i32),
    render_board: fn(HtmlElement<Canvas>, i32, i32, i32, &Board),
) -> impl IntoView {
    let (handle, set_handle): (
        ReadSignal<Option<Result<IntervalHandle, JsValue>>>,
        WriteSignal<Option<Result<IntervalHandle, JsValue>>>,
    ) = create_signal(None);
    let w = move || (width() / r_cell_size()) as usize;
    let h = move || (height() / r_cell_size()) as usize;
    let (paused, set_paused) = create_signal(true);
    let (delay, set_delay) = create_signal(1000);

    let slider_function = move |input: ev::Event| {
        input.prevent_default();
        w_cell_size(event_target_value(&input).parse().unwrap());
        w_board(Board::new(w(), h(), None));

        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        ctx.clear_rect(0.0, 0.0, width() as f64, height() as f64);

        render_grid(canvas_ref.get().unwrap(), width(), height(), r_cell_size());
    };

    let update = move || {
        let next = r_board().next();
        w_board.update(|b| b.grid = next);
        render_board(
            canvas_ref.get().unwrap(),
            width(),
            height(),
            r_cell_size(),
            &r_board(),
        );
    };

    let timer_function = move |input: ev::Event| {
        set_delay(event_target_value(&input).parse().unwrap());
        if !paused() {
            handle().unwrap().unwrap().clear();
            set_handle(Some(set_interval_with_handle(
                update,
                Duration::from_millis(delay()),
            )));
        }
    };

    create_effect(move |_| {
        if let Some(h) = handle() {
            if paused() {
                h.unwrap().clear();
            }
        }
    });

    let gen_rand = move || {
        let mut grid = vec![vec![0; w()]; h()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let rand = (random() * r_board().state_types.len() as f64) as usize;
                grid[i][j] = rand;
            }
        }
        w_board.update(|b| b.grid = grid);
    };

    view! {
        <button on:click=move |_| {gen_rand(); render_board(canvas_ref.get().unwrap(), width(), height(), r_cell_size(), &r_board())}>Generate Random Board</button>
        // <div>
        //     Cell Size:
        //     {r_cell_size}
        //     <input
        //         type="range"
        //         value=move || r_cell_size()
        //         min=32
        //         max=128
        //         step=32
        //         on:input=move |ev| {
        //             slider_function(ev);
        //         }
        //     />

        // </div>

        <div>
            <input
                value="Next"
                type="button"
                on:click=move |_| {
                    update();
                }
            />

        </div>

        <div>
            Delay:
            {delay}
            <input
                type="range"
                value=move || delay()
                min="10"
                max="5000"
                step="100"
                on:input=move |ev| {
                    timer_function(ev);
                }
            />

        </div>

        <input
            type="button"
            value=move || if paused() { "play" } else { "pause" }
            on:click=move |_| {
                set_paused(!paused());
                if !paused() {
                    set_handle(
                        Some(set_interval_with_handle(update, Duration::from_millis(delay()))),
                    );
                }
            }
        />

        <div></div>
    }
}
