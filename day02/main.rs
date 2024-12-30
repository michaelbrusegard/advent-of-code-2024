use advent_of_code_2024::{read_lines, Timer};

fn main() {
    let reports = parse_input("day02/input.txt");
    {
        let _timer = Timer::default();
        let safe_reports = calculate_safe_reports(&reports);
        println!("Safe reports: {}", safe_reports);
    }
    {
        let _timer = Timer::default();
        let safe_reports_with_dampener = calculate_safe_reports_with_dampener(&reports);
        println!("Safe reports with dampener: {}", safe_reports_with_dampener);
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

fn is_sequence_safe(sequence: &[i32]) -> bool {
    if sequence.len() < 2 {
        return false;
    }

    let mut increasing = None;

    for i in 1..sequence.len() {
        let diff = sequence[i] - sequence[i - 1];
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }

        match increasing {
            None => increasing = Some(diff > 0),
            Some(is_increasing) => {
                if (is_increasing && diff <= 0) || (!is_increasing && diff >= 0) {
                    return false;
                }
            }
        }
    }
    true
}

fn calculate_safe_reports(reports: &[Vec<i32>]) -> i32 {
    let mut total_safe_reports = 0;
    for report in reports {
        if is_sequence_safe(report) {
            total_safe_reports += 1;
        }
    }
    total_safe_reports
}

fn calculate_safe_reports_with_dampener(reports: &[Vec<i32>]) -> i32 {
    let mut total_safe_reports = 0;
    for report in reports {
        if is_sequence_safe(report) {
            total_safe_reports += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut modified_report = report.to_vec();
            modified_report.remove(i);
            if is_sequence_safe(&modified_report) {
                total_safe_reports += 1;
                break;
            }
        }
    }
    total_safe_reports
}
