use std::collections::HashSet;

const MAX_ROW: usize = 50;
const MAX_COL: usize = 50 * 2;

fn move_cols(
    curr_row: i32,
    curr_col: i32,
    dir_col: i32,
    board: &mut [[char; MAX_COL]; MAX_ROW],
) -> bool {
    if board[curr_row as usize][curr_col as usize] == '#' {
        return false;
    }

    if board[curr_row as usize][curr_col as usize] == '.' {
        return true;
    }

    let should_move = move_cols(curr_row, curr_col + dir_col, dir_col, board);

    if should_move {
        let next_piece = board[(curr_row) as usize][(curr_col + dir_col) as usize];
        board[(curr_row) as usize][(curr_col + dir_col) as usize] =
            board[curr_row as usize][curr_col as usize];
        board[curr_row as usize][curr_col as usize] = next_piece;
    }

    return should_move;
}

fn move_rows(
    points: &HashSet<(i32, i32)>,
    dir_row: i32,
    board: &mut [[char; MAX_COL]; MAX_ROW],
) -> bool {
    for p in points {
        let (r, c) = p.clone();
        if board[(r + dir_row) as usize][c as usize] == '#' {
            return false;
        }
    }

    let mut is_all_empty = true;
    for p in points {
        let (r, c) = p.clone();
        is_all_empty = is_all_empty && board[(r + dir_row) as usize][c as usize] == '.';
    }

    if is_all_empty {
        for p in points {
            let (r, c) = p.clone();

            let next_piece = board[(r + dir_row) as usize][c as usize];
            board[(r + dir_row) as usize][c as usize] = board[r as usize][c as usize];
            board[r as usize][c as usize] = next_piece;
        }
        return true;
    }

    let mut next_points = HashSet::<(i32, i32)>::new();
    for p in points {
        let (r, c) = p.clone();
        if board[r as usize][c as usize] == ']' {
            continue;
        } else if board[r as usize][c as usize] == '[' {
            if board[(r + dir_row) as usize][c as usize] == '[' {
                next_points.insert((r + dir_row, c));
                next_points.insert((r + dir_row, c + 1));
            } else if board[(r + dir_row) as usize][c as usize] == ']' {
                next_points.insert((r + dir_row, c));
                next_points.insert((r + dir_row, c - 1));
            }

            if board[(r + dir_row) as usize][(c + 1) as usize] == '[' {
                next_points.insert((r + dir_row, c + 1));
                next_points.insert((r + dir_row, c + 2));
            }
        } else {
            if board[(r + dir_row) as usize][c as usize] == ']' {
                next_points.insert((r + dir_row, c));
                next_points.insert((r + dir_row, c - 1));
            } else if board[(r + dir_row) as usize][c as usize] == '[' {
                next_points.insert((r + dir_row, c));
                next_points.insert((r + dir_row, c + 1));
            }
        }
    }

    let should_move = move_rows(&next_points, dir_row, board);

    if should_move {
        for p in points {
            let (r, c) = p.clone();

            let next_piece = board[(r + dir_row) as usize][c as usize];
            board[(r + dir_row) as usize][c as usize] = board[r as usize][c as usize];
            board[r as usize][c as usize] = next_piece;
        }
    }

    return should_move;
}

pub fn execute(data: &Vec<String>) {
    let mut board = [['.'; MAX_COL]; MAX_ROW];
    let mut total = 0;

    let mut process_path = false;
    let mut r = 0;
    let mut c;
    let mut curr_row = 0;
    let mut curr_col = 0;

    for line in data {
        if process_path {
            for direction in line.chars() {
                match direction {
                    '^' => {
                        let mut points = HashSet::<(i32, i32)>::new();
                        points.insert((curr_row, curr_col));
                        let should_move = move_rows(&points, -1, &mut board);
                        if should_move {
                            curr_row = curr_row - 1
                        }
                    }
                    '<' => {
                        let should_move = move_cols(curr_row, curr_col, -1, &mut board);
                        if should_move {
                            curr_col = curr_col - 1
                        }
                    }
                    'v' => {
                        let mut points = HashSet::<(i32, i32)>::new();
                        points.insert((curr_row, curr_col));
                        let should_move = move_rows(&points, 1, &mut board);
                        if should_move {
                            curr_row = curr_row + 1
                        }
                    }
                    '>' => {
                        let should_move = move_cols(curr_row, curr_col, 1, &mut board);
                        if should_move {
                            curr_col = curr_col + 1
                        }
                    }
                    _ => {}
                }
            }
        } else if line.len() == 0 {
            process_path = true;
        } else {
            c = 0;
            for space in line.chars() {
                match space {
                    '#' => {
                        board[r as usize][c as usize] = '#';
                        board[r as usize][c + 1 as usize] = '#';
                    }
                    '.' => {
                        board[r as usize][c as usize] = '.';
                        board[r as usize][c + 1 as usize] = '.';
                    }
                    'O' => {
                        board[r as usize][c as usize] = '[';
                        board[r as usize][c + 1 as usize] = ']';
                    }
                    '@' => {
                        board[r as usize][c as usize] = '@';
                        board[r as usize][c + 1 as usize] = '.';

                        curr_row = r;
                        curr_col = c as i32;
                    }
                    _ => {}
                }

                c += 2;
            }
            r += 1;
        }
    }

    for r in 0..MAX_ROW {
        for c in 0..MAX_COL {
            if board[r][c] == '[' {
                total += 100 * r + c;
            }
        }
    }

    println!("Step 2 Total {}", total);
}
