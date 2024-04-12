use std::fs;

use dec01::{max_calories, max_calories_top_n};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let calories: Vec<Vec<i32>> = chunks
        .iter()
        .map(|c| c.lines().map(|l| l.parse().unwrap()).collect())
        .collect();

    println!("max_calories: {:?}", max_calories(&calories));
    println!("max_calories_top_n: {:?}", max_calories_top_n(&calories, 3))
}
