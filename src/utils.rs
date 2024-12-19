use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

pub fn read_matrix(file_path: &str) -> Vec<Vec<char>> {
    read_lines(file_path)
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn read_num_matrix(file_path: &str) -> Vec<Vec<i32>> {
    read_lines(file_path)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}