enum Direction {
    None,
    Asc,
    Desc,
}

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;

    'line_loop: for line in data {
        let elements: Vec<&str> = line.split(' ').collect();

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
                        continue 'line_loop;
                    }
                }
                Direction::Desc => {
                    if num >= prev_num {
                        continue 'line_loop;
                    }
                }
                Direction::None => {
                    if num > prev_num {
                        dir = Direction::Asc;
                    } else if num < prev_num {
                        dir = Direction::Desc;
                    } else {
                        continue 'line_loop;
                    }
                }
            }

            if (num - prev_num).abs() > 3 {
                continue 'line_loop;
            }

            prev_num = num;
        }

        total += 1;
    }

    println!("Step 1 Total {}", total);
}
