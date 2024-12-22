use std::{
    collections::{HashMap, HashSet},
    i8,
};

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
    let mut seq_map = HashMap::<(i8, i8, i8, i8), i64>::new();

    for line in data {
        let num = line.parse::<i64>().unwrap();

        let mut next_num = num;
        let mut s0: i8;
        let mut s1 = i8::MAX;
        let mut s2 = i8::MAX;
        let mut s3 = i8::MAX;

        let mut seen = HashSet::<(i8, i8, i8, i8)>::new();

        for i in 0..2000 {
            let prev_price = next_num % 10;
            next_num = next_number(next_num);

            let next_price = next_num % 10;
            let change = next_price as i8 - prev_price as i8;

            s0 = s1;
            s1 = s2;
            s2 = s3;
            s3 = change;

            if i > 2 {
                if seen.contains(&(s0, s1, s2, s3)) == false {
                    seen.insert((s0, s1, s2, s3));

                    let t = seq_map.entry((s0, s1, s2, s3)).or_insert(0);
                    *t += next_price;
                }
            }
        }
    }

    let mut best_sequence = (i8::MAX, i8::MAX, i8::MAX, i8::MAX);
    let mut best_price = 0;

    for (seq, total) in seq_map {
        if total > best_price {
            best_sequence = seq;
            best_price = total;
        }
    }

    println!("Step 2 Total {:?} {}", best_sequence, best_price);
}
