use dec04::{ranges_containment, ranges_overlap};
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let ranges_with_containment = input
        .lines()
        .map(|line| ranges_containment(line))
        .filter(|&c| c)
        .count();
    let ranges_with_overlap = input
        .lines()
        .map(|line| ranges_overlap(line))
        .filter(|&c| c)
        .count();

    println!("with containment: {:?}", ranges_with_containment);
    println!("with overlap: {:?}", ranges_with_overlap);
}
