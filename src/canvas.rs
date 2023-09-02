use crate::life::*;
use crate::settings::Settings;
use crate::states::States;
use leptos::html::Canvas;
use leptos::*;
use std::f64;
pub use std::rc::Rc;
use wasm_bindgen::JsCast;
#[component]
pub fn Canvas() -> impl IntoView {
    let (width, _) = create_signal(1024);
    let (height, _) = create_signal(512);
    let (cell_size, set_cell_size) = create_signal(32 as i32);
    let w = move || (width() / cell_size()) as usize;
    let h = move || (height() / cell_size()) as usize;
    let (board, set_board) = create_signal(Board::new(w(), h(), None));
    let (curr_state, set_curr_state) = create_signal(1usize);
    let canvas_ref: NodeRef<html::Canvas> = create_node_ref();

    canvas_ref.on_load(move |canvas_ref| {
        canvas_ref.on_mount(move |x| {
            render_grid(x, width(), height(), cell_size());
        });
    });

    let click_function = move |mouse: ev::MouseEvent| {
        let x_index = index(mouse.page_x(), cell_size(), width);
        let y_index = index(mouse.page_y(), cell_size(), height);
        let state = board().state_types[curr_state()].clone();
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
            (x_index as i32 * cell_size()) as f64 + 1.,
            (y_index as i32 * cell_size()) as f64 + 1.,
            cell_size() as f64 - 1.,
            cell_size() as f64 - 1.,
        );
    };

    view! {
        <div>
            <canvas
                on:click=move |mouse| { click_function(mouse) }
                width=move || { width() }
                height=move || { height() }
                class="canvas"
                node_ref=canvas_ref
            ></canvas>

        </div>
        <Settings
            canvas_ref=canvas_ref
            width=width
            height=height
            r_cell_size=cell_size
            w_cell_size=set_cell_size
            r_board=board
            w_board=set_board
            render_grid=render_grid
            render_board=render_board
        />
        <States
            canvas_ref=canvas_ref
            width=width
            height=height
            r_cell_size=cell_size
            r_board=board
            w_board=set_board
            render_grid=render_grid
            render_board=render_board
            set_state=set_curr_state
        />
    }
}

fn index(n: i32, x: i32, limit: ReadSignal<i32>) -> usize {
    if n != limit() {
        ((n - n % x) / x) as usize
    } else {
        ((n / x) - 1) as usize
    }
}

fn render_grid(canvas_ref: HtmlElement<Canvas>, width: i32, height: i32, cell_size: i32) {
    let ctx = canvas_ref
        .get_context("2d")
        .ok()
        .flatten()
        .expect("canvas to have context")
        .unchecked_into::<web_sys::CanvasRenderingContext2d>();

    for i in (0..=width).step_by(cell_size as usize) {
        ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        ctx.begin_path();
        ctx.move_to(i as f64, 0.);
        ctx.line_to(i as f64 + 1., height as f64);
        ctx.stroke();
    }

    for i in (0..=height).step_by(cell_size as usize) {
        ctx.set_stroke_style(&wasm_bindgen::JsValue::from_str("#FFFFFF"));
        ctx.begin_path();
        ctx.move_to(0., i as f64);
        ctx.line_to(width as f64, i as f64 + 1.);
        ctx.stroke();
    }
}

fn render_board(
    canvas_ref: HtmlElement<Canvas>,
    width: i32,
    height: i32,
    cell_size: i32,
    board: &Board,
) {
    let ctx = canvas_ref
        .get_context("2d")
        .ok()
        .flatten()
        .expect("canvas to have context")
        .unchecked_into::<web_sys::CanvasRenderingContext2d>();
    let (grid, state_types) = (board.grid.clone(), board.state_types.clone());
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    for i in 0..grid_height {
        for j in 0..grid_width {
            if !state_types.is_empty() {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(
                    state_types[grid[i][j]].color.as_str(),
                ));
            } else {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
            }

            ctx.fill_rect(
                (j as i32 * cell_size) as f64,
                (i as i32 * cell_size) as f64,
                cell_size as f64,
                cell_size as f64,
            );
        }
    }
    render_grid(canvas_ref, width, height, cell_size);
}
