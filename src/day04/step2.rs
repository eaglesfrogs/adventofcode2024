pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;

    let mut puzzle = [['.'; 140]; 140];

    let mut row: i32 = 0;
    let mut col: i32 = 0;

    for line in data {
        col = 0;
        for c in line.chars() {
            puzzle[row as usize][col as usize] = c;
            col += 1;
        }
        row += 1;
    }

    for r in 1..139 {
        for c in 1..139 {
            let a_char = puzzle[r][c];

            if a_char == 'A' {
                if (puzzle[r - 1][c - 1] == 'M'
                    && puzzle[r - 1][c + 1] == 'M'
                    && puzzle[r + 1][c - 1] == 'S'
                    && puzzle[r + 1][c + 1] == 'S')
                    || (puzzle[r - 1][c - 1] == 'S'
                        && puzzle[r - 1][c + 1] == 'S'
                        && puzzle[r + 1][c - 1] == 'M'
                        && puzzle[r + 1][c + 1] == 'M')
                    || (puzzle[r - 1][c - 1] == 'M'
                        && puzzle[r - 1][c + 1] == 'S'
                        && puzzle[r + 1][c - 1] == 'M'
                        && puzzle[r + 1][c + 1] == 'S')
                    || (puzzle[r - 1][c - 1] == 'S'
                        && puzzle[r - 1][c + 1] == 'M'
                        && puzzle[r + 1][c - 1] == 'S'
                        && puzzle[r + 1][c + 1] == 'M')
                {
                    total += 1;
                }
            }
        }
    }

    println!("Step 1 Total {}", total);
}
