use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("Time: {:?}", elapsed);
    }
}

pub fn parse_to_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed to read line"));
    }

    lines
}

pub fn parse_to_number_grid(filename: &str) -> Vec<Vec<i32>> {
    parse_to_lines(filename)
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect()
}

pub fn parse_to_string(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Failed to read file to string")
}

pub fn parse_to_char_grid(filename: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(filename).expect("Failed to read file");
    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
