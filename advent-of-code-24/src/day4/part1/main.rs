use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let grid = read_grid_from_file("src/day4/part1/input.txt")?;
    let word = "XMAS";
    let count = count_word_occurrences(&grid, word);
    println!("The word '{}' appears {} times.", word, count);
    Ok(())
}

// Function to read the grid from a file
fn read_grid_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(grid)
}

// Function to count the occurrences of a word in the grid
fn count_word_occurrences(grid: &[String], word: &str) -> usize {
    let directions = vec![
        (0, 1),  // right
        (1, 0),  // down
        (1, 1),  // down-right
        (1, -1), // down-left
        (0, -1), // left
        (-1, 0), // up
        (-1, -1),// up-left
        (-1, 1), // up-right
    ];

    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(grid, word, i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

// Function to check if the word exists starting from (x, y) in direction (dx, dy)
fn check_word(grid: &[String], word: &str, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let word_len = word.len() as isize;

    for k in 0..word_len {
        let nx = x + k * dx;
        let ny = y + k * dy;

        if nx < 0 || nx >= rows || ny < 0 || ny >= cols {
            return false;
        }

        if grid[nx as usize].chars().nth(ny as usize).unwrap() != word.chars().nth(k as usize).unwrap() {
            return false;
        }
    }

    true
}