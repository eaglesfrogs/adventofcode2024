const ROWS_COLS: usize = 130;

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
}

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [['.'; ROWS_COLS]; ROWS_COLS];
    let mut total: i32 = 0;

    let mut row: usize = 0;
    let mut col: usize;

    let mut position: (i32, i32) = (0, 0);
    let mut direction: Direction = Direction::Up;

    for line in data {
        col = 0;

        for c in line.chars() {
            if c == '^' {
                position = (row as i32, col as i32);
                puzzle[row][col] = 'X';
            } else {
                puzzle[row][col] = c;
            }
            col += 1;
        }
        row += 1;
    }

    loop {
        let next_pos = direction.next_pos(position);
        let (row, col) = next_pos;

        if row < 0 || row >= ROWS_COLS as i32 || col < 0 || col >= ROWS_COLS as i32 {
            break;
        }

        if puzzle[row as usize][col as usize] == '#' {
            direction = direction.turn()
        } else {
            puzzle[row as usize][col as usize] = 'X';
            position = next_pos;
        }
    }

    for i in 0..ROWS_COLS {
        for j in 0..ROWS_COLS {
            if puzzle[i][j] == 'X' {
                total += 1;
            }
        }
    }

    println!("Step 1 Total {}", total);
}
