use crate::life::*;
use leptos::html::Canvas;
use leptos::*;
use std::rc::Rc;
#[component]
pub fn States(
    canvas_ref: NodeRef<html::Canvas>,
    width: ReadSignal<i32>,
    height: ReadSignal<i32>,
    r_cell_size: ReadSignal<i32>,
    r_board: ReadSignal<Board>,
    w_board: WriteSignal<Board>,
    render_board: fn(HtmlElement<Canvas>, i32, i32, i32, &Board),
    set_state: WriteSignal<usize>,
) -> impl IntoView {
    let mut next_id = r_board().state_types.len();
    let initial_states = (0..next_id)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (states, set_states) = create_signal(initial_states);
    let add_states = move |_| {
        let sig = create_signal(next_id + 1);
        let default_state = State::new(
            next_id,
            String::from("#FF0000"),
            0,
            vec![
                Rule::new(
                    next_id,
                    2,
                    Rc::new(|count, _| if count == 2 { true } else { false }),
                ),
                Rule::new(
                    next_id,
                    3,
                    Rc::new(|count, _| if count == 3 { true } else { false }),
                ),
            ],
        );
        w_board.update(|b| b.state_types.push(default_state));
        set_states.update(move |states| {
            states.push((next_id, sig));
        });
        next_id += 1;
    };

    let mut remove_states = move |id| {
        log!("{id}");
        set_states.update(|states| {
            states.retain(|(state_id, _)| state_id != &id);
        });
        for i in id..states().len() {
            set_states.update(|a| a[i].0 -= 1);
        }
        next_id -= 1;
    };
    view! {
        States:
        <button on:click=add_states>"Add State"</button>
        <For
            each=states
            key=|states| states.0
            view=move |(id, (_, _))| {
                view! {
                    <div>
                        State
                        {id}
                        <button on:click=move |_| {
                            set_state.set(id);
                        }>"Select State"</button>
                        <button>"Add Rule"</button>
                        <button on:click=move |_| {remove_states(id)}>"Remove State"</button>
                        Color:
                        <input
                            type="color"
                            value=move || { r_board().state_types[id].color.clone() }
                            on:input=move |ev| {
                                w_board
                                    .update(|b| {
                                        b.state_types[id].color = event_target_value(&ev);
                                    });
                                render_board(
                                    canvas_ref.get().unwrap(),
                                    width(),
                                    height(),
                                    r_cell_size(),
                                    &r_board(),
                                );
                            }
                        />

                        Fail State:
                        <select
                            id=id
                            on:input=move |ev| {
                                w_board
                                    .update(|b| {
                                        b
                                            .state_types[id]
                                            .fail_state = event_target_value(&ev)
                                            .parse::<usize>()
                                            .unwrap();
                                    });
                            }
                        >

                            <For
                                each=states
                                key=|states| states.0
                                view=move |(id, (_, _))| {
                                    view! {
                                        <option value=id>
                                            State
                                            {id}
                                        </option>
                                    }
                                }
                            />

                        </select>

                    // <div>
                    // Rules:
                    // {
                    // let next_rule = r_board().state_types[id].rules.len();
                    // let initial_rules = (0..next_rule)
                    // .map(|id| (id, create_signal(id + 1)))
                    // .collect::<Vec<_>>();
                    // let (rules, _) = create_signal(initial_rules);
                    // view!{
                    // <For
                    // each=rules
                    // key=|r| r.0
                    // view=move |(rule_id, (_, _))| {
                    // view!{
                    // <div></div>
                    // "If "<input size="10" value=move || {r_board().state_types[id].rules[rule_id].target_count}/> " neighbors of type "
                    // <select>
                    // <For
                    // each=states
                    // key=|states| states.0
                    // view=move |(id, (_, _))| {
                    // view! {
                    // <option value=id>
                    // State
                    // {id}
                    // </option>
                    // }
                    // }
                    // />
                    // </select>
                    // " go to "
                    // <select>
                    // <For
                    // each=states
                    // key=|states| states.0
                    // view=move |(id, (_, _))| {
                    // view! {
                    // <option value=id>
                    // State
                    // {id}
                    // </option>
                    // }
                    // }
                    // />
                    // </select>

                    // }
                    // }
                    // />
                    // }
                    // }
                    // </div>
                    </div>
                }
            }
        />
    }
}
