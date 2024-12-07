use regex::Regex;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // Construct the path to the input file relative to the source file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day3/part2/input.txt");

    // Open the input file
    let mut file = File::open(&path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Input read from file: {}", input);

    // Regex to match valid mul(X,Y) patterns where X and Y are 1-3 digit numbers
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // Regex to match do() and don't() instructions
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    
    let mut total_sum = 0;
    let mut mul_enabled = true; // At the beginning, mul instructions are enabled

    // Scan through the input
    let mut pos = 0;
    // check the input string for either a do or a don't instruction
    while pos < input.len() {
        // Check for "do()" instruction
        if let Some(mat) = re_do.find(&input[pos..]) {
            mul_enabled = true;
            pos += mat.end();
            continue;
        }

        // Check for "don't()" instruction
        if let Some(mat) = re_dont.find(&input[pos..]) {
            mul_enabled = false;
            pos += mat.end();
            continue;
        }

        // Execute the multiplication on each string till you reach a "don't" instruction
        if mul_enabled {
            if let Some(mat) = re_mul.find(&input[pos..]) {
                let cap = re_mul.captures(&input[pos..]).unwrap();
                let x: i32 = cap[1].parse().unwrap();
                let y: i32 = cap[2].parse().unwrap();
                total_sum += x * y;
                pos += mat.end();
            } else {
                break;
            }
        } else {
            pos += 1;
        }
    }
    
  
        
}