const MAX_R_C: usize = 141;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Space {
    Wall,
    Space(usize),
}

pub fn execute(data: &Vec<String>) {
    let mut maze = [[Space::Wall; MAX_R_C]; MAX_R_C];
    let mut total: i32 = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut row = 0;
    for line in data {
        let mut col = 0;

        for c in line.chars() {
            match c {
                '.' => maze[row][col] = Space::Space(0),
                'S' => {
                    maze[row][col] = Space::Space(0);
                    start = (row, col);
                }
                'E' => {
                    maze[row][col] = Space::Space(0);
                    end = (row, col);
                }
                _ => {}
            }

            col += 1;
        }

        row += 1;
    }

    let mut current = start;
    let mut steps = 0;
    loop {
        let (row, col) = current;
        maze[row][col] = Space::Space(steps);

        if current == end {
            break;
        }

        steps += 1;

        if row != 0 && (row - 1, col) != start && maze[row - 1][col] == Space::Space(0) {
            current = (row - 1, col);
        } else if row < MAX_R_C - 1 && (row + 1, col) != start && maze[row + 1][col] == Space::Space(0) {
            current = (row + 1, col);
        } else if col != 0 && (row, col - 1) != start && maze[row][col - 1] == Space::Space(0) {
            current = (row, col - 1);
        } else if col < MAX_R_C + 1 && (row, col + 1) != start && maze[row][col + 1] == Space::Space(0) {
            current = (row, col + 1);
        }
    }

    for r in 1..MAX_R_C - 2 {
        for c in 1..MAX_R_C - 4 {
            if let Space::Space(i) = maze[r][c] {
                if maze[r][c + 1] == Space::Wall {
                    if let Space::Space(j) = maze[r][c + 2] {
                        let diff = (i as i32 - j as i32).abs();

                        if diff >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    for r in 1..MAX_R_C - 4 {
        for c in 1..MAX_R_C - 2 {
            if let Space::Space(i) = maze[r][c] {
                if maze[r + 1][c] == Space::Wall {
                    if let Space::Space(j) = maze[r + 2][c] {
                        let diff = (i as i32 - j as i32).abs();

                        if diff >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Step 1 Total {}", total);
}
