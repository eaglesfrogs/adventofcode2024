use std::{cmp::Ordering, collections::HashMap};

pub fn execute(data: &Vec<String>) {
    let mut order_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut total: i32 = 0;

    let mut populate_ordered_map = true;

    for line in data {
        if line.trim().len() == 0 {
            populate_ordered_map = false;
        } else if populate_ordered_map {
            let tokens: Vec<&str> = line.split('|').collect();
            let num1 = tokens[0].parse::<i32>().unwrap();
            let num2 = tokens[1].parse::<i32>().unwrap();

            order_map.entry(num1).or_insert(Vec::new()).push(num2);
        } else {
            let mut tokens: Vec<i32> = line.split(',').map(|t| t.parse::<i32>().unwrap()).collect();

            let mut valid = true;

            for i in 1..tokens.len() {
                let order = order_map.get(&tokens[i]);

                match order {
                    Some(o) => {
                        for j in 0..i {
                            if o.contains(&tokens[j]) {
                                valid = false;
                            }
                        }
                    }
                    None => continue,
                }
            }

            if valid == false {
                tokens.sort_by(|a, b| {
                    let order = order_map.get(a);

                    match order {
                        Some(o) => {
                            if o.contains(b) {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        }
                        None => Ordering::Less,
                    }
                });

                let mid_idx: usize = tokens.len() as usize / 2;
                let mid_digit: i32 = tokens[mid_idx];
                total += mid_digit;
            }
        }
    }

    println!("Step 2 Total {}", total);
}
