use dec05::{parse_command, parse_stacks, Command, Crate};
use std::fs;

fn main() {
    let input = fs::read_to_string("input1-test.txt").expect("Failed to read input! Wat?1");
    let parts: Vec<&str> = input.splitn(2, "\n\n").collect();

    // stack
    let lines_before_empty_line: Vec<&str> = parts[0].lines().collect();
    let mut stacks = parse_stacks(lines_before_empty_line);

    // commands
    let lines_after_empty_line: Vec<&str> = parts[1].lines().collect();
    let commands: Vec<Command> = lines_after_empty_line
        .iter()
        .map(|line| parse_command(line))
        .collect();

    for command in &commands {
        println!("{}", command);
        let mut popped = stacks[command.source as usize - 1].pop(command.count);
        stacks[command.destination as usize - 1].push(&mut popped);
    }

    let result_char: Vec<char> = stacks.iter().map(|stack| stack.top_crate()).collect();
    let result_str: String = result_char.into_iter().collect();
    println!("{}", result_str);
}
