use std::fmt;

struct Stack {
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
}

struct Crate {
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
}

// let lines: Vec<Vec<&str>> = stack_map.lines().map(|line| line.split_whitespace().collect()).collect();
// let transposed: Vec<Vec<&str>> = (0..lines[0].len()).map(|i| lines.iter().map(|line| line[i]).collect()).collect();
// transposed.into_iter().map(|row| {
//     row.iter().map(|&s| s.chars().next().unwrap()).collect()
// }).collect()

// println!("{}", stack_map);
// println!("{}", transposed);

// pub fn parse_stacks(stack_map: &str) -> bool {
//     // let's do that later
//     return false;
// }
