use regex::Regex;

pub fn execute(data: &Vec<String>) {
    let re = Regex::new(r"XMAS").unwrap();
    let mut total: i32 = 0;

    let mut puzzle = [['.'; 140]; 140];
    let mut puzzle_rows: Vec<String> = Vec::new();

    let mut row: i32 = 0;
    let mut col: i32;

    for line in data {
        col = 0;
        for c in line.chars() {
            puzzle[row as usize][col as usize] = c;
            col += 1;
        }
        row += 1;
    }

    row = 0;

    while row < 140 {
        let mut puzzle_row: String = String::new();
        col = 0;

        while col < 140 {
            puzzle_row.push(puzzle[row as usize][col as usize]);
            col += 1;
        }

        puzzle_rows.push(puzzle_row.clone());
        puzzle_rows.push(puzzle_row.clone().chars().rev().collect::<String>());
        row += 1;
    }

    col = 0;

    while col < 140 {
        let mut puzzle_row: String = String::new();
        row = 0;

        while row < 140 {
            puzzle_row.push(puzzle[row as usize][col as usize]);
            row += 1;
        }

        puzzle_rows.push(puzzle_row.clone());
        puzzle_rows.push(puzzle_row.clone().chars().rev().collect::<String>());
        col += 1;
    }

    row = 139;
    col = 0;

    while col < 140 && row >= 0 {
        let mut puzzle_row: String = String::new();
        let mut r = row;
        let mut c = col;

        while r < 140 && c < 140 {
            puzzle_row.push(puzzle[r as usize][c as usize]);

            r += 1;
            c += 1;
        }

        puzzle_rows.push(puzzle_row.clone());
        puzzle_rows.push(puzzle_row.clone().chars().rev().collect::<String>());

        if row != 0 {
            row -= 1;
        } else {
            col += 1;
        }
    }

    row = 0;
    col = 0;

    while col < 140 && row < 140 {
        let mut puzzle_row: String = String::new();
        let mut r = row;
        let mut c = col;

        while r < 140 && c >= 0 {
            puzzle_row.push(puzzle[r as usize][c as usize]);

            r += 1;
            c -= 1;
        }

        puzzle_rows.push(puzzle_row.clone());
        puzzle_rows.push(puzzle_row.clone().chars().rev().collect::<String>());

        if col != 139 {
            col += 1;
        } else {
            row += 1;
        }
    }

    for puzzle_row in puzzle_rows {
        let counts = re.find_iter(&puzzle_row).count();
        total += counts as i32;
    }

    println!("Step 1 Total {}", total);
}
