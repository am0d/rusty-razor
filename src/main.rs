#![feature(phase)]
#[phase(plugin)] extern crate my_debug;

#[phase(plugin, link)] extern crate debug;

extern crate collections;
extern crate getopts;

use std::io;
use getopts::{optopt,getopts, OptGroup, short_usage, usage};
use std::os;
use std::io::File;

mod lexer;
mod parser;
mod token;
mod view_writer;

fn get_file_contents(file_path: &str) -> Result<String, io::IoError> {
    let path = from_str::<Path>(file_path).unwrap();

    File::open(&path).and_then(|file| {
        let mut file = file;
        file.read_to_str()
    })
}

fn output_file_from_input(input_file_path: &str) -> Path {
    let input_path = from_str::<Path>(input_file_path).unwrap();

    if input_file_path.ends_with(".rs.html") {
        from_str::<Path>(input_file_path.slice_to(input_file_path.len() - 5)).unwrap()
    } else {
        input_path.with_extension(".rs")
    }
}

fn print_usage(program_name: &str, options: &[OptGroup]) {
    println!("{}", usage(short_usage(program_name, options).as_slice(), options));
}

fn main() {
    let opts = [
        optopt("o", "out", "Filename of the generated rust code", "")
    ];
    let args: Vec<String> = os::args().iter()
                                .map(|a| a.to_string())
                                .collect();
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(e) => fail!("Unable to get options: {}", e)
    };

    if matches.free.is_empty() {
        print_usage(args.get(0).as_slice(), opts);
        return;
    }

    let input_file_name = matches.free.get(0).as_slice();
    let output_file_name = match matches.opt_str("o") {
        Some(ofn) => from_str::<Path>(ofn.as_slice()).unwrap(),
        None => output_file_from_input(input_file_name)
    };

    let contents = match get_file_contents(input_file_name) {
        Ok(contents) => contents,
        Err(e) => fail!(e.to_str())
    };

    let mut parser = parser::Parser::new(contents.as_slice());
    parser.parse();

    //for section in parser.sections.iter() {
    //    println!("{}", section);
    //}

    view_writer::write_view(&output_file_name, &parser.sections);
}
