use advent_of_code_2024::{parse_to_lines, Timer};

fn main() {
    let (mut list1, list2) = parse_to_two_number_lists("day01/input.txt");
    {
        let _timer = Timer::default();
        let total_distance = calculate_distance(&list1, &list2);
        println!("Total distance: {}", total_distance);
    }
    {
        let _timer = Timer::default();
        list1.dedup();
        let similarity_score = calculate_similarity(&list1, &list2);
        println!("Similarity score: {}", similarity_score);
    }
}

fn parse_to_two_number_lists(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in parse_to_lines(filename) {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if let (Some(&num1), Some(&num2)) = (numbers.first(), numbers.get(1)) {
            list1.push(num1);
            list2.push(num2);
        }
    }

    list1.sort();
    list2.sort();
    (list1, list2)
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

fn calculate_similarity(list1: &[i32], list2: &[i32]) -> i32 {
    let mut total_similarity = 0;
    for num in list1 {
        let count: i32 = list2
            .iter()
            .filter(|&&x| x == *num)
            .count()
            .try_into()
            .unwrap_or(0);
        total_similarity += num * count;
    }
    total_similarity
}
