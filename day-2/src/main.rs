use std::string::ParseError;

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn aginst(&self, opp: Move) -> Outcome {
        match self {
            Move::Rock => match opp {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Loss,
                Move::Scissor => Outcome::Win,
            },
            Move::Paper => match opp {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissor => Outcome::Loss,
            },
            Move::Scissor => match opp {
                Move::Rock => Outcome::Loss,
                Move::Paper => Outcome::Win,
                Move::Scissor => Outcome::Draw,
            },
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => unreachable!(),
        }
    }
}

enum Hint {
    Win,
    Draw,
    Loss,
}

impl Hint {
    fn get_move_aginst(&self, opp: Move) -> Move {
        match self {
            Hint::Loss => match opp {
                Move::Rock => Move::Scissor,
                Move::Paper => Move::Rock,
                Move::Scissor => Move::Paper,
            },
            Hint::Draw => opp,
            Hint::Win => match opp {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissor,
                Move::Scissor => Move::Rock,
            },
        }
    }
}

impl From<&str> for Hint {
    fn from(value: &str) -> Self {
        match value {
            "X" => Hint::Loss,
            "Y" => Hint::Draw,
            "Z" => Hint::Win,
            _ => unreachable!(),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self, _move: Move) -> usize {
        let x = match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };

        let y = match _move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        };

        x + y
    }
}

// score = move points + outcome points

struct GameInput {
    _move: Move,
    hint: Hint,
}

impl TryFrom<&str> for GameInput {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut input_parts = value.split_whitespace();
        let _move = input_parts.next().unwrap();
        let hint = input_parts.next().unwrap();

        Ok(GameInput {
            _move: Move::from(_move),
            hint: Hint::from(hint),
        })
    }
}

fn main() {
    let game_inputs = include_str!("input.txt")
        .lines()
        .filter_map(|line| GameInput::try_from(line).ok())
        .collect::<Vec<_>>();

    let mut score = 0;
    for input in game_inputs {
        let other_move = input._move;
        let my_move = input.hint.get_move_aginst(other_move.clone());
        score += my_move.aginst(other_move).score(my_move);
    }

    println!("Score = {}", score);
}
