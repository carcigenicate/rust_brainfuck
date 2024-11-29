use std::cmp::max;
use std::io::{BufRead, Read, Write};
use crate::ezfuck::interpreter::interpreter::{interpret, ExecutionState};
use crate::ezfuck::parser::parser::{compile_to_intermediate};

fn produce_cells_repr(cells: &Vec<u8>, cell_ptr: usize) -> String {
    let mut last_i = cells.iter().rposition(|cell| *cell != 0).unwrap_or(0);
    last_i = max(last_i, cell_ptr);

    return if cells.len() > 0 {
        let mut ptr_row: String = String::from("  ");
        let mut index_row: String = String::from("i ");
        let mut raw_row: String = String::from("d ");
        let mut ascii_row: String = String::from("a ");

        for i in 0..=last_i {
            let cell_value = cells[i];
            let cell_ascii = if cell_value >= 32 { cell_value as char } else { ' ' };
            let ptr_repr = if i == cell_ptr { "   V  " } else { "      " };

            ptr_row.push_str(ptr_repr);
            index_row.push_str(format!("| {i:0>3} ").as_str());
            raw_row.push_str(format!("| {cell_value:0>3} ").as_str());
            ascii_row.push_str(format!("|  {cell_ascii}  ").as_str());
        }

        format!("{ptr_row}\n{index_row}|\n{raw_row}|\n{ascii_row}|\n")
    } else {
        String::new()
    }
}

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
            let instructions = compile_to_intermediate(&input_buffer);

            out_stream.write(b"Output: ").unwrap();
            interpret(&instructions, &mut state, in_stream, out_stream);
            state.set_instruction_pointer(0);

            out_stream.write(b"\n").unwrap();
        }
    }
}

