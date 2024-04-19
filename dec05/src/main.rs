use dec05::{parse_command, parse_stacks, Command};
use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Failed to read input! Wat?1");
    let parts: Vec<&str> = input.splitn(2, "\n\n").collect();
    let lines_before_empty_line: Vec<&str> = parts[0].lines().collect();

    // stacks
    let mut stacks_pt1 = parse_stacks(&lines_before_empty_line);
    let mut stacks_pt2 = parse_stacks(&lines_before_empty_line);

    // commands
    let lines_after_empty_line: Vec<&str> = parts[1].lines().collect();
    let commands: Vec<Command> = lines_after_empty_line
        .iter()
        .map(|line| parse_command(line))
        .collect();

    // pt1
    for command in &commands {
        let mut popped = stacks_pt1[command.source as usize - 1].pop(command.count);
        stacks_pt1[command.destination as usize - 1].push(&mut popped);
    }
    let result_char_pt1: Vec<char> = stacks_pt1.iter().map(|stack| stack.top_crate()).collect();
    let result_str_pt1: String = result_char_pt1.into_iter().collect();
    println!("{}", result_str_pt1);

    // pt2
    for command in &commands {
        let mut popped = stacks_pt2[command.source as usize - 1].pop(command.count);
        stacks_pt2[command.destination as usize - 1].push_reversed(&mut popped);
    }
    let result_char_pt2: Vec<char> = stacks_pt2.iter().map(|stack| stack.top_crate()).collect();
    let result_str_pt2: String = result_char_pt2.into_iter().collect();
    println!("{}", result_str_pt2);
}
