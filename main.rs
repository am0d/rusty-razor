use std::io;
use std::io::File;

mod lexer;

fn prelude () -> ~str {
    ~"extern mod extra;
extern mod http;

use super::super::{View, SafeHtmlString};"
}

fn main() {
    let path = from_str::<Path>("src/compiler/index.rs.html").unwrap();

    let file = io::result(|| {File::open(&path)});
    let contents = match file {
        Ok(mut reader) => {
            reader.read_to_end()
        }
        Err(e) => {
            fail!("Error reading file: {}", e.to_str())
        }
    };

    let contents = std::str::from_utf8_owned(contents);

    let mut lexer = lexer::Lexer::new();
    lexer.lex(contents);

    println(prelude());

    println!("Lines: {}\nLast column: {}", lexer.line, lexer.column);

    for section in lexer.sections.iter() {
        match section {
            &lexer::Html(ref s) => {
                println!("Html({})", *s);
            },
            &lexer::Rust(ref s) => {
                println!("Rust({})", *s);
            },
            _ => {}
        }
    }
}
