use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let mut score = 0;

    for (i, line) in lines.enumerate() {
        let l = line?;
        let l_bytes = l.as_bytes();
        let opponent_shape = match l_bytes[0] {
            b'A' => std::io::Result::Ok(Shape::Rock),
            b'B' => std::io::Result::Ok(Shape::Paper),
            b'C' => std::io::Result::Ok(Shape::Scissors),
            _ => std::io::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("'Invalid opponent shape on line {i}"),
            )),
        }?;
        let target_outcome = match l_bytes[2] {
            b'X' => std::io::Result::Ok(Outcome::Lose),
            b'Y' => std::io::Result::Ok(Outcome::Draw),
            b'Z' => std::io::Result::Ok(Outcome::Win),
            _ => std::io::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("'Invalid round outcome on line {i}"),
            )),
        }?;

        // score for the round
        score += match target_outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        // shape needed to reach the round's outcome
        let player_shape = match (opponent_shape, target_outcome) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        };

        // score for the selected shape
        score += match player_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
    }

    println! {"Total score: {score}"};
    return Ok(());
}
