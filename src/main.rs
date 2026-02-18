use rlox::token_type;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", token_type::TokenType::Eof);
    for a in env::args() {
        println!("{a}");
    }
    if args.len() == 2 {
        run_file(&args[1]);
    } else if args.len() == 1 {
        run_prompt();
    } else {
        println!("Usage: jlox [script]")
    }
}

fn run_file(path: &str) {
    println!("run: {}", path);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    run(contents);
}

fn run_prompt() {
    println!("run prompt");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        run(line);
    }
}

fn run(source: String) {
    println!("source: {}", source)
}
