pub fn build_game(definition: &str) -> Game {
    let shapes: Vec<&str> = definition.split_whitespace().collect();
    let opponent_shape = build_shape(shapes[0]).unwrap();
    let my_shape = build_shape(shapes[1]).unwrap();
    Game {
        my: my_shape,
        opponent: opponent_shape,
    }
}

pub fn build_predetermined_game(definition: &str) -> Game {
    let strategy: Vec<&str> = definition.split_whitespace().collect();
    let opponent_shape = build_shape(strategy[0]).unwrap();
    let expected_result = build_result(strategy[1]).unwrap();
    let my_shape = build_shape_by_result(&opponent_shape, &expected_result);
    Game {
        my: my_shape,
        opponent: opponent_shape,
    }
}

fn build_shape(symbol: &str) -> Option<Shape> {
    match symbol {
        "X" | "A" => Some(Shape::Rock),
        "Y" | "B" => Some(Shape::Paper),
        "Z" | "C" => Some(Shape::Scissors),
        _ => None,
    }
}

fn build_result(symbol: &str) -> Option<Result> {
    match symbol {
        "X" => Some(Result::Loss),
        "Y" => Some(Result::Draw),
        "Z" => Some(Result::Win),
        _ => None,
    }
}

fn build_shape_by_result(shape: &Shape, result: &Result) -> Shape {
    // result is from the perspective of the returned shape
    match shape {
        Shape::Rock => match result {
            Result::Loss => Shape::Scissors,
            Result::Draw => Shape::Rock,
            Result::Win => Shape::Paper,
        },
        Shape::Paper => match result {
            Result::Loss => Shape::Rock,
            Result::Draw => Shape::Paper,
            Result::Win => Shape::Scissors,
        },
        Shape::Scissors => match result {
            Result::Loss => Shape::Paper,
            Result::Draw => Shape::Scissors,
            Result::Win => Shape::Rock,
        },
    }
}

pub struct Game {
    my: Shape,
    opponent: Shape,
}

impl Game {
    pub fn score(&self) -> i32 {
        let result = self.my.play(&self.opponent);
        let game_score = match result {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6,
        };
        let hand_score = self.my.score();
        game_score + hand_score
    }
}

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play(&self, other: &Shape) -> Result {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Result::Draw,
                Shape::Paper => Result::Loss,
                Shape::Scissors => Result::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => Result::Win,
                Shape::Paper => Result::Draw,
                Shape::Scissors => Result::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => Result::Loss,
                Shape::Paper => Result::Win,
                Shape::Scissors => Result::Draw,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum Result {
    Win,
    Loss,
    Draw,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_game() {
        let game1 = build_game("A Y");
        assert_eq!(Shape::Rock, game1.opponent);
        assert_eq!(Shape::Paper, game1.my);
        let game2 = build_game("B X");
        assert_eq!(Shape::Paper, game2.opponent);
        assert_eq!(Shape::Rock, game2.my);
        let game3 = build_game("C Z");
        assert_eq!(Shape::Scissors, game3.opponent);
        assert_eq!(Shape::Scissors, game3.my);
    }

    #[test]
    fn can_build_predetermined_game() {
        let game1 = build_predetermined_game("A Y");
        assert_eq!(Shape::Rock, game1.opponent);
        assert_eq!(Shape::Rock, game1.my);
        let game2 = build_predetermined_game("B X");
        assert_eq!(Shape::Paper, game2.opponent);
        assert_eq!(Shape::Rock, game2.my);
        let game3 = build_predetermined_game("C Z");
        assert_eq!(Shape::Scissors, game3.opponent);
        assert_eq!(Shape::Rock, game3.my);
    }

    #[test]
    fn can_play_game() {
        let game1 = Game {
            my: Shape::Paper,
            opponent: Shape::Rock,
        };
        assert_eq!(8, game1.score());

        let game2 = Game {
            my: Shape::Rock,
            opponent: Shape::Paper,
        };
        assert_eq!(1, game2.score());

        let game3 = Game {
            my: Shape::Scissors,
            opponent: Shape::Scissors,
        };
        assert_eq!(6, game3.score())
    }

    #[test]
    fn can_get_shape_score() {
        assert_eq!(1, Shape::Rock.score());
        assert_eq!(2, Shape::Paper.score());
        assert_eq!(3, Shape::Scissors.score());
    }

    #[test]
    fn can_play_rock() {
        assert_eq!(Shape::Rock.play(&Shape::Rock), Result::Draw);
        assert_eq!(Shape::Rock.play(&Shape::Paper), Result::Loss);
        assert_eq!(Shape::Rock.play(&Shape::Scissors), Result::Win);
    }

    #[test]
    fn can_play_paper() {
        assert_eq!(Shape::Paper.play(&Shape::Rock), Result::Win);
        assert_eq!(Shape::Paper.play(&Shape::Paper), Result::Draw);
        assert_eq!(Shape::Paper.play(&Shape::Scissors), Result::Loss);
    }

    #[test]
    fn can_play_scissors() {
        assert_eq!(Shape::Scissors.play(&Shape::Rock), Result::Loss);
        assert_eq!(Shape::Scissors.play(&Shape::Paper), Result::Win);
        assert_eq!(Shape::Scissors.play(&Shape::Scissors), Result::Draw);
    }
}
