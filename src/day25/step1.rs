#[derive(PartialEq)]
enum KeyOrLock {
    Key,
    Lock,
    Unknown,
}

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;
    let mut locks = Vec::<(i8, i8, i8, i8, i8)>::new();
    let mut keys = Vec::<(i8, i8, i8, i8, i8)>::new();

    let mut key_or_lock = KeyOrLock::Unknown;
    let mut schematic = [['.'; 5]; 7];

    let mut row: usize = 0;
    for line in data {
        if line.len() == 0 {
            calculate_locks(&mut locks, &mut keys, &key_or_lock, schematic);

            key_or_lock = KeyOrLock::Unknown;
            schematic = [['.'; 5]; 7];
            row = 0;
        } else {
            if row == 0 {
                if line.starts_with("#") {
                    key_or_lock = KeyOrLock::Lock;
                } else {
                    key_or_lock = KeyOrLock::Key;
                }
            }

            let mut col = 0;
            for c in line.chars() {
                schematic[row][col] = c;
                col += 1;
            }

            row += 1;
        }
    }

    calculate_locks(&mut locks, &mut keys, &key_or_lock, schematic);

    for (k1, k2, k3, k4, k5) in &keys {
        for (l1, l2, l3, l4, l5) in &locks {
            if *l1 + *k1 <= 5 && *l2 + *k2 <= 5 && *l3 + *k3 <= 5 && *l4 + *k4 <= 5 && *l5 + *k5 <= 5 {
                total += 1;
            }
        }
    }

    println!("Step 1 Total {}", total);
}

fn calculate_locks(locks: &mut Vec<(i8, i8, i8, i8, i8)>, keys: &mut Vec<(i8, i8, i8, i8, i8)>, key_or_lock: &KeyOrLock, schematic: [[char; 5]; 7]) {
    let mut t1 = -1;
    let mut t2 = -1;
    let mut t3 = -1;
    let mut t4 = -1;
    let mut t5 = -1;

    for row in schematic {
        if row[0] == '#' {
            t1 += 1;
        }
        if row[1] == '#' {
            t2 += 1;
        }
        if row[2] == '#' {
            t3 += 1;
        }
        if row[3] == '#' {
            t4 += 1;
        }
        if row[4] == '#' {
            t5 += 1;
        }
    }

    if *key_or_lock == KeyOrLock::Key {
        keys.push((t1, t2, t3, t4, t5));
    } else {
        locks.push((t1, t2, t3, t4, t5));
    }
}
