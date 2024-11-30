use std::cmp::max;
use std::io::{BufRead, Read, Write};

use crate::ezfuck::interpreter::interpreter::{interpret, ExecutionState};
use crate::ezfuck::parser::parser::{compile_to_intermediate};
use crate::ezfuck::repl::cell_repr::{produce_cells_repr};

pub fn start_repl<R: BufRead, W: Write>(in_stream: &mut R, out_stream: &mut W) {
    let mut state = ExecutionState::new();

    loop {
        let cells_repr = produce_cells_repr(&state.cells, state.cell_ptr);
        out_stream.write(cells_repr.as_bytes()).unwrap();
        out_stream.flush().unwrap();

        out_stream.write(b"EZ> ").unwrap();
        out_stream.flush().unwrap();

        let mut input_buffer: String = String::new();
        in_stream.read_line(&mut input_buffer).unwrap();

        if input_buffer.starts_with("!") {
            break;
        } else {
            let instructions = compile_to_intermediate(&input_buffer, false);

            out_stream.write(b"Output: ").unwrap();
            interpret(&instructions, &mut state, in_stream, out_stream, false);
            state.set_instruction_pointer(0);

            out_stream.write(b"\n").unwrap();
        }
    }
}

