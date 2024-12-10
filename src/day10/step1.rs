const ROWS_COLS: usize = 45;

pub fn execute(data: &Vec<String>) {
    let mut puzzle = [[0; ROWS_COLS]; ROWS_COLS];
    let mut total: i32 = 0;

    let mut starting_points = Vec::<(u32, u32)>::new();

    let mut i: u32 = 0;

    for line in data {
        let mut j: u32 = 0;

        for c in line.chars() {
            let d: u32 = c.to_digit(10).unwrap();
            puzzle[i][j] = d;

            if d == 0 {
                starting_points.push((i, j));
            }

            j += 1;
        }
        i += 1;
    }

    println!("Step 1 Total {}", total);
}
