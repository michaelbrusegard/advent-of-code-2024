use advent_of_code_2024::{read_to_string, Timer};
use regex::Regex;

fn main() {
    let string = read_to_string("day03/input.txt").expect("Failed to read file");
    {
        let _timer = Timer::default();
        let operations = get_operations(&string);
        let result = calculate_result(operations);
        println!("Operations result: {}", result);
    }
    {
        let _timer = Timer::default();
        let operations = get_operations_with_statements(&string);
        let result = calculate_result(operations);
        println!("Operations with statements result: {}", result);
    }
}

fn get_operations(string: &str) -> Vec<(i32, i32)> {
    let mut operations = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(string) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        operations.push((x, y));
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

fn get_operations_with_statements(string: &str) -> Vec<(i32, i32)> {
    let mut operations = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mut processing = true;

    for cap in re.captures_iter(string) {
        let matched = cap.get(0).unwrap().as_str();
        if dont_re.is_match(matched) {
            processing = false;
        } else if do_re.is_match(matched) {
            processing = true;
        } else if mul_re.is_match(matched) && processing {
            let x: i32 = mul_re.captures(matched).unwrap()[1].parse().unwrap();
            let y: i32 = mul_re.captures(matched).unwrap()[2].parse().unwrap();
            operations.push((x, y));
        }
    }

    operations
}
