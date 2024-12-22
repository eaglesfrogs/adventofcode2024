pub fn next_number(num: i64) -> i64 {
    let step1mul = num * 64;
    let step1mix = step1mul ^ num;
    let step1prune = step1mix % 16777216;

    let step2mul = step1prune / 32;
    let step2mix = step2mul ^ step1prune;
    let step2prune = step2mix % 16777216;

    let step3mul = step2prune * 2048;
    let step3mix = step3mul ^ step2prune;
    let step3prune = step3mix % 16777216;

    step3prune
}

pub fn execute(data: &Vec<String>) {
    let mut total: i64 = 0;

    for line in data {
        let num = line.parse::<i64>().unwrap();

        let mut next_num = num;

        for _ in 0..2000 {
            next_num = next_number(next_num);
        }

        total += next_num;
    }

    println!("Step 1 Total {}", total);
}
