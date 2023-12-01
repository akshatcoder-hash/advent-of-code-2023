use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "./inputs/day-1.txt";

    let mut sum = 0;

    // Open the file in read-only mode
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // Extract the first and last digits
        if let Some(first_digit) = line.chars().find(|c| c.is_digit(10)) {
            if let Some(last_digit) = line.chars().rev().find(|c| c.is_digit(10)) {
                // Combine them into a two-digit number
                let value = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
                sum += value;
            }
        }
    }
    // Sher speaks the solution
    println!("Total sum of calibration values: {}", sum);

    Ok(())
}
