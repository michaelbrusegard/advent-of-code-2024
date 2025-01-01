use advent_of_code_2024::{parse_to_lines, Timer};

fn main() {
    let (rules, updates) = parse_rules_and_updates("day05/input.txt");
    {
        let _timer = Timer::default();
        let correctly_ordered_updates_sum =
            calculate_correctly_ordered_updates_sum(&rules, &updates);
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

fn calculate_correctly_ordered_updates_sum(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i32 {
    let mut sum = 0;

    for update in updates {
        let mut valid_rules = Vec::new();
        for rule in rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                valid_rules.push(rule);
            }
        }

        if valid_rules.is_empty() {
            sum += update[update.len() / 2];
            continue;
        }

        let mut valid = true;
        for (i, &current) in update.iter().enumerate() {
            for rule in &valid_rules {
                if rule.1 == current && !update.iter().take(i).any(|&page| page == rule.0) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            sum += update[update.len() / 2];
        }
    }

    sum
}
