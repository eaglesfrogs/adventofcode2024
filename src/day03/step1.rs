use regex::Regex;

pub fn execute(data: &Vec<String>) {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut total: i32 = 0;

    for line in data {
        for (_, [_mul, d1, d2]) in re.captures_iter(line).map(|c| c.extract()) {
            total += d1.parse::<i32>().unwrap() * d2.parse::<i32>().unwrap();
        }
    }

    println!("Step 1 Total {}", total);
}
