use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
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

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub fn read_to_string(filename: &str) -> io::Result<String> {
    std::fs::read_to_string(filename)
}

pub fn parse_numbers<T>(line: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
