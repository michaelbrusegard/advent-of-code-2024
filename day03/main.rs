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
        println!("Operations result: {}", result);
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

fn get_operations_with_statements(input: &str) -> Vec<(i32, i32)> {
    let mut operations = Vec::new();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mut processing = true;
    let mut current_pos = 0;

    while current_pos < input.len() {
        let remaining = &input[current_pos..];

        if let Some(do_match) = do_re.find(remaining) {
            processing = true;
            current_pos += do_match.end();
        } else if let Some(dont_match) = dont_re.find(remaining) {
            processing = false;
            current_pos += dont_match.end();
        } else if let Some(mul_match) = mul_re.find(remaining) {
            if processing {
                if let Some(cap) = mul_re.captures(&remaining[mul_match.start()..mul_match.end()]) {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    operations.push((x, y));
                }
            }
            current_pos += mul_match.end();
        } else {
            current_pos += 1;
        }
    }

    operations
}
