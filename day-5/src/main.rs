use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::num::ParseIntError;

#[derive(Clone, Copy)]
struct CrateMovement {
    num_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

impl CrateMovement {
    // TODO: remove vec and add errors for missing values
    fn from_string(movement: String) -> Result<CrateMovement, ParseIntError> {
        let movement: Vec<_> = movement.split_whitespace().collect();
        let num_crates = movement[1].parse::<usize>()?;
        let from_stack = movement[3].parse::<usize>()?.saturating_sub(1);
        let to_stack = movement[5].parse::<usize>()?.saturating_sub(1);

        return Ok(CrateMovement {
            num_crates,
            from_stack,
            to_stack,
        });
    }
}

struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    // TODO: this needs cleaning up
    fn from_file(lines: &mut Lines<BufReader<File>>) -> Result<CrateStacks, std::io::Error> {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        loop {
            let line = lines.next().unwrap();
            let l = line?;
            let bytes = l.as_bytes();

            // TODO: add protection against reaching the end of the file before reaching the stack numbers
            if (bytes[1] as char).is_ascii_digit() {
                break;
            }

            while stacks.len() < (bytes.len() + 1) / 4 {
                stacks.push(Vec::new());
            }

            for (i, b) in bytes.iter().skip(1).step_by(4).enumerate() {
                if *b != b' ' {
                    stacks[i].push(*b as char);
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        return Ok(CrateStacks { stacks });
    }

    // TODO: these need refactoring too
    fn move_crates_sequential(&mut self, movement: CrateMovement) -> Result<(), ParseIntError> {
        let start = self.stacks[movement.from_stack].len().saturating_sub(1);

        for i in 0..movement.num_crates {
            let c = self.stacks[movement.from_stack][start - i];
            self.stacks[movement.to_stack].push(c);
        }

        self.stacks[movement.from_stack].truncate(start.saturating_sub(movement.num_crates) + 1);
        return Ok(());
    }

    fn move_crates_grouped(&mut self, movement: CrateMovement) -> Result<(), ParseIntError> {
        let start = self.stacks[movement.from_stack].len() - movement.num_crates;

        for i in 0..movement.num_crates {
            let c = self.stacks[movement.from_stack][start + i];
            self.stacks[movement.to_stack].push(c);
        }

        self.stacks[movement.from_stack].truncate(start);
        return Ok(());
    }

    fn get_top_crates(&mut self) -> String {
        let mut top_crates = String::new();
        for stack in self.stacks.iter() {
            let top_crate = stack[stack.len().saturating_sub(1)];
            top_crates.push(top_crate);
        }

        return top_crates;
    }
}

fn solution_part_1() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut lines = BufReader::new(file).lines();
    let mut crate_stacks = CrateStacks::from_file(&mut lines)?;

    for line in lines.skip(1) {
        let crate_movement = CrateMovement::from_string(line?)?;
        crate_stacks.move_crates_sequential(crate_movement)?;
    }

    return Ok(crate_stacks.get_top_crates());
}

fn solution_part_2() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut lines = BufReader::new(file).lines();
    let mut crate_stacks = CrateStacks::from_file(&mut lines)?;

    for line in lines.skip(1) {
        let crate_movement = CrateMovement::from_string(line?)?;
        crate_stacks.move_crates_grouped(crate_movement)?;
    }

    return Ok(crate_stacks.get_top_crates());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Top crates: {part_1}");
    println!("Part 2 - Top crates: {part_2}");
    return Ok(());
}
