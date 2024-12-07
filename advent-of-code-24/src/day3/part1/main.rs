use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // Construct the path to the input file relative to the source file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day3/part1/input.txt");

    // Open the input file
    let mut file = File::open(&path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    // Regex to match valid mul(X,Y) patterns where X and Y are 1-3 digit numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let mut total_sum = 0;

    // Find all matches in the input
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        total_sum += x * y;
    }

    println!("The total sum of all valid multiplications is: {}", total_sum);

    Ok(())
}