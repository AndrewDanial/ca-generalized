use crate::life::*;
use leptos::html::Canvas;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
#[component]
pub fn States(
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
    let mut next_id = r_board().state_types.len();
    let initial_states = (0..next_id)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (states, set_states) = create_signal(initial_states);

    let add_states = move |_| {
        let sig = create_signal(next_id + 1);
        let default_state = State::new(
            next_id,                 // index
            String::from("#000000"), // color
            0,                       // fail state
            vec![Rule::new(
                1,
                Rc::new(|count| if count == 3 { true } else { false }),
            )],
        );
        w_board.update(|b| b.state_types.push(default_state));
        set_states.update(move |states| {
            states.push((next_id, sig));
        });
        next_id += 1;
    };
    view! {
        States:
        <button on:click=add_states>"Add State"</button>
        <For
            each=states
            key=|states| states.0
            view=move |(id, (state, set_state))| {
                view! {
                    <div>
                        State
                        {id}
                        <button>
                            Select State
                        </button>
                        <button>
                            Add Rule
                        </button>
                        <p>
                            <input
                                type="color"
                                value=move || { r_board().state_types[id].color.clone() }
                                on:input=move |ev| {
                                    w_board.update(|b| b.state_types[id].color = event_target_value(&ev));
                                    render_board(canvas_ref.get().unwrap(), width(), height(), r_cell_size(), &r_board());
                                }
                            />
                        </p>
                    </div>
                }
            }
        />
    }
}
