use advent_of_code_2024::{read_lines, Timer};

fn main() {
    let reports = parse_input("day02/input.txt");
    {
        let _timer = Timer::default();
        let safe_reports = calculate_safe_reports(&reports);
        println!("Safe reports: {}", safe_reports);
    }
}

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let mut list = Vec::new();

    for line in read_lines(filename).expect("Failed to read file") {
        list.push(parse_line(&line));
    }

    list
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate_safe_reports(reports: &[Vec<i32>]) -> i32 {
    let mut total_safe_reports = 0;
    for report in reports {
        if report.len() < 2 {
            continue;
        }

        let mut is_valid = true;
        let mut increasing = None;

        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            if diff.abs() > 3 || diff.abs() < 1 {
                is_valid = false;
                break;
            }

            match increasing {
                None => increasing = Some(diff > 0),
                Some(is_increasing) => {
                    if (is_increasing && diff <= 0) || (!is_increasing && diff >= 0) {
                        is_valid = false;
                        break;
                    }
                }
            }
        }

        if is_valid {
            total_safe_reports += 1;
        }
    }
    total_safe_reports
}
