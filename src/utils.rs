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