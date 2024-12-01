use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    // Construct the path to the input file relative to the source file
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/day1/part2/input.txt");
    // Open the input file
    let file = File::open(&path).expect("Cannot open file");
    let reader = BufReader::new(file);

    // Initialize vectors for the two lists
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    // Read lines from the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|x| x.parse().expect("Unable to parse numbers"))
                                    .collect();

        if numbers.len() == 2 {
            vec1.push(numbers[0]);
            vec2.push(numbers[1]);
        } else {
            eprintln!("Each line must contain exactly two numbers");
            std::process::exit(1);
        }
    }
    
    // take an element from vec1 and multiply it by the number of similar element in vec2
    // then sum the result to get the total similarity score
    let total_similarity_score: i32 = vec1.iter()
                        .map(|x| vec2.iter().filter(|&y| x == y).count() as i32 * x)
                        .sum();


    // Print the result
    println!("Total similarity score between the list is: {}", total_similarity_score);

}