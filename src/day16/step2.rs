use std::{cmp::min, i32};

const MAX_R_C: usize = 141;

#[derive(Debug, Clone, Copy)]
struct Space {
    val: i32,
    dir: char,
    cool: bool,
}

impl Space {
    fn new() -> Self {
        return Space {
            val: i32::MAX,
            dir: 'x',
            cool: false,
        };
    }

    fn set(val: i32, dir: char) -> Self {
        return Space {
            val: val,
            dir: dir,
            cool: false,
        };
    }
}

fn navigate_maze(row: usize, col: usize, board: &mut [[Option<Space>; MAX_R_C]; MAX_R_C]) -> i32 {
    let curr_space = board[row][col].unwrap();

    let mut min_val = i32::MAX;

    if curr_space.dir == '^' {
        let mut a = -1;
        let mut b = -1;
        let mut c = -1;
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 <= x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1, '^'));
                a = navigate_maze(row - 1, col, board);
                min_val = a;
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1001, '<'));
                b = navigate_maze(row, col - 1, board);
                min_val = min(min_val, b);
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1001, '>'));
                c = navigate_maze(row, col + 1, board);
                min_val = min(min_val, c);
            }
        }

        if a != i32::MAX && a == min_val {
            let mut ahhh = board[row - 1][col].unwrap();
            ahhh.cool = true;
            board[row - 1][col] = Some(ahhh);
        }

        if b != i32::MAX && b == min_val {
            let mut ahhh = board[row][col - 1].unwrap();
            ahhh.cool = true;
            board[row][col - 1] = Some(ahhh);
        }

        if c != i32::MAX && c == min_val {
            let mut ahhh = board[row][col + 1].unwrap();
            ahhh.cool = true;
            board[row][col + 1] = Some(ahhh);
        }
    } else if curr_space.dir == '<' {
        let mut a = -1;
        let mut b = -1;
        let mut c = -1;
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1001, '^'));
                a = navigate_maze(row - 1, col, board);
                min_val = a;
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 <= x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1, '<'));
                b = navigate_maze(row, col - 1, board);
                min_val = min(min_val, b);
            }
        }

        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1001, 'v'));
                c = navigate_maze(row + 1, col, board);
                min_val = min(min_val, c);
            }
        }

        if a != i32::MAX && a == min_val {
            let mut ahhh = board[row - 1][col].unwrap();
            ahhh.cool = true;
            board[row - 1][col] = Some(ahhh);
        }

        if b != i32::MAX && b == min_val {
            let mut ahhh = board[row][col - 1].unwrap();
            ahhh.cool = true;
            board[row][col - 1] = Some(ahhh);
        }

        if c != i32::MAX && c == min_val {
            let mut ahhh = board[row + 1][col].unwrap();
            ahhh.cool = true;
            board[row + 1][col] = Some(ahhh);
        }
    } else if curr_space.dir == '>' {
        let mut a = -1;
        let mut b = -1;
        let mut c = -1;
        if let Some(x) = board[row - 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row - 1][col] = Some(Space::set(curr_space.val + 1001, '^'));
                a = navigate_maze(row - 1, col, board);
                min_val = a;
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 <= x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1, '>'));
                b = navigate_maze(row, col + 1, board);
                min_val = min(min_val, b);
            }
        }

        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1001, 'v'));
                c = navigate_maze(row + 1, col, board);
                min_val = min(min_val, c);
            }
        }

        if a != i32::MAX && a == min_val {
            let mut ahhh = board[row - 1][col].unwrap();
            ahhh.cool = true;
            board[row - 1][col] = Some(ahhh);
        }

        if b != i32::MAX && b == min_val {
            let mut ahhh = board[row][col + 1].unwrap();
            ahhh.cool = true;
            board[row][col + 1] = Some(ahhh);
        }

        if c != i32::MAX && c == min_val {
            let mut ahhh = board[row + 1][col].unwrap();
            ahhh.cool = true;
            board[row + 1][col] = Some(ahhh);
        }
    } else if curr_space.dir == 'v' {
        let mut a = -1;
        let mut b = -1;
        let mut c = -1;
        if let Some(x) = board[row + 1][col] {
            if x.dir == 'E' {
                return curr_space.val + 1;
            }

            if curr_space.val + 1 <= x.val {
                board[row + 1][col] = Some(Space::set(curr_space.val + 1, 'v'));
                a = navigate_maze(row + 1, col, board);
                min_val = a;
            }
        }

        if let Some(x) = board[row][col - 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row][col - 1] = Some(Space::set(curr_space.val + 1001, '<'));
                b = navigate_maze(row, col - 1, board);
                min_val = min(min_val, b);
            }
        }

        if let Some(x) = board[row][col + 1] {
            if x.dir == 'E' {
                return curr_space.val + 1001;
            }

            if curr_space.val + 1001 <= x.val {
                board[row][col + 1] = Some(Space::set(curr_space.val + 1001, '>'));
                c = navigate_maze(row, col + 1, board);
                min_val = min(min_val, c);
            }
        }

        if a != i32::MAX && a == min_val {
            let mut ahhh = board[row + 1][col].unwrap();
            ahhh.cool = true;
            board[row + 1][col] = Some(ahhh);
        }

        if b != i32::MAX && b == min_val {
            let mut ahhh = board[row][col - 1].unwrap();
            ahhh.cool = true;
            board[row][col - 1] = Some(ahhh);
        }

        if c != i32::MAX && c == min_val {
            let mut ahhh = board[row][col + 1].unwrap();
            ahhh.cool = true;
            board[row][col + 1] = Some(ahhh);
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

    navigate_maze(starting_row, starting_col, &mut board);

    let mut total = 0;

    for i in 0..MAX_R_C {
        for j in 0..MAX_R_C {
            if let Some(x) = board[i][j] {
                if x.cool {
                    total += 1;
                }
            }
        }
    }

    println!("Step 2 Total {}", total);
}
