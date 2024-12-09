#[derive(Clone)]
enum DiskEntry {
    File { id: u32, length: u32 },
    FreeSpace { length: u32 },
}

pub fn execute(data: &Vec<String>) {
    let mut disk: Vec<DiskEntry> = Vec::new();

    let mut total: i64 = 0;
    let mut is_file = true;
    let mut file_num = 0;

    for line in data {
        for size_c in line.chars() {
            let size: u32 = size_c.to_digit(10).unwrap();
            if is_file {
                disk.push(DiskEntry::File {
                    id: file_num,
                    length: size,
                });
                file_num += 1;
            } else {
                disk.push(DiskEntry::FreeSpace { length: size });
            }

            is_file = !is_file;
        }
    }

    let mut right_idx: usize = disk.len() - 1;

    let mut idx_map: [usize; 10] = [0; 10];

    'outer: while right_idx > 0 {
        let right_disk_entry: DiskEntry;
        let right_length: u32;
        let mut size_idx: usize;
        let empty_length;

        loop {
            match disk[right_idx] {
                DiskEntry::FreeSpace { length: _ } => right_idx -= 1,
                DiskEntry::File { id: _, length } => {
                    right_disk_entry = disk[right_idx].clone();
                    right_length = length;
                    size_idx = idx_map[length as usize];
                    break;
                }
            }
        }

        loop {
            if size_idx >= right_idx {
                right_idx -= 1;
                continue 'outer;
            }

            match disk[size_idx] {
                DiskEntry::File { id: _, length: _ } => {
                    size_idx += 1;
                    idx_map[right_length as usize] = size_idx;
                }
                DiskEntry::FreeSpace { length } => {
                    if length >= right_length {
                        empty_length = length;
                        break;
                    } else {
                        size_idx += 1;
                        idx_map[right_length as usize] = size_idx;
                    }
                }
            }
        }

        disk[right_idx] = DiskEntry::FreeSpace {
            length: right_length,
        };

        disk[size_idx] = DiskEntry::FreeSpace {
            length: empty_length - right_length,
        };
        disk.insert(size_idx, right_disk_entry);
    }

    let mut idx = 0;
    for d in disk {
        match d {
            DiskEntry::File { id, length } => {
                for _i in 0..length {
                    total += idx as i64 * id as i64;
                    idx += 1;
                }
            }
            DiskEntry::FreeSpace { length } => {
                idx += length;
            }
        }
    }

    println!("Step 2 Total {}", total);
}
