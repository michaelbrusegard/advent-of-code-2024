use advent_of_code_2024::{read_lines, Timer};
use regex::Regex;

fn main() {
    let strings = read_lines("day03/input.txt").expect("Failed to read file");
    {
        let _timer = Timer::default();
        let operations = get_operations(strings);
        let result = calculate_result(operations);
        println!("Operations result: {}", result);
    }
}

fn get_operations(strings: Vec<String>) -> Vec<(i32, i32)> {
    let mut operations = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for string in strings {
        for cap in re.captures_iter(&string) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            operations.push((x, y));
        }
    }
    operations
}

fn calculate_result(operations: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for (x, y) in operations {
        result += x * y;
    }

    result
}
