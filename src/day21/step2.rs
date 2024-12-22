use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::{cmp::min, collections::HashMap, i32, i64};

pub enum Keypads {
    Directional,
    Numeric,
}

lazy_static! {
    static ref NUMERIC_COORDS: HashMap<char, (i32, i32)> = {
        let mut m = HashMap::new();
        m.insert('A', (3, 2));
        m.insert('0', (3, 1));
        m.insert('X', (3, 0));
        m.insert('1', (2, 0));
        m.insert('2', (2, 1));
        m.insert('3', (2, 2));
        m.insert('4', (1, 0));
        m.insert('5', (1, 1));
        m.insert('6', (1, 2));
        m.insert('7', (0, 0));
        m.insert('8', (0, 1));
        m.insert('9', (0, 2));
        m
    };
    static ref DIRECTIONAL_COORDS: HashMap<char, (i32, i32)> = {
        let mut m = HashMap::new();
        m.insert('<', (1, 0));
        m.insert('v', (1, 1));
        m.insert('>', (1, 2));
        m.insert('X', (0, 0));
        m.insert('^', (0, 1));
        m.insert('A', (0, 2));
        m
    };
}

//Previous methods included doing directions like ^>^ which always results in more moves upstream
//So lets limit it to moves like ^^> or >^^
pub fn find_shortest(curr: char, target: char, keypad: &Keypads) -> Vec<String> {
    if curr == target {
        return vec![String::from("A")];
    }

    let (r1, c1): (i32, i32);
    let (r2, c2): (i32, i32);
    let (bad_r, bad_c): (i32, i32);

    match keypad {
        Keypads::Directional => {
            (r1, c1) = DIRECTIONAL_COORDS.get(&curr).unwrap().clone();
            (r2, c2) = DIRECTIONAL_COORDS.get(&target).unwrap().clone();
            (bad_r, bad_c) = DIRECTIONAL_COORDS.get(&'X').unwrap().clone();
        }
        Keypads::Numeric => {
            (r1, c1) = NUMERIC_COORDS.get(&curr).unwrap().clone();
            (r2, c2) = NUMERIC_COORDS.get(&target).unwrap().clone();
            (bad_r, bad_c) = NUMERIC_COORDS.get(&'X').unwrap().clone();
        }
    }

    let diff_r = r1 - r2;
    let diff_c = c1 - c2;
    let r_moves: String;
    let c_moves: String;

    if diff_r > 0 {
        r_moves = (0..diff_r).map(|_| "^").collect::<String>();
    } else if diff_r < 0 {
        r_moves = (0..diff_r.abs()).map(|_| "v").collect::<String>();
    } else {
        r_moves = String::new();
    }

    if diff_c > 0 {
        c_moves = (0..diff_c).map(|_| "<").collect::<String>();
    } else if diff_c < 0 {
        c_moves = (0..diff_c.abs()).map(|_| ">").collect::<String>();
    } else {
        c_moves = String::new();
    }

    if diff_r == 0 {
        return vec![format!("{}A", c_moves)];
    }
    if diff_c == 0 {
        return vec![format!("{}A", r_moves)];
    }
    if r1 == bad_r && c2 == bad_c {
        return vec![format!("{}{}A", r_moves, c_moves)];
    }
    if r2 == bad_r && c1 == bad_c {
        return vec![format!("{}{}A", c_moves, r_moves)];
    }

    vec![format!("{}{}A", r_moves, c_moves), format!("{}{}A", c_moves, r_moves)]
}

#[cached]
pub fn process_paths(code: String, run: i32, max_runs: i32) -> i64 {
    if run == max_runs {
        return code.len() as i64;
    }

    let keypad;
    if run == 0 {
        keypad = Keypads::Numeric;
    } else {
        keypad = Keypads::Directional;
    }

    let mut current = 'A';
    let mut total: i64 = 0;
    for key in code.chars() {
        let shortest_paths = find_shortest(current, key, &keypad);

        let mut min_result = i64::MAX;
        for shortest_path in shortest_paths {
            min_result = min(min_result, process_paths(shortest_path, run + 1, max_runs));
        }
        total += min_result;

        current = key;
    }

    return total;
}

pub fn execute(data: &Vec<String>) {
    let mut total: i64 = 0;

    for line in data {
        let path_length = process_paths(line.clone(), 0, 26);

        total += path_length * line[0..line.len() - 1].parse::<i64>().unwrap();
    }

    println!("Step 1 Total {}", total);
}
