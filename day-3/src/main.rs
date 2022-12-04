use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_item_priority(item: char) -> Result<usize, &'static str> {
    if item.is_ascii_lowercase() {
        return Ok((item as usize) - (b'a' as usize) + 1);
    } else if item.is_ascii_uppercase() {
        return Ok((item as usize) - (b'A' as usize) + 27);
    } else {
        return Err("Items must be uppercase or lowercase ASCII characters");
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let lines = BufReader::new(file).lines();
    let mut priority_sum = 0;

    'outer: for (i, line) in lines.enumerate() {
        let l = line.map_err(|e| format!("Error reading line {i}: {e:?}"))?;
        let compartments = l.split_at(l.len() / 2);

        for c0 in compartments.0.chars() {
            for c1 in compartments.1.chars() {
                if c0 == c1 {
                    priority_sum += get_item_priority(c1)
                        .map_err(|e| format!("Error parsing line {i}: {e}"))?;
                    continue 'outer;
                }
            }
        }
        return Err("No common item found")?;
    }
    return Ok(priority_sum);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    let mut priority_sum = 0;

    'outer: for (i, rucksacks) in lines.chunks_exact(3).enumerate() {
        for item_1 in rucksacks[0].chars() {
            for item_2 in rucksacks[1].chars() {
                for item_3 in rucksacks[2].chars() {
                    if item_1 == item_2 && item_1 == item_3 {
                        priority_sum += get_item_priority(item_1)
                            .map_err(|e| format!("Error parsing group {i}: {e}"))?;
                        continue 'outer;
                    }
                }
            }
        }
        return Err(format!("No common item found in group {i}"))?;
    }
    return Ok(priority_sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Rucksack priority sum: {part_1}");
    println!("Part 2 - Badge priority sum: {part_2}");
    return Ok(());
}
