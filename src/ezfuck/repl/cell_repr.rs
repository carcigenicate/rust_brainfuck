use std::cmp::max;

pub fn produce_cells_repr(cells: &Vec<u8>, cell_ptr: usize) -> String {
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