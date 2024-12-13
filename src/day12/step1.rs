const ROWS_COLS: usize = 140;

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [['.'; ROWS_COLS]; ROWS_COLS];
    let mut total: u32 = 0;

    let mut i = 0;
    for line in data {
        let mut j = 0;

        for c in line.chars() {
            puzzle[i][j] = c;

            j += 1;
        }

        i += 1;
    }

    let mut queue: Vec<(usize, usize)> = Vec::new();

    for i in 0..ROWS_COLS {
        for j in 0..ROWS_COLS {
            let entry = puzzle[i][j];

            if entry.is_ascii_lowercase() {
                continue;
            }

            queue.push((i, j));

            let mut area = 0;
            let mut perimeter = 0;

            while queue.is_empty() == false {
                let (x, y) = queue.pop().unwrap();

                if puzzle[x][y].is_ascii_lowercase() {
                    continue;
                }

                area += 1;
                puzzle[x][y] = entry.to_ascii_lowercase();

                if x == 0 || puzzle[x - 1][y] != entry {
                    if x == 0 || puzzle[x - 1][y] != entry.to_ascii_lowercase() {
                        perimeter += 1;
                    }
                } else {
                    queue.push((x - 1, y));
                }

                if y == 0 || puzzle[x][y - 1] != entry {
                    if y == 0 || puzzle[x][y - 1] != entry.to_ascii_lowercase() {
                        perimeter += 1;
                    }
                } else {
                    queue.push((x, y - 1));
                }

                if x == ROWS_COLS - 1 || puzzle[x + 1][y] != entry {
                    if x == ROWS_COLS - 1 || puzzle[x + 1][y] != entry.to_ascii_lowercase() {
                        perimeter += 1;
                    }
                } else {
                    queue.push((x + 1, y));
                }

                if y == ROWS_COLS - 1 || puzzle[x][y + 1] != entry {
                    if y == ROWS_COLS - 1 || puzzle[x][y + 1] != entry.to_ascii_lowercase() {
                        perimeter += 1;
                    }
                } else {
                    queue.push((x, y + 1));
                }
            }

            total += area * perimeter;
        }
    }

    println!("Step 1 Total {}", total);
}
