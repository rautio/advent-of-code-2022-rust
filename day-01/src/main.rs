use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut max_calories = 0;
    let mut current_calories = 0;
    let mut calorie_counts = Vec::new();
    for line in buffered.lines() {
        if let Ok(str) = line {
            if str.eq("") {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                calorie_counts.push(current_calories);
                current_calories = 0;
            } else {
                current_calories = current_calories + str.parse::<i32>().unwrap();
            }

        }
    }
    calorie_counts.sort_by(|a, b| b.cmp(a));
    
    println!("Answer 1: {}", max_calories);
    println!("Answer 2: {}", calorie_counts[0] + calorie_counts[1] + calorie_counts[2]);

    Ok(())
}