use std::io;
use std::io::{BufRead, Read, Write};
use strum_macros::Display;
use crate::ezfuck::parser::parser::{Instruction, EqualityOperator, MathOperator, InstructionValue, Direction};

#[derive(Clone, Debug)]
pub struct ExecutionState {
    pub cells: Vec<u8>,
    pub cell_ptr: usize,
    pub instruction_ptr: usize,
}

impl ExecutionState {
    pub fn new() -> ExecutionState {
        return ExecutionState {
            cell_ptr: 0,
            instruction_ptr: 0,
            cells: vec![0],
        };
    }

    pub fn set_instruction_pointer(self: &mut Self, ptr: usize) {
        self.instruction_ptr = ptr;
    }

    pub fn get_current_cell(self: &Self) -> u8 {
        return self.cells[self.cell_ptr];
    }

    pub fn set_current_cell(self: &mut Self, new_value: u8) -> () {
        self.cells[self.cell_ptr] = new_value;
    }

    pub fn set_cell_pointer(self: &mut Self, ptr: usize) {
        self.ensure_cell(ptr);
        self.cell_ptr = ptr;
    }

    fn ensure_cell(self: &mut Self, ptr: usize) -> () {
        let needed = (ptr as isize) - (self.cells.len() as isize) + 1;
        if needed > 0 {
            for _ in 0..needed {
                self.cells.push(0);
            }
        }
    }
}

fn apply_math_operator(current_cell_value: u8, operator: MathOperator, value: u8) -> u8 {
    return match operator {
        MathOperator::Addition => current_cell_value + value,
        MathOperator::Subtraction => current_cell_value - value,
        MathOperator::Multiplication => current_cell_value * value,
        MathOperator::Division => current_cell_value / value,  // TODO: Ensure is floor division
    }
}

fn add_cell_ptr_value(current_cell_ptr: usize, ptr_offset: isize) -> usize {
    return match current_cell_ptr.checked_add_signed(ptr_offset) {
        Some(added) => added,
        None => panic!("Cell Pointer Became Negative!")
    };
}

fn print_value<W: Write>(out_stream: &mut W, cell: u8) {
    write!(out_stream, "{}", char::from(cell)).unwrap();
    io::stdout().flush().unwrap();
}

fn read_value<R: BufRead>(in_stream: &mut R) -> u8 {
    let mut input = [0; 1];
    in_stream.read_exact(&mut input).expect("Reading byte from stdin");
    return input[0];
}

pub fn interpret_instruction<R: BufRead, W: Write>(instruction: Instruction, state: &mut ExecutionState, in_stream: &mut R, out_stream: &mut W) -> () {
    match instruction {
        Instruction::ApplyOperatorToCell { operator, value } => {
            let actual_value = value.determine_value(state.get_current_cell());
            let new_cell_value = apply_math_operator(state.get_current_cell(), operator, actual_value);
            state.set_current_cell(new_cell_value);
        }

        Instruction::AddToCellPtr { direction, offset } => {
            let abs_offset = offset.determine_value(state.get_current_cell());
            let signed_offset = if direction == Direction::Left { abs_offset as isize * -1 } else { abs_offset as isize };
            let new_cell_ptr = add_cell_ptr_value(state.cell_ptr, signed_offset);
            state.set_cell_pointer(new_cell_ptr);
        }

        Instruction::JumpToIf { position, operator, match_value } => {
            match operator {
                EqualityOperator::Equal => {
                    if state.get_current_cell() == match_value {
                        state.set_instruction_pointer(position);
                    }
                },
                EqualityOperator::NotEqual => {
                    if state.get_current_cell() != match_value {
                        state.set_instruction_pointer(position);
                    }
                }
            }
        }

        Instruction::PrintOut => {
            print_value(out_stream, state.get_current_cell());
        }

        Instruction::ReadIn => {
            let input = read_value(in_stream);
            state.set_current_cell(input);
        }

        Instruction::SetCell { value } => {
            let actual_value = value.determine_value(state.get_current_cell());
            state.set_current_cell(actual_value);
        }
    }
}

pub fn interpret<R: BufRead, W: Write>(instructions: &Vec<Instruction>, state: &mut ExecutionState, in_stream: &mut R, out_stream: &mut W) -> () {
    while state.instruction_ptr < instructions.len() {
        let current_instruction = instructions[state.instruction_ptr];

        interpret_instruction(current_instruction, state, in_stream, out_stream);

        state.instruction_ptr += 1;
    }
}

pub fn interpret_with_std_io(instructions: &Vec<Instruction>) {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut stdout = io::stdout();

    let mut state = ExecutionState::new();

    interpret(instructions, &mut state, &mut input, &mut stdout);
}

#[cfg(test)]
mod tests {
    use crate::ezfuck::parser::parser::compile_to_intermediate;
    use super::*;

    fn interpret_and_collect_output(instructions: &Vec<Instruction>, state: &mut ExecutionState, input: &[u8]) -> String {
        let mut input = &input[..];
        let mut output = vec![];

        interpret(&instructions, state, &mut input, &mut output);

        let output_string = String::from_utf8(output).unwrap();
        return output_string;
    }

    fn interpret_instruction_and_collect_output(instruction: Instruction, state: &mut ExecutionState, input: &[u8]) -> String {
        let mut input = &input[..];
        let mut output = vec![];


        interpret_instruction(instruction, state, &mut input, &mut output);

        let output_string = String::from_utf8(output).unwrap();
        return output_string;
    }

    #[test]
    fn it_should_add_to_the_current_cell() {
        let instruction = Instruction::ApplyOperatorToCell {
            operator: MathOperator::Addition,
            value: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cells, vec![5]);
    }

    #[test]
    fn it_should_subtract_from_the_current_cell() {
        let instruction = Instruction::ApplyOperatorToCell {
            operator: MathOperator::Subtraction,
            value: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(20);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cells, vec![15]);
    }

    #[test]
    fn it_should_multiply_the_current_cell() {
        let instruction = Instruction::ApplyOperatorToCell {
            operator: MathOperator::Multiplication,
            value: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(10);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cells, vec![50]);
    }

    #[test]
    fn it_should_divide_the_current_cell() {
        let instruction = Instruction::ApplyOperatorToCell {
            operator: MathOperator::Division,
            value: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(50);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cells, vec![10]);
    }

    #[test]
    fn it_should_jump_the_instruction_pointer_if_equal() {
        let instruction = Instruction::JumpToIf {
            operator: EqualityOperator::Equal,
            match_value: 10,
            position: 5,
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(10);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.instruction_ptr, 5);
    }

    #[test]
    fn it_should_not_jump_the_instruction_pointer_if_not_equal() {
        let instruction = Instruction::JumpToIf {
            operator: EqualityOperator::Equal,
            match_value: 100,
            position: 5,
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(10);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.instruction_ptr, 0);
    }

    #[test]
    fn it_should_jump_the_instruction_pointer_if_not_equal() {
        let instruction = Instruction::JumpToIf {
            operator: EqualityOperator::NotEqual,
            match_value: 10,
            position: 5,
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(5);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.instruction_ptr, 5);
    }

    #[test]
    fn it_should_move_the_instruction_pointer_to_the_left() {
        let instruction = Instruction::AddToCellPtr {
            direction: Direction::Left,
            offset: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        state.set_cell_pointer(20);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cell_ptr, 15);
    }

    #[test]
    fn it_should_set_the_current_cell() {
        let instruction = Instruction::SetCell {
            value: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cells, vec![5]);
    }

    #[test]
    fn it_should_move_the_instruction_pointer_to_the_right() {
        let instruction = Instruction::AddToCellPtr {
            direction: Direction::Right,
            offset: InstructionValue::Number(5),
        };

        let mut state = ExecutionState::new();
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.cell_ptr, 5);
    }

    #[test]
    fn it_should_not_jump_the_instruction_pointer_if_equal() {
        let instruction = Instruction::JumpToIf {
            operator: EqualityOperator::NotEqual,
            match_value: 10,
            position: 5,
        };

        let mut state = ExecutionState::new();
        state.set_current_cell(10);
        interpret_instruction_and_collect_output(instruction, &mut state, b"");
        assert_eq!(state.instruction_ptr, 0);
    }

    #[test]
    fn it_should_print_hello_world() {
        // TODO: Find a more isolated, clean way of doing this test without relying on the parser
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let instructions = compile_to_intermediate(code);

        let mut state = ExecutionState::new();
        let output_string = interpret_and_collect_output(&instructions, &mut state, b"");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn it_should_print_hello_world_using_values() {
        let code = "+8[>+4[>+2>+3>+3>+<4-]>+>+>->2+[<]<-]>2.>-3.+7..+3.>2.<-.<.+3.-6.-8.>2+.>+2.";
        let instructions = compile_to_intermediate(code);

        let mut state = ExecutionState::new();
        let output_string = interpret_and_collect_output(&instructions, &mut state, b"");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn it_should_set_cell_value_using_extraction() {
        let code = "^65 .";
        let instructions = compile_to_intermediate(code);

        let mut state = ExecutionState::new();
        let output_string = interpret_and_collect_output(&instructions, &mut state, b"");
        assert_eq!(output_string, "A");
    }

/*    #[test]
    fn it_should_get_cell_value_using_insertion() {
        let code = "^2 >V ";  // TODO: How to test?
        let instructions = compile_to_intermediate(code);

        let output_string = interpret_and_collect_output(&instructions, b"");
        assert_eq!(output_string, "A");
    }*/

    #[test]
    fn it_should_properly_parse_concurrent_insertions() {
        let code = "^^65 .";
        let instructions = compile_to_intermediate(code);

        let mut state = ExecutionState::new();
        let output_string = interpret_and_collect_output(&instructions, &mut state, b"");
        assert_eq!(output_string, "A");
    }

    // TODO: Cell value wrapping behavior
}