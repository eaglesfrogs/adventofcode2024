use std::collections::{HashMap, HashSet};

pub fn execute(data: &Vec<String>) {
    let mut total: i64 = 0;
    let mut answer_map = HashMap::<String, i8>::new();
    let mut gate_vector = Vec::<(String, char, String, String)>::new();

    let mut read_values = true;

    for line in data {
        if read_values {
            if line.len() == 0 {
                read_values = false;
            } else {
                let (key, val) = line.split_once(": ").unwrap();
                answer_map.insert(String::from(key), val.parse::<i8>().unwrap());
            }
        } else {
            let mut tokens = line.split_ascii_whitespace();
            let t1 = String::from(tokens.next().unwrap());
            let t2 = tokens.next().unwrap();
            let t3 = String::from(tokens.next().unwrap());
            tokens.next().unwrap(); //Arrow
            let t4 = String::from(tokens.next().unwrap());
            let operator: char;

            if t2 == "AND" {
                operator = '&';
            } else if t2 == "OR" {
                operator = '|';
            } else {
                operator = '^';
            }

            gate_vector.push((t1, operator, t3, t4));
        }
    }

    let mut processed = HashSet::<(String, char, String, String)>::new();

    while processed.len() < gate_vector.len() {
        for gate in &gate_vector {
            if processed.contains(&gate) {
                continue;
            }

            let (i1, op, i2, o) = gate.clone();

            if answer_map.contains_key(&i1) && answer_map.contains_key(&i2) {
                let v1 = answer_map.get(&i1).unwrap().clone();
                let v2 = answer_map.get(&i2).unwrap().clone();
                let answer: i8;

                if op == '&' {
                    answer = v1 & v2;
                } else if op == '|' {
                    answer = v1 | v2;
                } else {
                    answer = v1 ^ v2;
                }

                answer_map.insert(o, answer);
                processed.insert(gate.clone());
            }
        }
    }

    let mut z_tokens = Vec::<String>::new();
    for key in answer_map.keys() {
        if key.starts_with("z") {
            z_tokens.push(key.clone());
        }
    }

    z_tokens.sort();

    let mut i = 0;
    for z_token in z_tokens {
        let val = answer_map.get(&z_token).unwrap().clone();

        total += val as i64 * (2 as i64).pow(i);

        i += 1;
    }

    println!("Step 1 Total {}", total);
}
