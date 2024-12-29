use advent_of_code_2024::read_lines;

fn main() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let lines = read_lines("day01/input.txt").expect("Failed to read file");
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(first), Some(second)) = (parts.first(), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    list1.sort();
    list2.sort();

    let mut total_distance = 0;
    for i in 0..list1.len() {
        let num1 = list1[i];
        let num2 = list2[i];
        let distance = (num1 - num2).abs();
        total_distance += distance;
    }

    println!("Total distance: {}", total_distance);
}
