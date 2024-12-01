pub fn execute(data: &Vec<String>) {
    let mut total: i32 = 0;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in data {
        let first_num_string = &line[0..5];
        let second_num_string = &line[8..13];
        let first_num = first_num_string.parse::<i32>().unwrap();
        let second_num = second_num_string.parse::<i32>().unwrap();

        left_list.push(first_num);
        right_list.push(second_num);
    }

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        let first_num = left_list[i];
        let second_num = right_list[i];

        total = &total + (first_num - second_num).abs();
    }

    println!("Step 1 Total {}", total);
}
