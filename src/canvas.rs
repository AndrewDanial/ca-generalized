use crate::life::*;
use crate::states::States;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use std::f64;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[component]
pub fn Canvas() -> impl IntoView {
    let (width, _) = create_signal(1024);
    let (height, _) = create_signal(512);
    let (cell_size, set_cell_size) = create_signal(32 as i32);
    let (paused, set_paused) = create_signal(true);
    let w = move || (width() / cell_size()) as usize;
    let h = move || (height() / cell_size()) as usize;
    let (board, set_board) = create_signal(Board::new(w(), h()));
    let (handle, set_handle): (
        ReadSignal<Option<Result<IntervalHandle, JsValue>>>,
        WriteSignal<Option<Result<IntervalHandle, JsValue>>>,
    ) = create_signal(None);

    let (delay, set_delay) = create_signal(1000);
    let canvas_ref: NodeRef<html::Canvas> = create_node_ref();

    let render_grid = move || {
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();

        for i in (0..=width()).step_by(cell_size() as usize) {
            // ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
            ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
            ctx.begin_path();
            ctx.move_to(i as f64, 0.);
            ctx.line_to(i as f64 + 1., height() as f64);
            ctx.stroke();
        }

        for i in (0..=height()).step_by(cell_size() as usize) {
            ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
            ctx.begin_path();
            ctx.move_to(0., i as f64);
            ctx.line_to(width() as f64, i as f64 + 1.);
            ctx.stroke();
        }
    };

    canvas_ref.on_load(move |canvas_ref| {
        canvas_ref.on_mount(move |_| {
            render_grid();
        });
    });

    let slider_function = move |input: ev::Event| {
        input.prevent_default();
        set_cell_size(event_target_value(&input).parse().unwrap());
        set_board(Board::new(w(), h()));

        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        ctx.clear_rect(0.0, 0.0, width() as f64, height() as f64);

        render_grid();
    };

    let render_board = move || {
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        log!("before get");
        let (grid, state_types) = (board().grid, board().state_types);
        log!("after get");
        let grid_height = grid.len();
        let grid_width = grid[0].len();

        for i in 0..grid_height {
            for j in 0..grid_width {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(
                    state_types[grid[i][j]].color.as_str(),
                ));
                ctx.fill_rect(
                    (j as i32 * cell_size()) as f64,
                    (i as i32 * cell_size()) as f64,
                    cell_size() as f64,
                    cell_size() as f64,
                );
            }
        }
        render_grid();
    };

    let click_function = move |mouse: ev::MouseEvent| {
        let x_index = index(mouse.page_x(), cell_size(), width);
        let y_index = index(mouse.page_y(), cell_size(), height);
        let state = board().state_types[1].clone();
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        set_board.update(|b| b.grid[y_index][x_index] = state.index);
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(&state.color));
        ctx.fill_rect(
            (x_index as i32 * cell_size()) as f64,
            (y_index as i32 * cell_size()) as f64,
            cell_size() as f64,
            cell_size() as f64,
        );
    };
    let update = move || {
        log!("before next");
        let next = board().next();
        log!("after next");
        set_board.update(|b| b.grid = next);
        render_board();
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

    view! {
        <div>
        <canvas on:click=move |mouse| {click_function(mouse)}  width=move || {width()} height=move || {height()} class="canvas" node_ref=canvas_ref></canvas>
        // <div>
        //     "Width: "
        //     <input value=move || {width()} on:keypress=move |ev| {
        //         if ev.key_code() == 13 {
        //             let val = event_target_value(&ev).parse::<i32>();
        //             match val {
        //                 Ok(x) => {
        //                     set_width(x);
        //                     set_board(vec![vec![States::Dead; w()]; h()]);
        //                     render_grid();
        //                 }
        //                 Err(_) => { log!("are you stupid")}
        //             }

        //         }
        //     } >
        //     </input>
        // </div>
        // <div>
        //     "Height: "
        //     <input value=move || {height()} on:keypress=move |ev| {
        //         if ev.key_code() == 13 {
        //             let val = event_target_value(&ev).parse::<i32>();
        //             match val {
        //                 Ok(x) => {
        //                     set_height(x);
        //                     set_board(vec![vec![States::Dead; w()]; h()]);
        //                     render_grid();
        //                 }
        //                 Err(_) => { log!("are you stupid")}
        //             }

        //         }
        //     } >
        //     </input>
        // </div>
        <div>Cell Size: {cell_size}<input type="range" value=move || cell_size()
        min=32 max=128 step=32 on:input=move |ev| {
            slider_function(ev);
        } ></input></div>
        <div><input value="Next" type="button" on:click=move |_| {
            update();
        } ></input></div>

        <div>Delay: {delay}<input type="range" value=move || delay() min="10" max="5000" step="100" on:input=move |ev| {
            timer_function(ev);
        } ></input></div>

        <input type="button" value=move || if paused() { "play" } else { "pause"} on:click=move|_| {
            set_paused(!paused());
            if !paused() {
                set_handle(Some(set_interval_with_handle(update, Duration::from_millis(delay()))));
            }
        }></input>
        <div>
        </div>

        </div>
        <States board=board />
    }
}

fn index(n: i32, x: i32, limit: ReadSignal<i32>) -> usize {
    if n != limit() {
        ((n - n % x) / x) as usize
    } else {
        ((n / x) - 1) as usize
    }
}
