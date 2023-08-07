use std::fmt::Debug;
use std::rc::Rc;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
    Dead,
    Alive,
}

#[derive(Clone)]
struct Rule {
    target_state: usize,
    predicate: Rc<dyn Fn(u32) -> bool>,
}

#[derive(Clone)]
struct State {
    index: usize,
    color: String,
    fail_state: usize,
    rules: Vec<Rule>,
}
pub struct Board {
    grid: Vec<Vec<State>>,
    state_types: Vec<State>,
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
    fn new(index: usize, color: String, fail_state: usize, rules: Vec<Rule>) -> Self {
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
            0,
            String::from("#000000"),
            0,
            vec![Rule::new(
                1,
                Rc::new(|count| if count == 3 { true } else { false }),
            )],
        );

        let state_1 = State::new(
            1,
            String::from("#FFFFFF"),
            0,
            vec![Rule::new(
                1,
                Rc::new(|count| {
                    if count == 2 || count == 3 {
                        true
                    } else {
                        false
                    }
                }),
            )],
        );
        let state_types = vec![state_0.clone(), state_1];
        Board {
            grid: vec![vec![state_0.clone(); 1024]; 512],
            state_types,
        }
    }

    pub fn next(&mut self) {
        let mut next_gen = self.grid.clone();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let neighbors = self.count_neighbors(x as i32, y as i32);
                let mut found = false;
                for rule in self.grid[y][x].clone().rules {
                    let output = (rule.predicate)(neighbors[rule.target_state]);
                    if output {
                        next_gen[y][x] = self.state_types[rule.target_state].clone();
                        found = true;
                        break;
                    }
                }
                if !found {
                    next_gen[y][x] = self.state_types[self.grid[y][x].clone().fail_state].clone();
                }
            }
        }
        self.grid = next_gen;
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
                match self.grid[first][second].clone() {
                    x => {
                        counter[x.index] += 1;
                    }
                }
            }
        }
        counter
    }
}

pub fn next(board: &Vec<Vec<States>>) -> Vec<Vec<States>> {
    let mut next_gen: Vec<Vec<States>> = board.clone();
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            let neighbors = count_neighbors(&board, &(x as i32), &(y as i32));
            if let States::Alive = board[y][x] {
                if neighbors == 2 || neighbors == 3 {
                    next_gen[y][x] = States::Alive;
                    continue;
                }
            } else if let States::Dead = board[y][x] {
                if neighbors == 3 {
                    next_gen[y][x] = States::Alive;
                    continue;
                }
            }

            next_gen[y][x] = States::Dead;
        }
    }
    next_gen
}

// count the number of neighbors to a cell with wrap-around logic
fn count_neighbors(board: &Vec<Vec<States>>, x: &i32, y: &i32) -> i32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let first = if (y + i) < 0 {
                board.len() - 1
            } else if (y + i) as usize >= board.len() {
                0
            } else {
                (y + i) as usize
            };
            let second = if (x + j) < 0 {
                board[0].len() - 1
            } else if (x + j) as usize >= board[0].len() {
                0
            } else {
                (x + j) as usize
            };
            match board[first][second] {
                States::Alive => {
                    count += 1;
                }
                States::Dead => {}
            }
        }
    }
    count -= if let States::Alive = board[*y as usize][*x as usize] {
        1
    } else {
        0
    };
    count
}
