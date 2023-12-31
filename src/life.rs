use std::rc::Rc;
// If the predicate is true, go to target_state, else go to state's fail state
#[derive(Clone)]
pub struct Rule {
    pub target_state: usize,
    pub target_count: u32,
    pub count_state: usize,
    pub predicate: Rc<dyn Fn(u32, u32) -> bool>,
}

#[derive(Clone)]
pub struct State {
    pub index: usize,
    pub color: String,
    pub fail_state: usize,
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Board {
    pub grid: Vec<Vec<usize>>,
    pub state_types: Vec<State>,
}

impl Rule {
    pub fn new(
        target_state: usize, // state to go to if predicate is true
        target_count: u32,   // amount of neighbor of state type
        count_state: usize,  // the state type to check count of
        predicate: Rc<dyn Fn(u32, u32) -> bool>,
    ) -> Self {
        Rule {
            target_state,
            target_count,
            count_state,
            predicate,
        }
    }
}

impl State {
    pub fn new(index: usize, color: String, fail_state: usize, rules: Vec<Rule>) -> Self {
        State {
            index,
            color,
            fail_state,
            rules,
        }
    }
}

impl Board {
    pub fn new(width: usize, height: usize, state_types: Option<Vec<State>>) -> Self {
        if let Some(state_types) = state_types {
            Board {
                grid: vec![vec![0; width]; height],
                state_types,
            }
        } else {
            // By default, the board uses regular game of life rules.
            let state_0 = State::new(
                0,                       // index
                String::from("#000000"), // color
                0,                       // fail state
                vec![Rule::new(
                    1, // target state
                    3, // target count
                    1, // count state
                    Rc::new(|count, target_count| if count == target_count { true } else { false }),
                )],
            );

            let state_1 = State::new(
                1,
                String::from("#FFFFFF"),
                0,
                vec![
                    Rule::new(
                        1,
                        2,
                        1,
                        Rc::new(
                            |count, target_count| if count == target_count { true } else { false },
                        ),
                    ),
                    Rule::new(
                        1,
                        3,
                        1,
                        Rc::new(
                            |count, target_count| if count == target_count { true } else { false },
                        ),
                    ),
                ],
            );
            let states = vec![state_0, state_1];
            Board {
                grid: vec![vec![0; width]; height],
                state_types: states,
            }
        }
    }

    pub fn next(&self) -> Vec<Vec<usize>> {
        let mut next_gen = self.grid.clone();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let neighbors = self.count_neighbors(x as i32, y as i32);
                let mut found = false;
                for rule in self.state_types[self.grid[y][x]].clone().rules {
                    let output = (rule.predicate)(neighbors[rule.count_state], rule.target_count);
                    if output {
                        next_gen[y][x] = self.state_types[rule.target_state].index;
                        found = true;
                        break;
                    }
                }
                if !found {
                    next_gen[y][x] = self.state_types[self.grid[y][x]].fail_state;
                }
            }
        }
        next_gen
    }

    fn count_neighbors(&self, x: i32, y: i32) -> Vec<u32> {
        let mut counter = vec![0; self.state_types.len()];
        for i in -1 as i32..=1 {
            for j in -1 as i32..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let first = if (y + i) < 0 {
                    self.grid.len() - 1
                } else if (y + i) as usize >= self.grid.len() {
                    0
                } else {
                    (y + i) as usize
                };
                let second = if (x + j) < 0 {
                    self.grid[0].len() - 1
                } else if (x + j) as usize >= self.grid[0].len() {
                    0
                } else {
                    (x + j) as usize
                };
                counter[self.state_types[self.grid[first][second]].index] += 1;
            }
        }
        counter
    }
}
