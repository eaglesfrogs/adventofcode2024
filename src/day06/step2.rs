const ROWS_COLS: usize = 130;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Self::Up => Direction::Right,
            Self::Right => Direction::Down,
            Self::Down => Direction::Left,
            Self::Left => Direction::Up,
        }
    }

    fn next_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        let (r, c) = pos;

        match self {
            Self::Up => (r - 1, c),
            Self::Right => (r, c + 1),
            Self::Down => (r + 1, c),
            Self::Left => (r, c - 1),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [['.'; ROWS_COLS]; ROWS_COLS];
    let mut total: i32 = 0;

    let mut row: usize = 0;
    let mut col: usize;

    let mut position: (i32, i32) = (0, 0);
    let direction: Direction = Direction::Up;

    for line in data {
        col = 0;

        for c in line.chars() {
            if c == '^' {
                position = (row as i32, col as i32);
            }

            puzzle[row][col] = c;
            col += 1;
        }
        row += 1;
    }

    for i in 0..ROWS_COLS {
        for j in 0..ROWS_COLS {
            if puzzle[i][j] == '#' || puzzle[i][j] == '^' {
                continue;
            }

            let mut puzzle_clone = puzzle.clone();
            let mut direction_clone = direction.clone();
            let mut position_clone = position.clone();
            puzzle_clone[i][j] = '#';

            loop {
                let next_pos = direction_clone.next_pos(position_clone);
                let (row, col) = next_pos;

                if row < 0 || row >= ROWS_COLS as i32 || col < 0 || col >= ROWS_COLS as i32 {
                    break;
                }

                if direction_clone.to_char() == puzzle_clone[row as usize][col as usize] {
                    total += 1;
                    break;
                }

                if puzzle_clone[row as usize][col as usize] == '#' {
                    direction_clone = direction_clone.turn()
                } else {
                    puzzle_clone[row as usize][col as usize] = direction_clone.to_char();
                    position_clone = next_pos;
                }
            }
        }
    }

    println!("Step 2 Total {}", total);
}
