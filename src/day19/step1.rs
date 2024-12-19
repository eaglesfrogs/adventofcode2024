use std::collections::HashSet;

pub fn match_pattern(pattern: &str, longest_towel: usize, towel_set: &HashSet<&str>) -> bool {
    let mut window = 1;
    let mut matched = false;

    while window <= longest_towel && window <= pattern.len() {
        let chunk = &pattern[0..window];
        if towel_set.contains(chunk) {
            if chunk == pattern {
                return true;
            }

            matched = match_pattern(&pattern[window..], longest_towel, towel_set);

            if matched {
                break;
            }
        }

        window += 1;
    }

    return matched;
}

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;
    let mut towel_set = HashSet::<&str>::new();
    let mut longest_towel = 0;

    let mut parse_displays = false;

    for line in data {
        if parse_displays {
            let matched = match_pattern(line, longest_towel, &towel_set);

            if matched {
                total += 1;
            }
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
