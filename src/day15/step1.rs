use std::i32;

const MAX_R_C: usize = 50;

fn move_dir(
    curr_row: i32,
    curr_col: i32,
    dir_row: i32,
    dir_col: i32,
    board: &mut [[char; MAX_R_C]; MAX_R_C],
) -> bool {
    if board[curr_row as usize][curr_col as usize] == '#' {
        return false;
    }

    if board[curr_row as usize][curr_col as usize] == '.' {
        return true;
    }

    let should_move = move_dir(
        curr_row + dir_row,
        curr_col + dir_col,
        dir_row,
        dir_col,
        board,
    );

    if should_move {
        let next_piece = board[(curr_row + dir_row) as usize][(curr_col + dir_col) as usize];
        board[(curr_row + dir_row) as usize][(curr_col + dir_col) as usize] =
            board[curr_row as usize][curr_col as usize];
        board[curr_row as usize][curr_col as usize] = next_piece;
    }

    return should_move;
}

pub fn execute(data: &Vec<String>) {
    let mut board = [['.'; MAX_R_C]; MAX_R_C];
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
                        let should_move = move_dir(curr_row, curr_col, -1, 0, &mut board);
                        if should_move {
                            curr_row = curr_row - 1
                        }
                    }
                    '<' => {
                        let should_move = move_dir(curr_row, curr_col, 0, -1, &mut board);
                        if should_move {
                            curr_col = curr_col - 1
                        }
                    }
                    'v' => {
                        let should_move = move_dir(curr_row, curr_col, 1, 0, &mut board);
                        if should_move {
                            curr_row = curr_row + 1
                        }
                    }
                    '>' => {
                        let should_move = move_dir(curr_row, curr_col, 0, 1, &mut board);
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
                if space == '@' {
                    curr_row = r;
                    curr_col = c;
                }

                board[r as usize][c as usize] = space;
                c += 1;
            }
            r += 1;
        }
    }

    for r in 0..MAX_R_C {
        for c in 0..MAX_R_C {
            if board[r][c] == 'O' {
                total += 100 * r + c;
            }
        }
    }

    println!("Step 1 Total {}", total);
}
