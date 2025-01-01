use advent_of_code_2024::{parse_to_lines, Timer};

fn main() {
    let (mut rules, updates) = parse_rules_and_updates("day05/input.txt");
    {
        let _timer = Timer::default();
        let correctly_ordered_updates_sum = 0;
        println!(
            "Correctly ordered updates middle sum: {}",
            correctly_ordered_updates_sum
        );
    }
}

fn parse_rules_and_updates(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut parse_rules = true;

    let lines = parse_to_lines(filename);

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                if let (Ok(first), Ok(second)) = (parts[0].parse(), parts[1].parse()) {
                    rules.push((first, second));
                }
            }
        } else {
            let parts: Vec<&str> = line.split(',').collect();
            updates.push(parts.iter().filter_map(|s| s.parse().ok()).collect());
        }
    }

    (rules, updates)
}
