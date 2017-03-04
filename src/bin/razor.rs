extern crate getopts;
extern crate razor_codegen;

use std::fs::File;
use getopts::Options;
use std::env::args;
use std::path::PathBuf;

use ::razor_codegen::*;

fn print_usage(program_name: &str, options: Options) {
    println!("{}", options.usage(&options.short_usage(program_name)[..]));
}

fn main() {
    let mut opts = Options::new();
    opts.optopt("o", "out", "Filename of the generated rust code", "");
    let args: Vec<String> = args()
                                .map(|a| a.to_string())
                                .collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("Unable to get options: {}", e),
    };

    if matches.free.is_empty() {
        print_usage(&args[0][..], opts);
        return;
    }

    let input_file_name = &matches.free[0][..];
    let output_file_name = match matches.opt_str("o") {
        Some(ofn) => PathBuf::from(ofn),
        None => output_file_from_input(input_file_name),
    };

    let contents = match get_file_contents(input_file_name) {
        Ok(contents) => contents,
        Err(e) => panic!(e.to_string()),
    };

    let mut parser = Parser::new(&contents[..]);
    parser.parse();

    let result = File::create(output_file_name.as_path()).and_then(|mut file| {
    view_writer::write_view(&view_name(input_file_name)[..],
                            &mut file,
                            &parser.sections)
    });
    match result {
        Err(e) => println!("Error writing to file: {}", e),
        _ => (),
    };

}
