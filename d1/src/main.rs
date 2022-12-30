use std::error::Error;
use std::fs;

use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("./input.txt")?;
    
    let data = input_data
        .split("\n\n")
        .map(|row| row.split("\n")
            .filter(|item| item.len() > 0)
            .map(|item| i32::from_str(item).unwrap_or(0))
            .sum::<i32>()
        )
        .max();


    println!("{data:#?}");

    Ok(())
}
