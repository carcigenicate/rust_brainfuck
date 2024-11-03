use std::io;
use std::io::{BufRead, Read, Write};
use crate::standard_brainfuck::parser::parser::{Instruction, EqualityOperator};

fn ensure_cell(cells: &mut Vec<u8>, slot_i: usize) -> () {
    let needed = (slot_i as isize) - (cells.len() as isize) + 1;
    if needed > 0 {
        for _ in 0..needed {
            cells.push(0);
        }
    }
}

fn add_cell_value(current_cell_value: u8, add_value: i8) -> u8 {
    return current_cell_value.wrapping_add_signed(add_value);
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

pub fn interpret<R: BufRead, W: Write>(instructions: &Vec<Instruction>, in_stream: &mut R, out_stream: &mut W) {
    let mut instruction_ptr = 0;
    let mut cell_ptr = 0;

    let mut cells: Vec<u8> = vec![0];

    while instruction_ptr < instructions.len() {
        let current_instruction = instructions[instruction_ptr];

        match current_instruction {
            Instruction::AddToCell { n } => {
                let new_cell_value = add_cell_value(cells[cell_ptr], n);
                cells[cell_ptr] = new_cell_value;
            }

            Instruction::AddToCellPtr { offset } => {
                let new_cell_ptr = add_cell_ptr_value(cell_ptr, offset);
                ensure_cell(&mut cells, new_cell_ptr);
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
        }

        instruction_ptr += 1;
        // println!("Cell Ptr: {cell_ptr}, Inst Ptr: {instruction_ptr} Cells: {cells:?}");
    }
}

pub fn interpret_with_std_io(instructions: &Vec<Instruction>) {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut stdout = io::stdout();

    interpret(instructions, &mut input, &mut stdout);
}

#[cfg(test)]
mod tests {
    use crate::standard_brainfuck::parser::parser::parse;
    use super::*;

    #[test]
    fn it_should_print_hello_world() {
        // TODO: Find a more isolated, clean way of doing this test without relying on the parser
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let instructions = parse(code);

        let mut input = &b""[..];
        let mut output = vec![];
        interpret(&instructions, &mut input, &mut output);

        let output_string = String::from_utf8(output).unwrap();
        assert_eq!(output_string, "Hello World!\n");
    }
}