use std::fs;

pub fn read_file(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).expect("Failed to read file");

    let mut lines = Vec::new();

    for line in contents.lines() {
        lines.push(line.to_string());
    }

    lines
}
