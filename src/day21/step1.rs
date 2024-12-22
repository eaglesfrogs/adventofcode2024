use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::{cmp::min, collections::HashMap, i32, usize};

lazy_static! {
    static ref NUMERIC: HashMap<char, Vec<(char, char)>> = {
        let mut m = HashMap::new();
        m.insert('A', vec![('0', '<'), ('3', '^')]);
        m.insert('0', vec![('2', '^'), ('A', '>')]);
        m.insert('1', vec![('4', '^'), ('2', '>')]);
        m.insert('2', vec![('1', '<'), ('5', '^'), ('3', '>'), ('0', 'v')]);
        m.insert('3', vec![('2', '<'), ('6', '^'), ('A', 'v')]);
        m.insert('4', vec![('7', '^'), ('1', 'v'), ('5', '>')]);
        m.insert('5', vec![('4', '<'), ('8', '^'), ('6', '>'), ('2', 'v')]);
        m.insert('6', vec![('9', '^'), ('3', 'v'), ('5', '<')]);
        m.insert('7', vec![('4', 'v'), ('8', '>')]);
        m.insert('8', vec![('7', '<'), ('5', 'v'), ('9', '>')]);
        m.insert('9', vec![('8', '<'), ('6', 'v')]);

        m
    };
    static ref DIRECTIONAL: HashMap<char, Vec<(char, char)>> = {
        let mut m = HashMap::new();
        m.insert('A', vec![('^', '<'), ('>', 'v')]);
        m.insert('^', vec![('A', '>'), ('v', 'v')]);
        m.insert('<', vec![('v', '>')]);
        m.insert('v', vec![('<', '<'), ('^', '^'), ('>', '>')]);
        m.insert('>', vec![('v', '<'), ('A', '^')]);

        m
    };
}

#[cached]
pub fn find_shortest_numeric(curr: char, target: char, visited: Vec<char>) -> Vec<String> {
    let mut visited = visited.clone();
    visited.push(curr);

    let mut results = Vec::<String>::new();
    let options = NUMERIC.get(&curr).unwrap();

    let mut shortest_len = usize::MAX;

    for o in options {
        let (key, dir) = o;

        if visited.contains(key) {
            continue;
        }

        if *key == target {
            return vec![format!("{}A", dir)];
        }

        let resp = find_shortest_numeric(key.clone(), target, visited.clone());

        for r in resp {
            let s = format!("{}{}", dir, r);

            if s.len() < shortest_len {
                shortest_len = s.len();
            }

            results.push(s);
        }
    }

    let mut final_results = Vec::<String>::new();
    for r in results {
        if r.len() == shortest_len {
            final_results.push(r);
        }
    }

    return final_results;
}

pub fn process_path(path_candidates: Vec<String>, depth: i8) -> Vec<String> {
    let mut total_paths = Vec::<String>::new();
    for pc in path_candidates {
        let mut curr_char = 'A';
        let mut paths = Vec::<String>::new();
        for c in pc.chars() {
            let res = find_shortest_directional(curr_char, c, depth, Vec::<char>::new());

            if paths.len() == 0 {
                paths = res;
            } else {
                let mut new_paths = Vec::<String>::new();
                for p1 in paths {
                    for r in &res {
                        new_paths.push(format!("{}{}", p1, r));
                    }
                }
                paths = new_paths;
            }

            curr_char = c;
        }

        total_paths.append(&mut paths);
    }

    let mut min_length = usize::MAX;
    let mut final_results = Vec::<String>::new();
    for p in &total_paths {
        min_length = min(min_length, p.len());
    }

    for p in &total_paths {
        if p.len() == min_length {
            final_results.push(p.clone());
        }
    }

    return final_results;
}

#[cached]
pub fn find_shortest_directional(curr: char, target: char, depth: i8, visited: Vec<char>) -> Vec<String> {
    if curr == target {
        return vec![String::from("A")];
    }

    let mut visited = visited.clone();
    visited.push(curr);

    let mut results = Vec::<String>::new();
    let options = DIRECTIONAL.get(&curr).unwrap();

    let mut shortest_len = usize::MAX;

    for o in options {
        let (key, dir) = o;

        if visited.contains(key) {
            continue;
        }

        if *key == target {
            return vec![format!("{}A", dir)];
        }

        let resp = find_shortest_directional(key.clone(), target, depth, visited.clone());

        for r in resp {
            let s = format!("{}{}", dir, r);

            if s.len() < shortest_len {
                shortest_len = s.len();
            }

            results.push(s);
        }
    }

    let mut final_results = Vec::<String>::new();
    for r in results {
        if r.len() == shortest_len {
            final_results.push(r);
        }
    }

    return final_results;
}

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;

    for line in data {
        let mut paths = Vec::<String>::new();
        let mut curr_char = 'A';
        for c in line.chars() {
            let res = find_shortest_numeric(curr_char, c, Vec::<char>::new());

            if paths.len() == 0 {
                paths = res;
            } else {
                let mut new_paths = Vec::<String>::new();
                for p1 in paths {
                    for r in &res {
                        new_paths.push(format!("{}{}", p1, r));
                    }
                }
                paths = new_paths;
            }

            curr_char = c;
        }

        let paths = process_path(paths, 2);
        let paths = process_path(paths, 1);

        total += paths[0].len() as i32 * line[0..line.len() - 1].parse::<i32>().unwrap();
    }

    println!("Step 1 Total {}", total);
}
