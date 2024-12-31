use advent_of_code_2024::{parse_to_char_grid, Timer};

fn main() {
    let char_grid = parse_to_char_grid("day04/input.txt");
    {
        let _timer = Timer::default();
        let xmas_occurences = find_xmas_occurences(&char_grid);
        println!("XMAS occurences: {}", xmas_occurences);
    }
}

fn find_xmas_occurences(char_grid: &Vec<Vec<char>>) -> i32 {}
