const MAX_R_C: usize = 141;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Space {
    Wall,
    Space(usize),
}

pub fn distance(r1: i32, c1: i32, r2: i32, c2: i32) -> i32 {
    (r1 - r2).abs() + (c1 - c2).abs()
}

pub fn execute(data: &Vec<String>) {
    let mut maze = [[Space::Wall; MAX_R_C]; MAX_R_C];
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
    let mut path_points = Vec::<(i32, i32)>::new();
    loop {
        let (row, col) = current;
        path_points.push((row as i32, col as i32));

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

    let mut total: u32 = 0;

    for i in 0..path_points.len() - 1 {
        for j in i + 1..path_points.len() {
            let (r1, c1) = path_points[i];
            let (r2, c2) = path_points[j];

            let d = distance(r1, c1, r2, c2);

            if d <= 20 {
                if let Space::Space(start_val) = maze[r1 as usize][c1 as usize] {
                    if let Space::Space(target_val) = maze[r2 as usize][c2 as usize] {
                        if target_val > start_val && target_val - start_val - d as usize >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Step 2 Total {} ", total);
}
