use std::fs;

fn main() {
    let input = fs::read_to_string("input1-test.txt").expect("Failed to read input!");
    let lines: Vec<&str> = input.lines().collect();
}
