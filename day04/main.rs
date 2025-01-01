use advent_of_code_2024::{parse_to_char_grid, Timer};

fn main() {
    let char_grid = parse_to_char_grid("day04/input.txt");
    {
        let _timer = Timer::default();
        let xmas_occurences = find_xmas_occurences(&char_grid);
        println!("XMAS occurences: {}", xmas_occurences);
    }
    {
        let _timer = Timer::default();
        let x_mas_occurences = find_x_mas_occurences(&char_grid);
        println!("X-MAS occurences: {}", x_mas_occurences);
    }
}

fn find_xmas_occurences(char_grid: &[Vec<char>]) -> i32 {
    let mut xmas_occurences = 0;
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];
    let rows = char_grid.len();
    let cols = char_grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if char_grid[row][col] != 'X' {
                continue;
            }
            for &(dx, dy) in &directions {
                if pos_in_bounds(row as i32 + dx, col as i32 + dy, rows, cols)
                    && char_grid[(row as i32 + dx) as usize][(col as i32 + dy) as usize] == 'M'
                    && pos_in_bounds(row as i32 + dx * 2, col as i32 + dy * 2, rows, cols)
                    && char_grid[(row as i32 + dx * 2) as usize][(col as i32 + dy * 2) as usize]
                        == 'A'
                    && pos_in_bounds(row as i32 + dx * 3, col as i32 + dy * 3, rows, cols)
                    && char_grid[(row as i32 + dx * 3) as usize][(col as i32 + dy * 3) as usize]
                        == 'S'
                {
                    xmas_occurences += 1;
                }
            }
        }
    }

    xmas_occurences
}

fn pos_in_bounds(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
}

fn find_x_mas_occurences(char_grid: &[Vec<char>]) -> i32 {
    let mut x_mas_occurences = 0;
    let directions: [(i32, i32); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
    let rows = char_grid.len();
    let cols = char_grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if char_grid[row][col] != 'A' {
                continue;
            }
            let mut valid = true;
            for (dx, dy) in &directions {
                if !pos_in_bounds(row as i32 + dx, col as i32 + dy, rows, cols) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }
            if ((char_grid[(row as i32 + directions[0].0) as usize]
                [(col as i32 + directions[0].1) as usize]
                == 'M'
                && char_grid[(row as i32 + directions[1].0) as usize]
                    [(col as i32 + directions[1].1) as usize]
                    == 'S')
                || (char_grid[(row as i32 + directions[0].0) as usize]
                    [(col as i32 + directions[0].1) as usize]
                    == 'S'
                    && char_grid[(row as i32 + directions[1].0) as usize]
                        [(col as i32 + directions[1].1) as usize]
                        == 'M'))
                && ((char_grid[(row as i32 + directions[2].0) as usize]
                    [(col as i32 + directions[2].1) as usize]
                    == 'M'
                    && char_grid[(row as i32 + directions[3].0) as usize]
                        [(col as i32 + directions[3].1) as usize]
                        == 'S')
                    || (char_grid[(row as i32 + directions[2].0) as usize]
                        [(col as i32 + directions[2].1) as usize]
                        == 'S'
                        && char_grid[(row as i32 + directions[3].0) as usize]
                            [(col as i32 + directions[3].1) as usize]
                            == 'M'))
            {
                x_mas_occurences += 1;
            }
        }
    }

    x_mas_occurences
}
