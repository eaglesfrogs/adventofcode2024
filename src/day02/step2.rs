enum Direction {
    None,
    Asc,
    Desc,
}

pub fn test(elements: &Vec<&str>) -> bool {
    let mut prev_num = -1;
    let mut dir = Direction::None;

    for e in elements {
        let num = e.parse::<i32>().unwrap();
        if prev_num == -1 {
            prev_num = num;
            continue;
        }

        match dir {
            Direction::Asc => {
                if num <= prev_num {
                    return false;
                }
            }
            Direction::Desc => {
                if num >= prev_num {
                    return false;
                }
            }
            Direction::None => {
                if num > prev_num {
                    dir = Direction::Asc;
                } else if num < prev_num {
                    dir = Direction::Desc;
                } else {
                    return false;
                }
            }
        }

        if (num - prev_num).abs() > 3 {
            return false;
        }

        prev_num = num;
    }

    true
}

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;

    for line in data {
        let elements: Vec<&str> = line.split(' ').collect();
        let vec_size = elements.len();

        let mut valid = test(&elements);
        let mut idx = 0;

        while valid == false && idx < vec_size {
            let mut edited_vector = elements.to_vec();
            edited_vector.remove(idx);

            valid = test(&edited_vector);
            idx += 1;
        }

        if valid {
            total += 1;
        }
    }

    println!("Step 1 Total {}", total);
}
