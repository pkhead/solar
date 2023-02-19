use std::io::Read;
use std::collections::VecDeque;
use tokens::{Token, Keyword};

pub mod tokens;
pub mod parser;

pub fn run(file: &mut std::fs::File) -> Result<(), std::io::Error> {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut tokens = Vec::new();
    tokens::read_tokens(&file_contents, &mut tokens);

    for (_i, tok) in tokens.iter().enumerate() {
        match tok {
            Token::Symbol(v) => println!("SYMBOL {}", v),
            Token::Keyword(v) => println!("KEYWORD {}", v),
            Token::Identifier(v) => println!("IDENTIFIER {}", v),
            Token::Number(v) => println!("NUMBER {}", v),
            Token::String(v) => println!("STRING {}", v)
        }
    }

    let mut token_queue = VecDeque::from(tokens);

    match parser::parse_ast(&mut token_queue) {
        Ok(ast) => {
            println!("success");
            dbg!(ast);
        },
        Err(err) => {
            println!("error: {}", err.msg);
        }
    };

    Ok(())
}
