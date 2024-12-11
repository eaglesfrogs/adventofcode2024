const ROWS_COLS: usize = 45;

fn count_paths(point: (usize, usize), puzzle: &[[usize; 45]; 45]) -> usize {
    let (x, y) = point;
    let val = puzzle[x][y];
    let mut count = 0;

    if val == 9 {
        return 1;
    }

    if x > 0 && val + 1 == puzzle[x - 1][y] {
        count += count_paths((x - 1, y), puzzle);
    }

    if y > 0 && val + 1 == puzzle[x][y - 1] {
        count += count_paths((x, y - 1), puzzle);
    }

    if x < ROWS_COLS - 1 && val + 1 == puzzle[x + 1][y] {
        count += count_paths((x + 1, y), puzzle);
    }

    if y < ROWS_COLS - 1 && val + 1 == puzzle[x][y + 1] {
        count += count_paths((x, y + 1), puzzle);
    }

    count
}

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [[0; ROWS_COLS]; ROWS_COLS];
    let mut total: usize = 0;

    let mut starting_points = Vec::<(usize, usize)>::new();

    let mut i: usize = 0;

    for line in data {
        let mut j: usize = 0;

        for c in line.chars() {
            let d: u32 = c.to_digit(10).unwrap();
            puzzle[i][j] = d as usize;

            if d == 0 {
                starting_points.push((i, j));
            }

            j += 1;
        }
        i += 1;
    }

    while starting_points.len() > 0 {
        let starting_point = starting_points.pop().unwrap();

        total += count_paths(starting_point, &puzzle);
    }

    println!("Step 1 Total {}", total);
}
