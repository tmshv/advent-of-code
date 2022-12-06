use std::io;

#[derive(Debug)]
enum Shape {
    Rock,
    Scissors,
    Papper,
}

#[derive(Debug)]
enum GameResult {
    Lost,
    Won,
    Draw,
}

fn get_result(you: &Shape, other: &Shape) -> GameResult {
    match (you, other) {
        (Shape::Rock, Shape::Scissors) => GameResult::Won,
        (Shape::Papper, Shape::Rock) => GameResult::Won,
        (Shape::Scissors, Shape::Papper) => GameResult::Won,

        (Shape::Rock, Shape::Papper) => GameResult::Lost,
        (Shape::Papper, Shape::Scissors) => GameResult::Lost,
        (Shape::Scissors, Shape::Rock) => GameResult::Lost,

        (Shape::Rock, Shape::Rock) => GameResult::Draw,
        (Shape::Papper, Shape::Papper) => GameResult::Draw,
        (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
    }
}

fn get_game_score(result: &GameResult) -> i32 {
    match result {
        GameResult::Won => 6,
        GameResult::Lost => 0,
        GameResult::Draw => 3,
    }
}

fn get_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Papper => 2,
        Shape::Scissors => 3,
    }
}

fn main() {
    let mut total_score = 0;

    for line in io::stdin().lines() {
        let l = line.unwrap();
        let s: Vec<&str> = l.split(" ").collect();

        // let elf_turn = elf.get(s[0]).unwrap();
        // let you_turn = you.get(s[1]).unwrap();
        let (elf_turn, you_turn) = match (s[0], s[1]) {
            ("A", "X") => (&Shape::Rock, &Shape::Scissors),
            ("A", "Y") => (&Shape::Rock, &Shape::Rock),
            ("A", "Z") => (&Shape::Rock, &Shape::Papper),

            ("B", "X") => (&Shape::Papper, &Shape::Rock),
            ("B", "Y") => (&Shape::Papper, &Shape::Papper),
            ("B", "Z") => (&Shape::Papper, &Shape::Scissors),

            ("C", "X") => (&Shape::Scissors, &Shape::Papper),
            ("C", "Y") => (&Shape::Scissors, &Shape::Scissors),

            // ("C", "Z")
            (_, _) => (&Shape::Scissors, &Shape::Rock),
        };

        let result = get_result(you_turn, elf_turn);

        let shape_score = get_shape_score(you_turn);
        let game_score = get_game_score(&result);
        total_score += shape_score + game_score;

        println!("{:?}/{:?}: game={} shape={} total={}", elf_turn, you_turn, game_score, shape_score, total_score);
    }

    println!("Total score = {}", total_score);
}
