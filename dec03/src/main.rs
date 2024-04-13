use dec03::inspect_rucksack;
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let total_priority: i32 = input
        .lines()
        .map(|rucksack| inspect_rucksack(rucksack))
        .sum();
    println!("total_priority: {:?}", total_priority);
}
