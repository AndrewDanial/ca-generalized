use crate::life::*;
use js_sys::Math::random;
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
    r_board: ReadSignal<Board>,
    w_board: WriteSignal<Board>,
    render_grid: fn(HtmlElement<Canvas>, i32, i32, i32),
    render_board: fn(HtmlElement<Canvas>, i32, i32, i32, &Board),
    set_state: WriteSignal<usize>,
) -> impl IntoView {
    let (next_id, set_next_id) = create_signal(r_board().state_types.len());
    let initial_states = (0..next_id())
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (states, set_states) = create_signal(initial_states);

    let add_states = move |_| {
        let sig = create_signal(next_id() + 1);
        let random_color = format!("#{:X}", ((random() * 16777215.) as i32));
        let default_state = State::new(
            next_id(),
            random_color,
            (random() * r_board().state_types.len() as f64) as usize,
            vec![
                Rule::new(
                    next_id(),
                    (random() * 8. + 1.) as u32,
                    (random() * r_board().state_types.len() as f64) as usize,
                    Rc::new(|count, target| if count == target { true } else { false }),
                ),
                Rule::new(
                    next_id(),
                    (random() * 8. + 1.) as u32,
                    (random() * r_board().state_types.len() as f64) as usize,
                    Rc::new(|count, target| if count == target { true } else { false }),
                ),
            ],
        );
        w_board.update(|b| b.state_types.push(default_state));
        set_states.update(move |states| {
            states.push((next_id(), sig));
        });

        let w = move || (width() / r_cell_size()) as usize;
        let h = move || (height() / r_cell_size()) as usize;

        w_board.set(Board::new(w(), h(), Some(r_board().state_types.clone())));
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(
            &r_board().state_types[0].color,
        ));
        ctx.fill_rect(0.0, 0.0, width() as f64, height() as f64);

        render_grid(canvas_ref.get().unwrap(), width(), height(), r_cell_size());
        set_next_id.update(|a| *a += 1);
    };

    let remove_states = move |id| {
        set_states.update(|states| {
            states.retain(|(state_id, _)| &(*state_id as i32) != &(id as i32));
        });
        for i in id..states().len() {
            set_states.update(|a| a[i].0 -= 1);
        }

        set_next_id.update(|a| *a -= 1);
        w_board.update(|b| {
            b.state_types.remove(id);
        });
        for i in id..r_board().state_types.len() {
            w_board.update(|b| b.state_types[i].index -= 1);
        }

        for i in 0..r_board().state_types.len() {
            for j in 0..r_board().state_types[i].rules.len() {
                let rule = r_board().state_types[i].rules[j].clone();
                if rule.count_state >= r_board().state_types.len() {
                    w_board.update(|b| b.state_types[i].rules[j].count_state = 0);
                }
                if rule.target_state >= r_board().state_types.len() {
                    w_board.update(|b| b.state_types[i].rules[j].target_state = 0);
                }
            }
        }
        let w = move || (width() / r_cell_size()) as usize;
        let h = move || (height() / r_cell_size()) as usize;

        w_board.set(Board::new(
            w() as usize,
            h() as usize,
            Some(r_board().state_types.clone()),
        ));
        let ctx = canvas_ref
            .get()
            .unwrap()
            .get_context("2d")
            .ok()
            .flatten()
            .expect("canvas to have context")
            .unchecked_into::<web_sys::CanvasRenderingContext2d>();
        ctx.clear_rect(0.0, 0.0, width() as f64, height() as f64);
        if id == 0 {
            render_board(
                canvas_ref.get().unwrap(),
                width(),
                height(),
                r_cell_size(),
                &r_board(),
            );
        } else {
            render_grid(canvas_ref.get().unwrap(), width(), height(), r_cell_size());
        }
    };

    view! {
        States:
        <button on:click=add_states>"Add State"</button>
        <For
            each=states
            key=|states| states.0
            view=move |(state_index, (_, _))| {
                view! {
                    <div>
                        State
                        {state_index}
                        <button on:click=move |_| {
                            set_state.set(state_index);
                        }>"Select State"</button>

                        <button on:click=move |_| {
                            remove_states(state_index)
                        }>"Remove State"</button>
                        Color:
                        <input
                            type="color"
                            value=move || { let c = r_board().state_types[state_index].color.clone();
                            c }
                            on:input=move |ev| {
                                w_board
                                    .update(|b| {
                                        b.state_types[state_index].color = event_target_value(&ev);
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
                            on:input=move |ev| {
                                w_board
                                    .update(|b| {
                                        b
                                            .state_types[state_index]
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
                                        <option id=state_index value=id selected=move || {
                                            let fail = r_board().state_types[state_index].fail_state;
                                            fail == id
                                        }>
                                            State
                                            {id}
                                        </option>
                                    }
                                }
                            />

                        </select>

                        <div>

                            {
                                let (next_rule, set_next_rule) = create_signal(
                                    r_board().state_types[state_index].rules.len(),
                                );
                                let initial_rules = (0..next_rule())
                                    .map(|id| (id, create_signal(id + 1)))
                                    .collect::<Vec<_>>();
                                let (rules, set_rules) = create_signal(initial_rules);
                                let add_rule = move |_| {
                                    let sig = create_signal(next_rule() + 1);
                                    w_board
                                        .update(|board| {
                                            board
                                                .state_types[state_index]
                                                .rules
                                                .push(
                                                    Rule::new(
                                                        next_rule(),
                                                        3,
                                                        1,
                                                        Rc::new(|count, target_count| {
                                                            if count == target_count { true } else { false }
                                                        }),
                                                    ),
                                                )
                                        });
                                    set_rules
                                        .update(move |rules| {
                                            rules.push((next_rule(), sig));
                                        });
                                    set_next_rule.update(|a| *a += 1);
                                };
                                let remove_rule = move |id| {
                                    set_rules
                                        .update(|states| {
                                            states.retain(|(r_id, _)| &(*r_id as i32) != &(id as i32));
                                        });
                                    for i in id..rules().len() {
                                        set_rules.update(|a| a[i].0 -= 1);
                                    }
                                    set_next_rule.update(|a| *a -= 1);
                                    w_board
                                        .update(|b| {
                                            b.state_types[state_index].rules.remove(id);
                                        });
                                };
                                let set_count = move |ev, state_index: usize, rule_id: usize| {
                                    w_board
                                        .update(|b| {
                                            b.state_types[state_index].rules[rule_id].target_count = ev;
                                        });
                                };

                                view! {
                                    Rules:
                                    <button on:click=add_rule>"Add Rule"</button>
                                    <For
                                        each=rules
                                        key=|rule| rule.0
                                        view=move |(rule_id, (_, _))| {
                                            view! {
                                                <div></div>
                                                "If "
                                                <input
                                                    value=move || {
                                                        r_board()
                                                            .state_types[state_index]
                                                            .rules[rule_id]
                                                            .target_count
                                                    }

                                                    type="number"
                                                    size="1"
                                                    min="0"
                                                    max="8"
                                                    on:input=move |ev| {
                                                        set_count(
                                                            event_target_value(&ev).parse().unwrap(),
                                                            state_index,
                                                            rule_id,
                                                        );
                                                    }
                                                />

                                                " neighbors of type "
                                                <select
                                                    on:input=move |ev| {
                                                        let count = event_target_value(&ev).parse::<usize>().unwrap();
                                                        w_board
                                                            .update(|b| {
                                                                b.state_types[state_index].rules[rule_id].count_state = count;
                                                            });
                                                    }
                                                >
                                                    <For
                                                        each=states
                                                        key=|states| states.0

                                                        view=move |(id, (_, _))| {
                                                            view! {
                                                                <option
                                                                    value=id
                                                                    id=rule_id
                                                                    selected=move || {
                                                                        let count = r_board().state_types[state_index].rules[rule_id].count_state;
                                                                        count == id
                                                                    }
                                                                >
                                                                    State
                                                                    {id}
                                                                </option>
                                                            }
                                                        }
                                                    />

                                                </select>
                                                " go to "
                                                <select
                                                    on:input=move |ev| {
                                                        let target = event_target_value(&ev).parse::<usize>().unwrap();
                                                        w_board
                                                            .update(|b| {
                                                                b.state_types[state_index].rules[rule_id].target_state = target;
                                                            });
                                                    }
                                                >
                                                    <For
                                                        each=states
                                                        key=|states| states.0
                                                        view=move |(id, (_, _))| {
                                                            view! {
                                                                <option id=rule_id value=id selected=move || {
                                                                    let target = r_board().state_types[state_index].rules[rule_id].target_state;
                                                                    target == id
                                                                }>
                                                                    State
                                                                    {id}
                                                                </option>
                                                            }
                                                        }
                                                    />

                                                </select>

                                                <button on:click=move |_| {
                                                    remove_rule(rule_id)
                                                }>"Remove Rule"</button>
                                            }
                                        }
                                    />
                                }
                            }

                        </div>
                    </div>
                }
            }
        />
    }
}
