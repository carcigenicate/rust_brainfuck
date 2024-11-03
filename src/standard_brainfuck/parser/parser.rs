use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum EqualityOperator {
    NotEqual = 0,
    Equal = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    AddToCell { n: i8 },
    AddToCellPtr { offset: isize },
    JumpToIf { position: usize, operator: EqualityOperator, match_value: u8 },
    PrintOut,
    ReadIn,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Instruction::AddToCell { n } => format!("Add {n}"),
            Instruction::AddToCellPtr { offset } => format!("Move slots by {offset}"),
            Instruction::JumpToIf { position, operator, match_value } => format!("Jump to {position} when value {operator} {match_value}"),
            Instruction::PrintOut => "Print".to_string(),
            Instruction::ReadIn => "Read".to_string(),
        };

        return write!(f, "{}", output);
    }
}

fn find_loop_indices(tokens: &Vec<u8>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut start_to_end: HashMap<usize, usize> = HashMap::new();
    let mut end_to_start: HashMap<usize, usize> = HashMap::new();

    let mut loop_start_stack = vec![];

    for (i, symbol) in tokens.iter().enumerate() {
        if *symbol == b'[' {
            loop_start_stack.push(i);
        } else if *symbol == b']' {
            let start_i = match loop_start_stack.pop() {
                Some(start_i) => start_i,
                None => panic!("] missing a matching [ at {i}"),
            };

            start_to_end.insert(start_i, i);
            end_to_start.insert(i, start_i);
        }
    }

    if loop_start_stack.len() > 0 {
        panic!("[ missing a matching ]: {loop_start_stack:?}");
    }

    return (start_to_end, end_to_start);
}

pub fn parse(code: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let tokens = Vec::from(code);

    let (start_to_end, end_to_start) = find_loop_indices(&tokens);
    println!("End to Start: {end_to_start:?}");

    for (i, symbol) in tokens.iter().enumerate() {
        let instruction = match symbol {
            b'+' => Some(Instruction::AddToCell { n: 1 }),
            b'-' => Some(Instruction::AddToCell { n: -1 }),
            b'<' => Some(Instruction::AddToCellPtr { offset: -1 }),
            b'>' => Some(Instruction::AddToCellPtr { offset: 1 }),
            b'[' => {
                let end_i = start_to_end.get(&i).unwrap();
                Some(Instruction::JumpToIf { position: *end_i, operator: EqualityOperator::Equal, match_value: 0 })
            },
            b']' => {
                let start_i = end_to_start.get(&i).unwrap();
                Some(Instruction::JumpToIf { position: *start_i, operator: EqualityOperator::NotEqual, match_value: 0 })
            },
            b'.' => Some(Instruction::PrintOut),
            b',' => Some(Instruction::ReadIn),
            _ => None,
        };

        match instruction {
            Some(inst) => instructions.push(inst),
            None => (),
        }
    }

    return instructions;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_produce_the_correct_instruction_for_each_token() {
        let code = "[+-<>.,]";
        let instructions = parse(code);

        assert_eq!(instructions[0], Instruction::JumpToIf { position: 7, operator: EqualityOperator::Equal, match_value: 0 });
        assert_eq!(instructions[7], Instruction::JumpToIf { position: 0, operator: EqualityOperator::NotEqual, match_value: 0 });

        assert_eq!(instructions[1], Instruction::AddToCell { n: 1 });
        assert_eq!(instructions[2], Instruction::AddToCell { n: -1 });
        assert_eq!(instructions[3], Instruction::AddToCellPtr { offset: -1 });
        assert_eq!(instructions[4], Instruction::AddToCellPtr { offset: 1 });
        assert_eq!(instructions[5], Instruction::PrintOut);
        assert_eq!(instructions[6], Instruction::ReadIn);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_on_mismatched_start_brace() {
        let code = "+[-";
        parse(code);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_on_mismatched_end_brace() {
        let code = "+]-";
        parse(code);
    }
}