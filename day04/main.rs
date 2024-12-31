use advent_of_code_2024::{read_lines, Timer};

fn main() {
    let lines = parse_input("day04/input.txt");
    {
        let _timer = Timer::default();
        let xmas_occurences = find_xmas_occurences(&lines);
        println!("XMAS occurences: {}", xmas_occurences);
    }
}

fn find_xmas_occurences(lines: &[String]) -> i32 {

}
