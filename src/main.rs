mod standard_brainfuck;


fn main() {
    let instructions = standard_brainfuck::parser::parser::parse(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    );

    // println!("{:?}", instructions.iter().map(|inst| inst.to_string()).collect::<Vec<String>>().join(", "));

    for (i, inst) in instructions.iter().enumerate() {
        println!("{i}: {inst:?}");
    }

    standard_brainfuck::interpreter::interpreter::interpret_with_std_io(&instructions);
}
