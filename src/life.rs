use std::fmt::Debug;
use std::rc::Rc;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
    Dead,
    Alive,
}

#[derive(Clone)]
pub struct Rule {
    pub target_state: usize,
    pub predicate: Rc<dyn Fn(u32) -> bool>,
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
    pub fn new(target_state: usize, predicate: Rc<dyn Fn(u32) -> bool>) -> Self {
        Rule {
            target_state,
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

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.index)?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        // By default, the board uses regular game of life rules.
        let state_0 = State::new(
            0,                       // index
            String::from("#000000"), // color
            0,                       // fail state
            vec![Rule::new(
                1,
                Rc::new(|count| if count == 3 { true } else { false }),
            )],
        );

        let state_1 = State::new(
            1,
            String::from("#FFFFFF"),
            0,
            vec![
                Rule::new(1, Rc::new(|count| if count == 2 { true } else { false })),
                Rule::new(1, Rc::new(|count| if count == 3 { true } else { false })),
            ],
        );
        let state_types = vec![state_0.clone(), state_1];
        Board {
            grid: vec![vec![0; 1024]; 512],
            state_types,
        }
    }

    pub fn next(&self) -> Vec<Vec<usize>> {
        let mut next_gen = vec![vec![0; 1024]; 512];
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let neighbors = self.count_neighbors(x as i32, y as i32);
                let mut found = false;
                for rule in self.state_types[self.grid[y][x]].clone().rules {
                    let output = (rule.predicate)(neighbors[rule.target_state]);
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
