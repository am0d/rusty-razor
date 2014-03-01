use std::io;
use std::io::File;

mod lexer;
mod parser;
mod token;

fn prelude () -> ~str {
    ~"extern mod extra;
extern mod http;

use super::super::{View, SafeHtmlString};"
}

fn get_file_contents() -> Result<~[u8], io::IoError> {
    let path = from_str::<Path>("src/compiler/index.rs.html").unwrap();

    File::open(&path).and_then(|file| {
        let mut file = file;
        file.read_to_end()
    })
}

fn main() {
    let contents = match get_file_contents() {
        Ok(contents) => std::str::from_utf8_owned(contents).expect("Non-utf8 source file"),
        Err(e) => fail!(e)
    };

    let mut lexer = lexer::Lexer::new(contents);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.parse();

    println!("{}", prelude());

    println!("Lines: {}\nLast column: {}", parser.lexer.line, parser.lexer.column);

    for section in parser.sections.iter() {
        match section {
            &parser::Html(ref s) => {
                println!("Html({})", *s);
            },
            &parser::Rust(ref s) => {
                println!("Rust({})", *s);
            },
            _ => {}
        }
    }
}
