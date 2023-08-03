#[derive(Debug, Clone, Copy, PartialEq)]
pub enum States {
    Dead,
    Alive,
}
pub struct Rule {
    count: i32,
    next_state_index: usize,
}
pub struct Board {
    states: u32,
    colors: Vec<String>,
    rules: Vec<Rule>,
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
