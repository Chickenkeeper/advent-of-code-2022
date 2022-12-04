use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_shape_and_outcome(shape: Shape, target_outcome: Outcome) -> Shape {
        match (shape, target_outcome) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }

    fn from_char(char: char) -> Result<Self, &'static str> {
        match char {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("Invalid char"),
        }
    }

    fn to_score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_shapes(player_shape: Shape, opponent_shape: Shape) -> Outcome {
        match (opponent_shape, player_shape) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Lose,
            (Shape::Paper, Shape::Rock) => Outcome::Lose,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Lose,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn from_char(char: char) -> Result<Self, &'static str> {
        match char {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Invalid char"),
        }
    }

    fn to_score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let lines = BufReader::new(file).lines();
    let mut score = 0;

    for (i, line) in lines.enumerate() {
        let l = line.map_err(|e| format!("Error reading line {i}: {e:?}"))?;
        let l_bytes = l.as_bytes();
        let opponent_shape = Shape::from_char(l_bytes[0] as char)
            .map_err(|_| format!("Invalid opponent shape on line {i}"))?;
        let player_shape = Shape::from_char(l_bytes[2] as char)
            .map_err(|_| format!("Invalid player shape on line {i}"))?;
        let round_outcome = Outcome::from_shapes(player_shape, opponent_shape);

        score += player_shape.to_score();
        score += round_outcome.to_score();
    }

    return Ok(score);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let lines = BufReader::new(file).lines();
    let mut score = 0;

    for (i, line) in lines.enumerate() {
        let l = line.map_err(|e| format!("Error reading line {i}: {e:?}"))?;
        let l_bytes = l.as_bytes();
        let opponent_shape = Shape::from_char(l_bytes[0] as char)
            .map_err(|_| format!("Invalid opponent shape on line {i}"))?;
        let target_outcome = Outcome::from_char(l_bytes[2] as char)
            .map_err(|_| format!("Invalid round outcome on line {i}"))?;
        let target_player_shape = Shape::from_shape_and_outcome(opponent_shape, target_outcome);

        score += target_outcome.to_score();
        score += target_player_shape.to_score();
    }

    return Ok(score);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Total score: {part_1}");
    println!("Part 2 - Total score: {part_2}");
    return Ok(());
}
