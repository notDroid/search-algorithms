use std::fs;

pub fn test_file_iterator(file_path: &str) -> Vec<(String, i32)> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            (parts[0].to_string(), parts[1].parse().unwrap())
        })
        .collect()
}
