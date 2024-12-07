pub fn calculate(curr_val: i64, idx: usize, data: &Vec<i32>, expected_result: i64) -> bool {
    let mut ans = curr_val + data[idx] as i64;

    if idx + 1 < data.len() {
        let result = calculate(ans, idx + 1, data, expected_result);
        if result {
            return true;
        }
    } else if ans == expected_result {
        return true;
    }

    ans = curr_val * data[idx] as i64;
    if idx + 1 < data.len() {
        let result = calculate(ans, idx + 1, data, expected_result);
        if result {
            return true;
        }
    } else if ans == expected_result {
        return true;
    }

    ans = format!("{}{}", curr_val, data[idx]).parse::<i64>().unwrap();
    if idx + 1 < data.len() {
        let result = calculate(ans, idx + 1, data, expected_result);
        if result {
            return true;
        }
    } else if ans == expected_result {
        return true;
    }

    return false;
}

pub fn execute(data: &Vec<String>) {
    let mut total: i64 = 0;

    for line in data {
        let tokens: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
        let expected_result = tokens[0][0..tokens[0].len() - 1].parse::<i64>().unwrap();
        let mut data: Vec<i32> = Vec::new();

        for i in 1..tokens.len() {
            data.push(tokens[i].parse::<i32>().unwrap());
        }

        let result = calculate(data[0] as i64, 1, &data, expected_result);

        if result {
            total += expected_result;
        }
    }

    println!("Step 2 Total {}", total);
}
