use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    // Construct the path to the input file relative to the source file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day2/part1/input.txt");
    // Open the input file
    let file = File::open(&path).expect("Cannot open file");
    let reader = BufReader::new(file);

    // Initialize vectors for the reports
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // Read lines from the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|x| x.parse().expect("Unable to parse numbers"))
                                    .collect();
        reports.push(numbers);
    }

    // Function to check if a report is safe
    fn is_safe(report: &Vec<i32>) -> bool {
        if report.len() < 2 {
            return false;
        }

        let mut increasing = true;
        let mut decreasing = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i] - report[i + 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
            if report[i] < report[i + 1] {
                decreasing = false;
            }
            if report[i] > report[i + 1] {
                increasing = false;
            }
        }

        increasing || decreasing
    }

    // Count the number of safe reports
    let safe_count = reports.iter().filter(|&report| is_safe(report)).count();

    // Print the result
    println!("Number of safe reports: {}", safe_count);
}