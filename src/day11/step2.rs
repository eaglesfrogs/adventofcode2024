fn get_num_digits(mut num: u64) -> i32 {
    if num == 0 {
        return 1;
    }

    let mut count = 0;

    while num != 0 {
        num = num / 10;
        count += 1;
    }

    count
}

pub fn execute(data: &Vec<String>) {
    let mut stones: Vec<u64> = Vec::<u64>::new();

    for line in data {
        let elements: Vec<&str> = line.split(' ').collect();
        for e in elements {
            let num = e.parse::<u64>().unwrap();
            stones.push(num);
        }
    }

    for _i in 0..75 {
        let mut next_stones = Vec::<u64>::new();

        for stone in stones {
            if stone == 0 {
                next_stones.push(1);
            } else {
                let num_digits = get_num_digits(stone);

                if num_digits % 2 == 0 {
                    let tens = (10 as u64).pow((num_digits / 2) as u32);
                    let left: u64 = stone / tens;
                    let right: u64 = stone % tens;

                    next_stones.push(left);
                    next_stones.push(right);
                } else {
                    next_stones.push(stone * 2024);
                }
            }
        }

        println!("{}", next_stones.len());

        stones = next_stones;
    }

    println!("Step 2 Total {}", stones.len());
}
