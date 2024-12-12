use cached::proc_macro::cached;

const MAX_ITERATIONS: u32 = 75;

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

#[cached]
fn formulate(stone: u64, iteration: u32) -> u64 {
    if iteration == MAX_ITERATIONS {
        return 1;
    }

    let mut total: u64 = 0;

    if stone == 0 {
        total += formulate(1 as u64, iteration + 1);
    } else {
        let num_digits = get_num_digits(stone);

        if num_digits % 2 == 0 {
            let tens = (10 as u64).pow((num_digits / 2) as u32);
            let left: u64 = stone / tens;
            let right: u64 = stone % tens;

            total += formulate(left, iteration + 1);
            total += formulate(right, iteration + 1);
        } else {
            total += formulate(stone * 2024, iteration + 1);
        }
    }

    return total;
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

    let mut total: u64 = 0;

    for stone in stones {
        total += formulate(stone, 0);
    }

    println!("Step 2 Total {}", total);
}
