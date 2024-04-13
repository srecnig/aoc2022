use dec03::{inspect_rucksack, inspect_rucksack_group};
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let total_priority: i32 = input
        .lines()
        .map(|rucksack| inspect_rucksack(rucksack))
        .sum();

    let rucksacks: Vec<_> = input.lines().collect();
    let rucksack_groups: Vec<_> = rucksacks.chunks(3).collect();
    let group_priority: i32 = rucksack_groups
        .iter()
        .map(|rucksack_group| {
            inspect_rucksack_group(rucksack_group[0], rucksack_group[1], rucksack_group[2])
        })
        .sum();

    println!("total_priority: {:?}", total_priority);
    println!("group_priority: {:?}", group_priority);
}
