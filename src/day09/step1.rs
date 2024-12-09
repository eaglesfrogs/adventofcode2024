pub fn execute(data: &Vec<String>) {
    let mut disk: Vec<i32> = Vec::new();

    let mut total: i64 = 0;
    let mut is_file = true;
    let mut file_num = 0;

    for line in data {
        for size_c in line.chars() {
            let size: u32 = size_c.to_digit(10).unwrap();
            if is_file {
                for _i in 0..size {
                    disk.push(file_num);
                }

                file_num += 1;
            } else {
                for _i in 0..size {
                    disk.push(-1);
                }
            }

            is_file = !is_file;
        }
    }

    let mut left_idx: usize = 0;
    let mut right_idx: usize = disk.len() - 1;

    while left_idx < right_idx {
        while disk[left_idx] != -1 {
            left_idx += 1;
        }

        while disk[right_idx] == -1 {
            right_idx -= 1;
        }

        if left_idx < right_idx {
            disk[left_idx] = disk[right_idx];
            disk[right_idx] = -1;
            left_idx += 1;
            right_idx -= 1;
        }
    }

    for i in 0..disk.len() {
        if disk[i] != -1 {
            total += i as i64 * disk[i] as i64;
        }
    }

    println!("Step 1 Total {}", total);
}
