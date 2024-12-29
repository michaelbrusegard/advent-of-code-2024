use advent_of_code_2024::read_lines;

fn main() {
    let (list1, list2) = parse_input("day01/input.txt");
    let total_distance = calculate_distance(&list1, &list2);
    println!("Total distance: {}", total_distance);
}

fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in read_lines(filename).expect("Failed to read file") {
        if let Some((num1, num2)) = parse_line(&line) {
            list1.push(num1);
            list2.push(num2);
        }
    }

    list1.sort();
    list2.sort();
    (list1, list2)
}

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let mut numbers = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    match (numbers.next(), numbers.next()) {
        (Some(n1), Some(n2)) => Some((n1, n2)),
        _ => None,
    }
}

fn calculate_distance(list1: &[i32], list2: &[i32]) -> i32 {
    let mut total_distance = 0;
    for i in 0..list1.len() {
        let num1 = list1[i];
        let num2 = list2[i];
        let distance = (num1 - num2).abs();
        total_distance += distance;
    }
    total_distance
}
