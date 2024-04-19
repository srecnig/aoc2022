use std::fmt;

pub struct Command {
    pub source: i32,
    pub destination: i32,
    pub count: i32,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.count, self.source, self.destination
        )
    }
}

pub fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split("from").collect(); // move 1 from 2 to 1
    let move_part = parts[0]; // move 1
    let location_part = parts[1]; // 2 to 1

    let count_str: Vec<&str> = move_part.trim().split_whitespace().collect();
    let count = count_str[1].parse::<i32>().unwrap();
    let locations: Vec<i32> = location_part
        .trim()
        .split("to")
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect();
    Command {
        source: locations[0],
        destination: locations[1],
        count,
    }
}

pub struct Stack {
    number: i32,
    crates: Vec<Crate>,
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let crates_str = self
            .crates
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}: {}", self.number, crates_str)
    }
}

impl Stack {
    pub fn pop(&mut self, count: i32) -> Vec<Crate> {
        let mut popped: Vec<Crate> = Vec::new();
        for _ in 0..count {
            popped.push(self.crates.pop().unwrap());
        }
        return popped;
    }

    pub fn push(&mut self, push: &mut Vec<Crate>) {
        self.crates.append(push);
    }

    pub fn top_crate(&self) -> char {
        self.crates.last().unwrap().symbol
    }
}

pub fn parse_stacks(lines: Vec<&str>) -> Vec<Stack> {
    // // stacks
    // let lines_before_empty_line: Vec<&str> = parts[0].lines().collect();
    let mut stacks: Vec<Stack> = Vec::new();

    // first, get the index of all the stacks and create the empty stacks
    let baseline = lines.last().unwrap();
    let mut positions: Vec<usize> = Vec::new();
    for (index, ch) in baseline.char_indices() {
        if ch.is_digit(10) {
            positions.push(index);
            stacks.push(Stack {
                number: ch.to_string().parse::<i32>().unwrap(),
                crates: vec![],
            });
        }
    }
    let positions = positions; // we won't mutate it anymore

    // now fill the stacks backwards
    for line in lines.iter().rev().skip(1) {
        println!("{}", line);
        for (stack_index, &crate_index) in positions.iter().enumerate() {
            let crate_symbol = line.chars().nth(crate_index);
            match crate_symbol {
                Some(symbol) => {
                    if !symbol.is_whitespace() {
                        let mut new_crate = vec![Crate { symbol }];
                        stacks[stack_index].push(&mut new_crate);
                    }
                }
                None => continue,
            }
        }
    }
    stacks
}

pub struct Crate {
    symbol: char,
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_do_somthing() {
        // not a real test, just for debugging
        let mut stack1 = Stack {
            number: 1,
            crates: vec![Crate { symbol: 'Z' }, Crate { symbol: 'N' }],
        };

        let stack2 = Stack {
            number: 2,
            crates: vec![
                Crate { symbol: 'M' },
                Crate { symbol: 'C' },
                Crate { symbol: 'D' },
            ],
        };

        let mut stack3 = Stack {
            number: 3,
            crates: vec![Crate { symbol: 'P' }],
        };

        println!("before move");
        println!("{}", stack1);
        println!("{}", stack2);
        println!("{}", stack3);

        let mut popped = stack1.pop(2);
        stack3.push(&mut popped);

        println!("after move");
        println!("{}", stack1);
        println!("{}", stack2);
        println!("{}", stack3);
    }

    #[test]
    fn can_parse_command() {
        let command_str = "move 3 from 1 to 2";
        let command = parse_command(command_str);
        assert_eq!(3, command.count);
        assert_eq!(1, command.source);
        assert_eq!(2, command.destination);
    }

    #[test]
    fn can_parse_stacks() {
        let stack_description = r#"    [D]
    [N] [C]
[Z] [M] [P]
 1   2   3"#;
        println!("geilo!!");
        let stacks = parse_stacks(stack_description.lines().collect());
        assert_eq!("1: [Z]", &stacks[0].to_string());
        assert_eq!("2: [M] [N] [D]", &stacks[1].to_string());
        assert_eq!("3: [P] [C]", &stacks[2].to_string());
    }

    #[test]
    fn can_pop_from_stack() {
        let mut stack = Stack {
            number: 1,
            crates: vec![
                Crate { symbol: 'M' },
                Crate { symbol: 'C' },
                Crate { symbol: 'D' },
            ],
        };
        let popped = stack.pop(2);
        assert_eq!('D', popped[0].symbol);
        assert_eq!('C', popped[1].symbol);
        assert_eq!('M', stack.crates[0].symbol);
    }

    #[test]
    fn can_push_to_stack() {
        let mut stack = Stack {
            number: 1,
            crates: vec![],
        };
        let mut to_push = vec![Crate { symbol: 'X' }, Crate { symbol: 'A' }];
        stack.push(&mut to_push);
        assert_eq!('X', stack.crates[0].symbol);
        assert_eq!('A', stack.crates[1].symbol);
    }

    #[test]
    fn can_get_top_crate_symbol() {
        let stack = Stack {
            number: 1,
            crates: vec![Crate { symbol: 'A' }, Crate { symbol: 'Z' }],
        };
        assert_eq!('Z', stack.top_crate());
    }
}
