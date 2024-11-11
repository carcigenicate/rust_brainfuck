mod standard_brainfuck;
mod ezfuck;

fn main() {
    let instructions = ezfuck::parser::parser::compile_to_intermediate(
        "+-><"
    );

    // println!("{:?}", instructions.iter().map(|inst| inst.to_string()).collect::<Vec<String>>().join(", "));

    for (i, inst) in instructions.iter().enumerate() {
        println!("{i}: {inst:?}");
    }

    ezfuck::interpreter::interpreter::interpret_with_std_io(&instructions);
}
