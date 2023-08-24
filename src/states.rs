use crate::life::Board;
use leptos::*;
#[component]
pub fn States(board: ReadSignal<Board>) -> impl IntoView {
    let mut next_id = board().state_types.len();
    let initial_states = (0..next_id)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();
    let (states, set_states) = create_signal(initial_states);

    let add_states = move |_| {
        let sig = create_signal(next_id + 1);
        set_states.update(move |states| {
            states.push((next_id, sig));
        });
        next_id += 1;
    };
    view! {
        States:
        <button on:click=add_states>
            "Add State"
        </button>
        <For
            each=states
            key=|states| states.0
            view=move|(id, (state, set_state))| {
                view! {
                    <div>State {id}</div>
                }
            }/>

    }
}
