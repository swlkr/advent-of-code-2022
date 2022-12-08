use std::fs::read_to_string;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Game {
    player_one_shape: Shape,
    player_two_shape: Shape,
}

impl Game {
    fn from(player_one: &str, player_two: &str) -> Game {
        return Game {
            player_one_shape: Game::player_one_shape(player_one),
            player_two_shape: Game::player_two_shape(player_two),
        };
    }

    fn player_two_shape(player_two: &str) -> Shape {
        match player_two {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Shape for player two could not be decoded"),
        }
    }

    fn player_one_shape(player_one: &str) -> Shape {
        match player_one {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Shape for player one could not be decoded"),
        }
    }

    fn player_two_shape_part2(player_one_shape: &Shape, player_two: &str) -> Shape {
        match player_two {
            "X" => match player_one_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            "Y" => match player_one_shape {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
            "Z" => match player_one_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            _ => panic!("Shape for player two could not be decoded"),
        }
    }

    fn from_part2(player_one: &str, player_two: &str) -> Game {
        let player_one_shape = Game::player_one_shape(player_one);
        let player_two_shape = Game::player_two_shape_part2(&player_one_shape, player_two);
        return Game {
            player_one_shape,
            player_two_shape,
        };
    }

    fn shape_score(&self) -> usize {
        match self.player_two_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome_score(&self) -> usize {
        // lose cases are zero
        // draw cases are 3
        // second player win cases are 6
        match (&self.player_one_shape, &self.player_two_shape) {
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
        }
    }

    fn score(&self) -> usize {
        self.shape_score() + self.outcome_score()
    }
}

fn main() -> std::io::Result<()> {
    let input = read_to_string("./bin/day2.txt")?;
    let mut score = 0;
    for line in input.lines() {
        if let Some(parts) = line.split_once(' ') {
            let game = Game::from(parts.0, parts.1);
            score += game.score();
        }
    }
    println!("{}", score);
    let mut score2 = 0;
    for line in input.lines() {
        if let Some(parts) = line.split_once(' ') {
            let game = Game::from_part2(parts.0, parts.1);
            score2 += game.score();
        }
    }
    println!("{}", score2);
    return Ok(());
}
