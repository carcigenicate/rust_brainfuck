use std::io;
use std::io::{BufRead, Read, Write};
use crate::ezfuck::parser::parser::{Instruction, EqualityOperator, MathOperator, InstructionValue, Direction};

fn ensure_cell(cells: &mut Vec<u8>, slot_i: usize) -> () {
    let needed = (slot_i as isize) - (cells.len() as isize) + 1;
    if needed > 0 {
        for _ in 0..needed {
            cells.push(0);
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

pub fn interpret<R: BufRead, W: Write>(instructions: &Vec<Instruction>, cells: &mut Vec<u8>, initial_cell_ptr: usize, in_stream: &mut R, out_stream: &mut W) -> usize {
    let mut instruction_ptr = 0;
    let mut cell_ptr = initial_cell_ptr;

    while instruction_ptr < instructions.len() {
        let current_instruction = instructions[instruction_ptr];

        match current_instruction {
            Instruction::ApplyOperatorToCell { operator, value } => {
                let actual_value = value.determine_value(cells[cell_ptr]);
                let new_cell_value = apply_math_operator(cells[cell_ptr], operator, actual_value);
                cells[cell_ptr] = new_cell_value;
            }

            Instruction::AddToCellPtr { direction, offset } => {
                let abs_offset = offset.determine_value(cells[cell_ptr]);
                let signed_offset = if direction == Direction::Left { abs_offset as isize * -1 } else { abs_offset as isize };
                let new_cell_ptr = add_cell_ptr_value(cell_ptr, signed_offset);
                ensure_cell(cells, new_cell_ptr);
                cell_ptr = new_cell_ptr;
            }

            Instruction::JumpToIf { position, operator, match_value } => {
                match operator {
                    EqualityOperator::Equal => {
                        if cells[cell_ptr] == match_value {
                            instruction_ptr = position
                        }
                    },
                    EqualityOperator::NotEqual => {
                        if cells[cell_ptr] != match_value {
                            instruction_ptr = position
                        }
                    }
                }
            }

            Instruction::PrintOut => {
                print_value(out_stream, cells[cell_ptr]);
            }

            Instruction::ReadIn => {
                let input = read_value(in_stream);
                cells[cell_ptr] = input;
            }

            Instruction::SetCell { value } => {
                let actual_value = value.determine_value(cells[cell_ptr]);
                cells[cell_ptr] = actual_value;
            }
        }

        instruction_ptr += 1;
        // println!("Cell Ptr: {cell_ptr}, Inst Ptr: {instruction_ptr} Cells: {cells:?}");
    }

    return cell_ptr;
}

pub fn interpret_with_std_io(instructions: &Vec<Instruction>) {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut stdout = io::stdout();

    let mut cells = vec![0];

    interpret(instructions, &mut cells, 0, &mut input, &mut stdout);
}

#[cfg(test)]
mod tests {
    use crate::ezfuck::parser::parser::compile_to_intermediate;
    use super::*;

    fn interpret_and_collect_output(instructions: &Vec<Instruction>, input: &[u8]) -> String {
        let mut input = &input[..];
        let mut output = vec![];

        let mut cells = vec![0];

        interpret(&instructions, &mut cells, 0, &mut input, &mut output);

        let output_string = String::from_utf8(output).unwrap();
        return output_string;
    }

    #[test]
    fn it_should_print_hello_world() {
        // TODO: Find a more isolated, clean way of doing this test without relying on the parser
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let instructions = compile_to_intermediate(code);

        let output_string = interpret_and_collect_output(&instructions, b"");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn it_should_print_hello_world_using_values() {
        let code = "+8[>+4[>+2>+3>+3>+<4-]>+>+>->2+[<]<-]>2.>-3.+7..+3.>2.<-.<.+3.-6.-8.>2+.>+2.";
        let instructions = compile_to_intermediate(code);

        let output_string = interpret_and_collect_output(&instructions, b"");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn it_should_set_cell_value_using_extraction() {
        let code = "^65 .";
        let instructions = compile_to_intermediate(code);

        let output_string = interpret_and_collect_output(&instructions, b"");
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

        let output_string = interpret_and_collect_output(&instructions, b"");
        assert_eq!(output_string, "A");
    }

    // TODO: Cell value wrapping behavior
}