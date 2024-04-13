use dec02::build_game;
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let game_definitions: Vec<&str> = input.lines().collect();
    let total_score: i32 = game_definitions
        .iter()
        .map(|game_definition| {
            let game = build_game(game_definition);
            game.score()
        })
        .sum();
    println!("total_score: {:?}", total_score);
}
