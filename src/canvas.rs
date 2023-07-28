use crate::life::*;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use std::f64;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[component]
pub fn Canvas(cx: Scope) -> impl IntoView {
    let (width, set_width) = create_signal(cx, 512);
    let (height, set_height) = create_signal(cx, 512);
    let (cell_size, set_cell_size) = create_signal(cx, 32 as i32);
    let (paused, set_paused) = create_signal(cx, true);
    let w = move || (width() / cell_size()) as usize;
    let h = move || (height() / cell_size()) as usize;
    let (board, set_board) = create_signal(cx, vec![vec![States::Dead; w()]; h()]);
    let (handle, set_handle): (
        ReadSignal<Option<Result<IntervalHandle, JsValue>>>,
        WriteSignal<Option<Result<IntervalHandle, JsValue>>>,
    ) = create_signal(cx, None);

    let (delay, set_delay) = create_signal(cx, 1000);

    let canvas_ref: NodeRef<html::Canvas> = create_node_ref(cx);

    let draw_grid = move |_| {
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();

        for i in (0..=height()).step_by(cell_size() as usize) {
            ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
            ctx.begin_path();
            ctx.move_to(i as f64, 0.);
            ctx.line_to(i as f64 + 1., height() as f64);
            ctx.fill();
        }

        for i in (0..=height()).step_by(cell_size() as usize) {
            ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
            ctx.begin_path();
            ctx.move_to(0., i as f64);
            ctx.line_to(width() as f64, i as f64 + 1.);
            ctx.fill();
        }
    };

    canvas_ref.on_load(cx, draw_grid);
    let click_function = move |mouse: ev::MouseEvent| {
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        // set color of squares to blue

        let x_index = index(mouse.page_x(), cell_size(), width);
        let y_index = index(mouse.page_y(), cell_size(), height);
        if mouse.button() == 0 {
            log!("yeah");
        }
        set_board.update(|b| b[y_index][x_index] = States::Alive);

        for i in 0..board().len() {
            for j in 0..board()[i].len() {
                if let States::Alive = board()[i][j] {
                    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
                } else {
                    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
                }
                ctx.fill_rect(
                    (j as i32 * cell_size()) as f64,
                    (i as i32 * cell_size()) as f64,
                    cell_size() as f64,
                    cell_size() as f64,
                );
            }
        }

        canvas_ref.on_load(cx, draw_grid);
    };

    let slider_function = move |input: ev::Event| {
        input.prevent_default();
        set_cell_size(event_target_value(&input).parse().unwrap());
        set_board(vec![vec![States::Dead; w()]; h()]);

        log!("before error");
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        log!("after error");
        ctx.clear_rect(0.0, 0.0, width() as f64, height() as f64);

        //canvas_ref.on_load(cx, draw_grid);
    };

    let update = move || {
        let next = next(&board());
        set_board.update(|b| *b = next);
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        for i in 0..board().len() {
            for j in 0..board()[i].len() {
                if let States::Alive = board()[i][j] {
                    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
                } else {
                    ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
                }
                ctx.fill_rect(
                    (j as i32 * cell_size()) as f64,
                    (i as i32 * cell_size()) as f64,
                    cell_size() as f64,
                    cell_size() as f64,
                );
            }
        }
        canvas_ref.on_load(cx, draw_grid);
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
    create_effect(cx, move |_| {
        if let Some(h) = handle() {
            if paused() {
                h.unwrap().clear();
            }
        }
    });
    view! {cx,
        <div>
        <canvas on:click=move |mouse| {click_function(mouse)}  width=move || {width()} height=move || {height()} class=("canvas") node_ref=canvas_ref></canvas>
        <div>
            "Width: "
            <input value=move || {width()} on:keypress=move |ev| {
                if ev.key_code() == 13 {
                    let val = event_target_value(&ev).parse::<i32>();
                    match val {
                        Ok(x) => {
                            set_width(x);
                            set_board(vec![vec![States::Dead; w()]; h()]);
                        }
                        Err(_) => { log!("are you stupid")}
                    }

                }
            } >
            </input>
        </div>
        <div>
            "Height: "
            <input value=move || {height()} on:keypress=move |ev| {
                if ev.key_code() == 13 {
                    let val = event_target_value(&ev).parse::<i32>();
                    match val {
                        Ok(x) => {
                            set_height(x);
                            set_board(vec![vec![States::Dead; w()]; h()]);
                        }
                        Err(_) => { log!("are you stupid")}
                    }

                }
            } >
            </input>
        </div>
        <div>Cell Size: {cell_size}<input type="range" value="32"
        min="32" max="128" step="32" on:input=move |ev| {
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
        </div>
    }
}

fn index(n: i32, x: i32, limit: ReadSignal<i32>) -> usize {
    if n != limit() {
        ((n - n % x) / x) as usize
    } else {
        ((n / x) - 1) as usize
    }
}
