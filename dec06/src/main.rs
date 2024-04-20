use dec06::detect_marker;
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let marker = detect_marker(&input);
    println!("{}", marker + 1);
}
