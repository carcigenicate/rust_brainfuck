use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::iter::Scan;
use std::string::ToString;
use strum_macros::Display;

// const COMMAND_SYMBOLS: [&str; 12] = ["+", "-", "*", "/", "<", ">", "[", "]", "^", ",", ".", "!"];
const COMMAND_SYMBOLS: &str = "+-*/<>[]^.,!@";
const VALUELESS_COMMAND_SYMBOLS: &str = "[],.!";
const NUMERIC_LITERAL_SYMBOLS: &str  = "1234567890";
const CURRENT_CELL_SYMBOLS: &str  = "V";
const VALUE_SYMBOLS: &str = "1234567890V";

fn is_command_lexeme(lexeme: &String) -> bool {
    let first_symbol = lexeme.chars().next().unwrap();
    return lexeme.len() == 1 && COMMAND_SYMBOLS.contains(first_symbol);
}

fn is_numeric_literal_lexeme(lexeme: &String) -> bool {
    let first_symbol = lexeme.chars().next().unwrap();
    return NUMERIC_LITERAL_SYMBOLS.contains(first_symbol);
}

fn is_current_cell_lexeme(lexeme: &String) -> bool {
    let first_symbol = lexeme.chars().next().unwrap();
    return CURRENT_CELL_SYMBOLS.contains(first_symbol);
}

struct Scanner {
    lexemes: Vec<String>,
    partial_lexeme: String,
}

impl Scanner {
    fn new() -> Self {
        return Scanner {
            lexemes: vec![],
            partial_lexeme: String::new(),
        }
    }

    fn add_partial_as_lexeme(self: &mut Self) {
        if self.partial_lexeme.is_empty() == false {
            self.lexemes.push(self.partial_lexeme.clone());
            self.partial_lexeme = String::new();
        }
    }

    fn add_lexeme(self: &mut Self, lexeme: String) {
        if lexeme.is_empty() == false {
            self.lexemes.push(lexeme);
        }
    }

    fn add_to_partial_lexeme(self: &mut Self, chr: char) {
        self.partial_lexeme.push(chr);
    }
}

fn scan_code(code: &Vec<char>) -> Vec<String> {
    let mut scanner = Scanner::new();

    let mut last_chr = ' ';
    for chr in code {
        // These two lexemes can only ever be a single character long
        if COMMAND_SYMBOLS.contains(*chr) || CURRENT_CELL_SYMBOLS.contains(*chr) {
            scanner.add_partial_as_lexeme();
            scanner.add_lexeme(chr.to_string());
        } else if NUMERIC_LITERAL_SYMBOLS.contains(*chr) {
            if NUMERIC_LITERAL_SYMBOLS.contains(last_chr) == false {
                scanner.add_partial_as_lexeme();
            }

            scanner.add_to_partial_lexeme(*chr);
        }

        last_chr = *chr;
    }

    scanner.add_partial_as_lexeme();

    return scanner.lexemes;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Command { value: char },
    IntegerLiteral { value: u8 },  // TODO: How big of integer?
    CurrentCellReference,
}

impl Token {
    pub fn from_lexeme(lexeme: String) -> Self {
        if is_command_lexeme(&lexeme) {
            let first_char = lexeme.chars().next().unwrap();
            return Token::Command { value: first_char };
        } else if is_numeric_literal_lexeme(&lexeme) {
            let parsed: u8 = lexeme.parse().expect(format!("Could not parse {lexeme} as integer literal").as_str());
            return Token::IntegerLiteral { value: parsed };
        } else if is_current_cell_lexeme(&lexeme) {
            return Token::CurrentCellReference;
        } else {
            panic!("Unknown lexeme: {lexeme}");
        }
    }
}

fn evaluate_lexemes(lexemes: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for lexeme in lexemes {
        let token = Token::from_lexeme(lexeme);
        tokens.push(token);
    }

    return tokens;
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Value {
    CurrentCell,
    Number(u8),
}

impl Value {
    pub fn determine_value(self, current_cell_value: u8) -> u8 {
        return match self {
            Value::Number(n) => n,
            Value::CurrentCell => current_cell_value,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Command {
    symbol: char,
    value: Option<Value>,
}

impl Command {
    pub fn has_value(self: &Self) -> bool {
        return self.value.is_some();
    }

    pub fn get_defaulted_value(self: &Self) -> Value {
        return self.value.unwrap_or(Value::Number(1));
    }
}

fn parse_tokens(tokens: Vec<Token>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    let mut command_symbol: Option<char> = None;
    for token in tokens {
        match token {
            Token::Command { value } => {
                if let Some(existing_symbol) = command_symbol {
                    commands.push(Command { symbol: existing_symbol, value: None });
                }

                command_symbol = Some(value);
            }
            Token::IntegerLiteral { value } => {
                match command_symbol {
                    Some(symbol) => {
                        commands.push(Command { symbol: symbol, value: Some(Value::Number(value)) });
                        command_symbol = None;
                    }
                    None => {
                        panic!("Integer literal {value} must come after a command.")
                    }
                }
            }
            Token::CurrentCellReference => {
                match command_symbol {
                    Some(symbol) => {
                        commands.push(Command { symbol: symbol, value: Some(Value::CurrentCell) });
                        command_symbol = None;  // TODO: How to prevent all this duplication?
                    }
                    None => {
                        panic!("\"V\" must come after a command.")
                    }
                }
            }
        }
    }

    if let Some(command_symbol) = command_symbol {
        commands.push(Command { symbol: command_symbol, value: None });
    }

    return commands;
}

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
pub enum CellMoveOperator {
    Left,
    Right,
    Set,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    ApplyOperatorToCell { operator: MathOperator, value: Value },
    ApplyOperatorToCellPtr { operator: CellMoveOperator, value: Value },
    JumpToIf { position: usize, operator: EqualityOperator, match_value: u8 },
    PrintOut,
    ReadIn,
    SetCell { value: Value },
    Breakpoint,
}

// impl Display for Instruction {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let output = match self {
//             Instruction::ApplyOperatorToCell { operator, value } => format!("Cell <{operator}> {value}"),
//             Instruction::ApplyOperatorToCellPtr { operator, value } => format!("Move slots by {offset} {direction}"),
//             Instruction::JumpToIf { position, operator, match_value } => format!("Jump to {position} when value {operator} {match_value}"),
//             Instruction::PrintOut => "Print".to_string(),
//             Instruction::ReadIn => "Read".to_string(),
//             Instruction::SetCell { value} => format!("Set Cell to {value}"),
//             Instruction::Breakpoint => "Breakpoint".to_string(),
//         };
//
//         return write!(f, "{}", output);
//     }
// }

fn find_loop_indices(commands: &Vec<Command>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut start_to_end: HashMap<usize, usize> = HashMap::new();
    let mut end_to_start: HashMap<usize, usize> = HashMap::new();

    let mut loop_start_stack = vec![];

    for (i, token) in commands.iter().enumerate() {
        let symbol = token.symbol;
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

fn assert_valueless(command: Command) {
    if command.has_value() {
        panic!("Command {:?} cannot be given a value. Given {:?}.", command.symbol, command.value);
    }
}

fn compile_commands_to_intermediate(commands: Vec<Command>, allow_debugging: bool) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let (start_to_end, end_to_start) = find_loop_indices(&commands);
    for (i, command) in commands.iter().enumerate() {
        if VALUELESS_COMMAND_SYMBOLS.contains(command.symbol) {
            assert_valueless(*command);
        }

        let defaulted_value = command.get_defaulted_value();
        let instruction = match command.symbol {
            '+' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: defaulted_value }),
            '-' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Subtraction, value: defaulted_value }),
            '*' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: defaulted_value }),
            '/' => Some(Instruction::ApplyOperatorToCell { operator: MathOperator::Division, value: defaulted_value }),
            '<' => Some(Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Left, value: defaulted_value }),
            '>' => Some(Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Right, value: defaulted_value }),
            '@' => Some(Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Set, value: defaulted_value }),
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
    let lexemes = scan_code(&code_vec);
    let tokens = evaluate_lexemes(lexemes);
    let commands = parse_tokens(tokens);
    let instructions = compile_commands_to_intermediate(commands, allow_debugging);
    return instructions;
}

#[cfg(test)]
mod tests {
    mod scan_code {
        use super::super::*;

        #[test]
        fn it_should_produce_the_correct_lexemes() {
            let code = vec!['+', ' ', 'V', '1', '+', '2', '3', '+', '4', ' '];
            let lexemes = scan_code(&code);

            assert_eq!(lexemes, vec!["+", "V", "1", "+", "23", "+", "4"]);
        }

        #[test]
        fn it_should_split_consecutive_commands_in_a_row_as_different_lexemes() {
            let code = vec!['+', '+', '-', '-'];
            let lexemes = scan_code(&code);

            assert_eq!(lexemes, vec!["+", "+", "-", "-"]);
        }
    }

    mod evaluate_lexemes {
        use super::super::*;

        #[test]
        fn it_should_produce_the_correct_tokens() {
            let lexemes = vec![String::from("+"), String::from("123"), String::from("-"), String::from("V")];
            let tokens = evaluate_lexemes(lexemes);

            assert_eq!(tokens, vec![
                Token::Command { value: '+' },
                Token::IntegerLiteral { value: 123 },
                Token::Command { value: '-' },
                Token::CurrentCellReference,
            ]);
        }

        #[test]
        #[should_panic]
        fn it_should_panic_if_an_unknown_lexeme_is_passed() {
            let lexemes = vec![String::from("|")];
            evaluate_lexemes(lexemes);
        }
    }

    mod parse_tokens {
        use super::super::*;

        #[test]
        fn it_should_produce_the_correct_commands() {
            let tokens = vec![
                Token::Command { value: '+' },
                Token::Command { value: '+' },
                Token::IntegerLiteral { value: 123 },
                Token::Command { value: '-' },
                Token::CurrentCellReference,
            ];
            let commands = parse_tokens(tokens);

            assert_eq!(commands, vec![
                Command { symbol: '+', value: None },
                Command { symbol: '+', value: Some(Value::Number(123)) },
                Command { symbol: '-', value: Some(Value::CurrentCell) },
            ]);
        }
    }

    mod compile_to_intermediate {
        use super::super::*;

        #[test]
        fn it_should_ignore_invalid_characters() {
            let code = "+None of this should be considered*";
            let instructions = compile_to_intermediate(code, false);

            assert_eq!(instructions.len(), 2);

            assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(1) });
            assert_eq!(instructions[1], Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: Value::Number(1) });
        }

        #[test]
        fn it_should_produce_the_correct_instruction_for_each_token() {
            let code = "[]+-*/<>@.,^";
            let instructions = compile_to_intermediate(code, false);

            assert_eq!(instructions.len(), 12);

            assert_eq!(instructions[0], Instruction::JumpToIf { position: 1, operator: EqualityOperator::Equal, match_value: 0 });
            assert_eq!(instructions[1], Instruction::JumpToIf { position: 0, operator: EqualityOperator::NotEqual, match_value: 0 });

            assert_eq!(instructions[2], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(1) });
            assert_eq!(instructions[3], Instruction::ApplyOperatorToCell { operator: MathOperator::Subtraction, value: Value::Number(1) });
            assert_eq!(instructions[4], Instruction::ApplyOperatorToCell { operator: MathOperator::Multiplication, value: Value::Number(1) });
            assert_eq!(instructions[5], Instruction::ApplyOperatorToCell { operator: MathOperator::Division, value: Value::Number(1) });
            assert_eq!(instructions[6], Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Left, value: Value::Number(1) });
            assert_eq!(instructions[7], Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Right, value: Value::Number(1) });
            assert_eq!(instructions[8], Instruction::ApplyOperatorToCellPtr { operator: CellMoveOperator::Set, value: Value::Number(1) });
            assert_eq!(instructions[9], Instruction::PrintOut);
            assert_eq!(instructions[10], Instruction::ReadIn);
        }

        #[test]
        fn it_should_properly_read_instruction_values_and_default_missing_ones_to_one() {
            let code = "++1+2+3+40+200";
            let instructions = compile_to_intermediate(code, false);

            assert_eq!(instructions.len(), 6);

            assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(1) });
            assert_eq!(instructions[1], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(1) });
            assert_eq!(instructions[2], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(2) });
            assert_eq!(instructions[3], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(3) });
            assert_eq!(instructions[4], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(40) });
            assert_eq!(instructions[5], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::Number(200) });
        }

        #[test]
        fn it_should_properly_add_insertion_values() {
            let code = "+V";
            let instructions = compile_to_intermediate(code, false);

            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0], Instruction::ApplyOperatorToCell { operator: MathOperator::Addition, value: Value::CurrentCell });
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


}