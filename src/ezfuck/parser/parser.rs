use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::string::ToString;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum EqualityOperator {
    NotEqual,
    Equal,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum MathOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum InstructionValue {
    CurrentCell,
    Number(u8),
}

impl InstructionValue {
    pub fn determine_value(self, current_cell_value: u8) -> u8 {
        return match self {
            InstructionValue::Number(n) => n,
            InstructionValue::CurrentCell => current_cell_value,
        }
    }
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    ApplyOperatorToCell { operator: MathOperator, value: InstructionValue },
    AddToCellPtr { direction: Direction, offset: InstructionValue },
    JumpToIf { position: usize, operator: EqualityOperator, match_value: u8 },
    PrintOut,
    ReadIn,
    SetCell { value: InstructionValue },
    Breakpoint,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Instruction::ApplyOperatorToCell { operator, value } => format!("Cell <{operator}> {value}"),
            Instruction::AddToCellPtr { direction, offset } => format!("Move slots by {offset} {direction}"),
            Instruction::JumpToIf { position, operator, match_value } => format!("Jump to {position} when value {operator} {match_value}"),
            Instruction::PrintOut => "Print".to_string(),
            Instruction::ReadIn => "Read".to_string(),
            Instruction::SetCell { value} => format!("Set Cell to {value}"),
            Instruction::Breakpoint => "Breakpoint".to_string(),
        };

        return write!(f, "{}", output);
    }
}

#[derive(Copy, Clone)]
pub struct Token {
    instruction_symbol: char,
    value: Option<InstructionValue>,
}

impl Token {
    fn get_defaulted_value(self) -> InstructionValue {
        return self.value.unwrap_or(InstructionValue::Number(1));
    }

    fn has_value(self) -> bool {
        return self.value.is_some();
    }
}

const INSTRUCTION_SYMBOLS: &str = "+-*/<>[]^.,!";
const VALUELESS_INSTRUCTION_SYMBOLS: &str = "[],.!";
const VALUE_SYMBOLS: &str = "1234567890V";

fn find_loop_indices(tokens: &Vec<Token>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut start_to_end: HashMap<usize, usize> = HashMap::new();
    let mut end_to_start: HashMap<usize, usize> = HashMap::new();

    let mut loop_start_stack = vec![];

    for (i, token) in tokens.iter().enumerate() {
        let symbol = token.instruction_symbol;
        if symbol == '[' {
            loop_start_stack.push(i);
        } else if symbol == ']' {
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
// +3[>+4<-]
fn get_token_and_advance(code: &Vec<char>, start_i: usize) -> Option<(Token, usize)> {
    let mut instruction_symbol: Option<char> = None;
    let mut raw_value: String = String::new();

    let mut last_consumed_i: usize = start_i;
    for i in start_i..start_i+4 {  // "+4" to account for the max length: +255
        match code.get(i) {
            Some(symbol) => {
                if INSTRUCTION_SYMBOLS.contains(*symbol) {
                    if instruction_symbol.is_none() {
                        instruction_symbol = Some(*symbol);
                        last_consumed_i = i;
                    } else {
                        break;
                    }
                } else if VALUE_SYMBOLS.contains(*symbol) && instruction_symbol.is_some() {
                    raw_value.push(*symbol);
                    last_consumed_i = i;
                }
            },
            None => {
                break;
            }
        }
    }

    return match instruction_symbol {
        Some(symbol) => {
            let mut value: Option<InstructionValue> = None;
            if raw_value.len() > 0 {
                if raw_value == "V" {
                    value = Some(InstructionValue::CurrentCell);
                } else {
                    value = match raw_value.parse::<u8>() {
                        Ok(parsed) => Some(InstructionValue::Number(parsed)),
                        Err(e) => {
                            panic!("Error parsing value {raw_value}: {e:?}");
                        }
                    }
                }
            }

            let token = Token { instruction_symbol: symbol, value: value };
            Some((token, last_consumed_i))
        },
        None => None,
    }
}

fn lex(code: &Vec<char>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_token_i = 0;
    while current_token_i < code.len() {
        match get_token_and_advance(code, current_token_i) {
            Some((token, i)) => {
                tokens.push(token);
                current_token_i = i + 1;
            },
            None => {
                current_token_i += 1;
            },
        }
    }

    return tokens;
}

fn assert_valueless(token: Token) {
    if token.has_value() {
        panic!("Command {:?} cannot be given a value. Given {:?}.", token.instruction_symbol, token.value);
    }
}

fn parse(tokens: Vec<Token>, allow_debugging: bool) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let (start_to_end, end_to_start) = find_loop_indices(&tokens);
    for (i, token) in tokens.iter().enumerate() {
        if VALUELESS_INSTRUCTION_SYMBOLS.contains(token.instruction_symbol) {
            assert_valueless(*token);
        }

        let defaulted_value = token.get_defaulted_value();
        let instruction = match token.instruction_symbol {
            '+' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: defaulted_value }),
            '-' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Subtraction, value: defaulted_value }),
            '*' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: defaulted_value }),
            '/' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Division, value: defaulted_value }),
            '<' => Some(Instruction::AddToCellPtr { direction: Direction::Left, offset: defaulted_value }),
            '>' => Some(Instruction::AddToCellPtr { direction: Direction::Right, offset: defaulted_value }),
            '[' => {
                let end_i = start_to_end.get(&i).unwrap();
                Some(Instruction::JumpToIf { position: *end_i, operator: EqualityOperator::Equal, match_value: 0 })
            },
            ']' => {
                let start_i = end_to_start.get(&i).unwrap();
                Some(Instruction::JumpToIf { position: *start_i, operator: EqualityOperator::NotEqual, match_value: 0 })
            },
            '.' => Some(Instruction::PrintOut),
            ',' => Some(Instruction::ReadIn),
            '^' => Some(Instruction::SetCell { value: defaulted_value }),
            '!' => if allow_debugging { Some(Instruction::Breakpoint) } else { None },
            _ => None,
        };

        match instruction {
            Some(inst) => instructions.push(inst),
            None => (),
        }
    }

    return instructions;
}

pub fn compile_to_intermediate(code: &str, allow_debugging: bool) -> Vec<Instruction> {
    let code_vec: Vec<char> = code.chars().collect();
    let tokens = lex(&code_vec);
    let instructions = parse(tokens, allow_debugging);
    return instructions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_ignore_invalid_characters() {
        let code = "+None of this should be considered*";
        let instructions = compile_to_intermediate(code, false);

        assert_eq!(instructions.len(), 2);

        assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(1) });
        assert_eq!(instructions[1], Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: InstructionValue::Number(1) });
    }

    #[test]
    fn it_should_produce_the_correct_instruction_for_each_token() {
        let code = "[]+-*/<>.,^";
        let instructions = compile_to_intermediate(code, false);

        assert_eq!(instructions.len(), 11);

        assert_eq!(instructions[0], Instruction::JumpToIf { position: 1, operator: EqualityOperator::Equal, match_value: 0 });
        assert_eq!(instructions[1], Instruction::JumpToIf { position: 0, operator: EqualityOperator::NotEqual, match_value: 0 });

        assert_eq!(instructions[2], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(1) });
        assert_eq!(instructions[3], Instruction::ApplyOperatorToCell { operator: MathOperator::Subtraction, value: InstructionValue::Number(1) });
        assert_eq!(instructions[4], Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: InstructionValue::Number(1) });
        assert_eq!(instructions[5], Instruction::ApplyOperatorToCell { operator: MathOperator::Division, value: InstructionValue::Number(1) });
        assert_eq!(instructions[6], Instruction::AddToCellPtr { direction: Direction::Left, offset: InstructionValue::Number(1) });
        assert_eq!(instructions[7], Instruction::AddToCellPtr { direction: Direction::Right, offset: InstructionValue::Number(1) });
        assert_eq!(instructions[8], Instruction::PrintOut);
        assert_eq!(instructions[9], Instruction::ReadIn);
    }

    #[test]
    fn it_should_properly_read_instruction_values_and_default_missing_ones_to_one() {
        let code = "++1+2+3+40+200";
        let instructions = compile_to_intermediate(code, false);

        assert_eq!(instructions.len(), 6);

        assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(1) });
        assert_eq!(instructions[1], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(1) });
        assert_eq!(instructions[2], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(2) });
        assert_eq!(instructions[3], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(3) });
        assert_eq!(instructions[4], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(40) });
        assert_eq!(instructions[5], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::Number(200) });
    }

    #[test]
    fn it_should_properly_add_insertion_values() {
        let code = "+V";
        let instructions = compile_to_intermediate(code, false);

        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: InstructionValue::CurrentCell });
    }

    #[test]
    #[should_panic]
    fn it_should_panic_on_mismatched_start_brace() {
        let code = "+[-";
        compile_to_intermediate(code, false);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_on_mismatched_end_brace() {
        let code = "+]-";
        compile_to_intermediate(code, false);
    }
}