use std::{cmp::min, i32};

const MAX_R_C: usize = 141;

#[derive(Debug, Clone, Copy)]
struct Space {
    val: i32,
    dir: char,
}

impl Space {
    fn new() -> Self {
        return Space {
            val: i32::MAX,
            dir: 'x',
        };
    }

    fn set(val: i32, dir: char) -> Self {
        return Space { val: val, dir: dir };
    }
}

fn navigate_maze(row: usize, col: usize, board: &mut [[Option<Space>; MAX_R_C]; MAX_R_C]) -> i32 {
    let curr_space = board[row][col].unwrap();

    let mut min_val = i32::MAX;

    if curr_space.dir == '^' {
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 < x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1, '^'));
                min_val = navigate_maze(row - 1, col, board);
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1001, '<'));
                min_val = min(min_val, navigate_maze(row, col - 1, board));
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1001, '>'));
                min_val = min(min_val, navigate_maze(row, col + 1, board));
            }
        }
    } else if curr_space.dir == '<' {
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1001, '^'));
                min_val = navigate_maze(row - 1, col, board);
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 < x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1, '<'));
                min_val = min(min_val, navigate_maze(row, col - 1, board));
            }
        }

        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1001, 'v'));
                min_val = min(min_val, navigate_maze(row + 1, col, board));
            }
        }
    } else if curr_space.dir == '>' {
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1001, '^'));
                min_val = navigate_maze(row - 1, col, board);
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 < x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1, '>'));
                min_val = min(min_val, navigate_maze(row, col + 1, board));
            }
        }

        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1001, 'v'));
                min_val = min(min_val, navigate_maze(row + 1, col, board));
            }
        }
    } else if curr_space.dir == 'v' {
        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 < x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1, 'v'));
                min_val = navigate_maze(row + 1, col, board);
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1001, '<'));
                min_val = min(min_val, navigate_maze(row, col - 1, board));
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 < x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1001, '>'));
                min_val = min(min_val, navigate_maze(row, col + 1, board));
            }
        }
    }

    return min_val;
}

pub fn execute(data: &Vec<String>) {
    let mut board: [[Option<Space>; MAX_R_C]; MAX_R_C] = [[None; MAX_R_C]; MAX_R_C];

    let mut r = 0;
    let mut c;
    let mut starting_row: usize = 0;
    let mut starting_col: usize = 0;

    for line in data {
        c = 0;
        for space in line.chars() {
            if space == 'S' {
                starting_row = r;
                starting_col = c;
                board[starting_row][starting_col] = Some(Space::set(0, '>'));
            } else if space == 'E' {
                board[r as usize][c as usize] = Some(Space::set(i32::MAX, 'E'));
            } else if space != '#' {
                board[r as usize][c as usize] = Some(Space::new());
            }

            c += 1;
        }
        r += 1;
    }

    let total = navigate_maze(starting_row, starting_col, &mut board);

    println!("Step 1 Total {}", total);
}
