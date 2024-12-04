fn main() {
    let grid = vec![
        "MMMSXXMASM".to_string(),
        "MSAMXMSMSA".to_string(),
        "AMXSXMAAMM".to_string(),
        "MSAMASMSMX".to_string(),
        "XMASAMXAMM".to_string(),
        "XXAMMXXAMA".to_string(),
        "SMSMSASXSS".to_string(),
        "SAXAMASAAA".to_string(),
        "MAMMMXMMMM".to_string(),
        "MXMXAXMASX".to_string(),
    ];

    let count = count_xmas_occurrences(&grid);
    println!("Number of X-MAS patterns: {}", count);
}

fn count_xmas_occurrences(grid: &[String]) -> usize {
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..rows {
        for j in 0..cols {
            if check_xmas(grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn check_xmas(grid: &[String], row: isize, col: isize) -> bool {
    let patterns = [
        ["M", "A", "S", "A", "M"], // Forward X-MAS
        ["S", "A", "M", "A", "S"], // Backward X-MAS
    ];

    for pattern in &patterns {
        if check_pattern(grid, row, col, pattern) {
            return true;
        }
    }

    false
}

fn check_pattern(grid: &[String], row: isize, col: isize, pattern: &[&str]) -> bool {
    let positions = [
        (0, 0), (1, -1), (2, -2), (1, 1), (2, 2), // X-MAS pattern positions
    ];

    for (i, &(dx, dy)) in positions.iter().enumerate() {
        let new_row = row + dx;
        let new_col = col + dy;

        if new_row < 0 || new_row >= grid.len() as isize || new_col < 0 || new_col >= grid[0].len() as isize {
            return false;
        }

        if grid[new_row as usize].chars().nth(new_col as usize).unwrap() != pattern[i].chars().next().unwrap() {
            return false;
        }
    }

    true
}