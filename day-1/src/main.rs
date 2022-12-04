use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let lines = BufReader::new(file).lines();
    let mut max_calories = 0;
    let mut current_calories = 0;

    for (i, line) in lines.enumerate() {
        let l = line.map_err(|e| format!("Error reading line {i}: {e:?}"))?;

        if !l.is_empty() {
            current_calories += l
                .parse::<usize>()
                .map_err(|e| format!("Error parsing line {i}: {e:?}"))?;
        } else {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    return Ok(max_calories);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let lines = BufReader::new(file).lines();
    let mut max_calories = 0;
    let mut second_most_calories = 0;
    let mut third_most_calories = 0;
    let mut current_calories = 0;

    for (i, line) in lines.enumerate() {
        let l = line.map_err(|e| format!("Error reading line {i}: {e:?}"))?;

        if !l.is_empty() {
            current_calories += l
                .parse::<usize>()
                .map_err(|e| format!("Error parsing line {i}: {e:?}"))?;
        } else {
            if current_calories > max_calories {
                third_most_calories = second_most_calories;
                second_most_calories = max_calories;
                max_calories = current_calories;
            } else if current_calories > second_most_calories {
                third_most_calories = second_most_calories;
                second_most_calories = current_calories;
            } else if current_calories > third_most_calories {
                third_most_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    return Ok(max_calories + second_most_calories + third_most_calories);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Maximum Calories: {part_1}");
    println!("Part 2 - Top 3 Calories: {part_2}");
    return Ok(());
}
