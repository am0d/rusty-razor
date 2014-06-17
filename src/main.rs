#![feature(phase)]
#[phase(plugin)] extern crate my_debug;

#[phase(plugin, link)] extern crate debug;

extern crate collections;

use std::io;
use std::io::File;

mod lexer;
mod parser;
mod token;
mod view_writer;

fn get_file_contents() -> Result<String, io::IoError> {
    let path = from_str::<Path>("test/index.rs.html").unwrap();

    File::open(&path).and_then(|file| {
        let mut file = file;
        file.read_to_str()
    })
}

fn main() {
    let contents = match get_file_contents() {
        Ok(contents) => contents,
        Err(e) => fail!(e.to_str())
    };

    let mut parser = parser::Parser::new(contents.as_slice());
    parser.parse();

    //for section in parser.sections.iter() {
    //    println!("{}", section);
    //}

    view_writer::write_view(&Path::new("test/index.rs"), &parser.sections);
}
