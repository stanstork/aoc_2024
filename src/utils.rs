use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
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

pub fn in_bounds<T>(matrix: &Vec<Vec<T>>, r: isize, c: isize) -> bool {
    r >= 0 && (r as usize) < matrix.len() && c >= 0 && (c as usize) < matrix[0].len()
}

pub fn on_edge<T>(matrix: &Vec<Vec<T>>, row: isize, col: isize) -> bool {
    row == 0 || row == matrix.len() as isize - 1 || col == 0 || col == matrix[0].len() as isize - 1
}

pub fn split_lines_whitespace(file_path: &str) -> (Vec<String>, Vec<String>) {
    let lines = read_lines(file_path);
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut first_done = false;
    for line in lines {
        if line.is_empty() {
            first_done = true;
            continue;
        }
        if !first_done {
            first.push(line);
        } else {
            second.push(line);
        }
    }
    (first, second)
}
