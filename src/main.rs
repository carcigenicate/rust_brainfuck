mod standard_brainfuck;
mod ezfuck;

fn main() {
    let instructions = ezfuck::parser::parser::compile_to_intermediate(
        "+8[>+4[>+2>+3>+3>+<4-]>+>+>->2+[<]<-]>2.>-3.+7..+3.>2.<-.<.+3.-6.-8.>2+.>+2."
    );

    // for (i, inst) in instructions.iter().enumerate() {
    //     println!("{i}: {inst:?}");
    // }

    ezfuck::interpreter::interpreter::interpret_with_std_io(&instructions);
}
