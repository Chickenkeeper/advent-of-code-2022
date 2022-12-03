use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Shape {
    Rock,
    Paper,
    Scissors,
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
        let player_shape = match l_bytes[2] {
            b'X' => std::io::Result::Ok(Shape::Rock),
            b'Y' => std::io::Result::Ok(Shape::Paper),
            b'Z' => std::io::Result::Ok(Shape::Scissors),
            _ => std::io::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("'Invalid player shape on line {i}"),
            )),
        }?;

        // score for the selected shape
        score += match player_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        // score for the round
        score += match (opponent_shape, player_shape) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
        }
    }

    println! {"Total score: {score}"};
    return Ok(());
}
