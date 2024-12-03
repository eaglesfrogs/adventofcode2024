use std::collections::HashMap;

pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list_map: HashMap<i32, i32> = HashMap::new();

    for line in data {
        let first_num_string = &line[0..5];
        let second_num_string = &line[8..13];
        let first_num = first_num_string.parse::<i32>().unwrap();
        let second_num = second_num_string.parse::<i32>().unwrap();

        left_list.push(first_num);

        let num_count = right_list_map.entry(second_num).or_insert(0);
        *num_count += 1;
    }

    for i in 0..left_list.len() {
        let first_num = left_list[i];
        let right_list_count = right_list_map.get(&first_num).copied().unwrap_or(0);

        total = &total + &first_num * right_list_count;
    }

    println!("Step 2 Total {}", total);
}
