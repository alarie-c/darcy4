use lexer::Lexer;
use parser::Parser;
use std::{env, fs};

mod lexer;
mod parser;
mod token;

// split source code by lines
fn lines(source: &String) -> Vec<String> {
    source.split("\n").map(|x| x.to_owned()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // get file path
    if args.len() >= 2 {
        let source = match fs::read_to_string(&args[1]) {
            Ok(source) => source,
            Err(_) => panic!("[DY4] Error reading file content"),
        };

        let mut lexer = match Lexer::new(source.chars().peekable()) {
            Some(lexer) => lexer,
            None => std::process::exit(0),
        };

        lexer.scan();
        println!("{:#?}", lexer.output);

        let mut parser = Parser::new(&lexer.output);
        parser.parse();

        //println!("{}", source);
        //println!("\n\n\n{:#?}", lines(&source));
    } else {
        panic!("[DY4] File path not specified");
    }
}
