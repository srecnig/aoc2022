use dec06::detect_marker;
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input!");
    let packet_marker = detect_marker(&input, 4);
    let message_marker = detect_marker(&input, 14);
    println!("{}", packet_marker + 1);
    println!("{}", message_marker + 1);
}
