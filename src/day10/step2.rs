const ROWS_COLS: usize = 45;

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [[0; ROWS_COLS]; ROWS_COLS];
    let mut total: i32 = 0;

    let mut i = 0;

    for line in data {
        let mut j = 0;

        for c in line.chars() {
            let d: u32 = c.to_digit(10).unwrap();
            puzzle[i][j] = d;

            j += 1;
        }
        i += 1;
    }

    println!("Step 1 Total {}", total);
}
