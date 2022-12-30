use std::error::Error;
use std::fs;

use std::str::FromStr;

use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = fs::read_to_string("./input.txt")?;
    
    let data = input_data
        .split("\n\n")
        .map(|row| row.split("\n")
            .filter(|item| item.len() > 0)
            .map(|item| i32::from_str(item).unwrap_or(0))
            .sum::<i32>()
        )
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec();

    let result = data.iter().rev()
        .take(3)
        .sum::<i32>();


    println!("{result:#?}");

    Ok(())
}
