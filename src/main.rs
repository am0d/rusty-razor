/* Copyright 2014 Damien Schoof

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

extern crate collections;

use std::io;
use std::io::File;

mod lexer;
mod parser;
mod token;
mod view_writer;

fn get_file_contents() -> Result<~str, io::IoError> {
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

    let mut lexer = lexer::Lexer::new(contents);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.parse();

    println!("Lines: {}\nLast column: {}", parser.lexer.line, parser.lexer.column);

    //debug!("{}", parser.sections);

    view_writer::write_view(&Path::new("test/index.rs"), &parser.sections);
}
