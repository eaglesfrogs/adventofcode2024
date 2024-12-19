use std::collections::{HashMap, HashSet};

struct TowelProcessor<'a> {
    longest_towel: usize,
    towel_set: HashSet<&'a str>,
    towel_cache: HashMap<&'a str, i64>,
}

impl<'a> TowelProcessor<'a> {
    pub fn new(longest_towel: usize, towel_set: HashSet<&'a str>) -> Self {
        TowelProcessor {
            longest_towel: longest_towel,
            towel_set: towel_set,
            towel_cache: HashMap::new(),
        }
    }

    pub fn match_pattern(&mut self, pattern: &'a str) -> i64 {
        let mut window = 1;
        let mut matched_total = 0;

        while window <= self.longest_towel && window <= pattern.len() {
            let chunk = &pattern[0..window];
            if self.towel_set.contains(chunk) {
                if chunk == pattern {
                    return matched_total + 1;
                }

                if window == pattern.len() {
                    return 0;
                }

                let next_pattern = &pattern[window..];

                if self.towel_cache.contains_key(next_pattern) {
                    matched_total += self.towel_cache.get(next_pattern).unwrap().clone();
                } else {
                    let r = self.match_pattern(next_pattern);
                    self.towel_cache.insert(next_pattern, r);
                    matched_total += r;
                }
            }

            window += 1;
        }

        return matched_total;
    }
}

pub fn execute(data: &Vec<String>) {
    let mut total: i64 = 0;
    let mut towel_set = HashSet::<&str>::new();
    let mut longest_towel = 0;

    let mut parse_displays = false;

    for line in data {
        if parse_displays {
            let mut tp = TowelProcessor::new(longest_towel, towel_set.clone());
            total += &tp.match_pattern(line);
        } else if line.len() == 0 {
            parse_displays = true;
        } else {
            let patterns: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            for p in patterns {
                towel_set.insert(p);

                if p.len() > longest_towel {
                    longest_towel = p.len();
                }
            }
        }
    }

    println!("Step 1 Total {}", total);
}
