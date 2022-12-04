use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut calories = Vec::new();
    let mut current_calories = 0;

    for (i, line) in lines.enumerate() {
        let l = line?;

        if l.len() > 0 {
            current_calories += match l.parse::<usize>() {
                Ok(n) => std::io::Result::Ok(n),
                Err(e) => std::io::Result::Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Couldn't parse line {i}: {e:?}"),
                )),
            }?;
        } else {
            calories.push(current_calories);
            current_calories = 0;
        }
    }

    calories.sort();
    let max_calories = calories.last().unwrap();
    let top_3_calories: usize = calories.iter().rev().take(3).sum();
    println!("a) Maximum Calories: {max_calories}");
    println!("b) Top 3 Calories: {top_3_calories}");
    return Ok(());
}
