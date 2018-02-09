extern crate getopts;
extern crate razor_codegen;

use std::fs::{self, File};
use getopts::Options;
use std::env::args;
use std::path::PathBuf;

use razor_codegen::*;

fn print_usage(program_name: &str, options: &Options) {
    println!("{}", options.usage(&options.short_usage(program_name)[..]));
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("d", "directory", "Directory to parse", "");
    let args: Vec<String> = args().map(|a| a.to_string()).collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("Unable to get options: {}", e),
    };

    match matches.opt_str("d") {
        Some(directory) => {
            let directory_path = PathBuf::from(directory);
            process_directory(&directory_path);
        }
        None => {
            if matches.free.is_empty() {
                print_usage(&args[0][..], &opts);
                return;
            }

            let input_file_name = PathBuf::from(&matches.free[0][..]);
            parse_and_write_file(&input_file_name);
        }
    };
}

fn process_directory(directory_path: &PathBuf) {
    if directory_path.is_dir() {
        for entry in fs::read_dir(directory_path).expect("Couldn't read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    process_directory(&path);
                } else if is_rusty_razor_file(&path) {
                    parse_and_write_file(&path);
                }
            }
        }
    }
}

fn parse_and_write_file(input_file_name: &PathBuf) {
    let output_file_name = output_file_from_input(input_file_name);

    let contents = match get_file_contents(input_file_name) {
        Ok(contents) => contents,
        Err(e) => panic!(e.to_string()),
    };

    let parser = Parser::new(&contents[..]);
    let sections = parser.parse();

    let result = File::create(output_file_name.as_path()).and_then(|mut file| {
        view_writer::write_view(&view_name(input_file_name)[..], &mut file, &sections)
    });
    if let Err(e) = result {
        println!("Error writing to file: {}", e);
    }
}
