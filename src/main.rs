use std::io;
use clap::Parser;
use crate::ezfuck::repl::repl::start_repl;

mod standard_brainfuck;
mod ezfuck;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

fn interpret_string(code: &str, allow_debugging: bool) -> () {
    let instructions = ezfuck::parser::parser::compile_to_intermediate(code, allow_debugging);
    ezfuck::interpreter::interpreter::interpret_with_std_io(&instructions, allow_debugging);
}

fn main() {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(code) => {
                    interpret_string(code.as_str(), true);
                }
                Err(err) => {
                    eprintln!("Could not read file: {err}");
                }
            }
        }
        None => {
            let stdin = io::stdin();
            let mut input = stdin.lock();

            let mut stdout = io::stdout();

            start_repl(&mut input, &mut stdout);
        }
    }
}
