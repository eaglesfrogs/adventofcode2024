use regex::Regex;

pub fn execute(data: &Vec<String>) {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))()()|(don't\(\))()()").unwrap();
    let mut total: i32 = 0;

    let mut enabled = true;

    for line in data {
        for (_, [mul, d1, d2]) in re.captures_iter(line).map(|c| c.extract()) {
            // println!("{}", mul);

            if mul == "do()" {
                enabled = true;
            } else if mul == "don't()" {
                enabled = false;
            } else if enabled && mul.starts_with("mul") {
                total += d1.parse::<i32>().unwrap() * d2.parse::<i32>().unwrap();
            }
        }
    }

    println!("Step 1 Total {}", total);
}
