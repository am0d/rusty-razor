extern crate getopts;

use std::io::Read;
use getopts::{Options};
use std::env::args;
use std::fs::File;
use std::path::{Path,PathBuf};

mod lexer;
mod parser;
mod token;
mod view_writer;

fn get_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(file_path);

    File::open(path).and_then(|file| {
        let mut file = file;
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        Ok(s)
    })
}

fn output_file_from_input(input_file_path: &str) -> PathBuf {
    let input_path = PathBuf::from(input_file_path);

    if input_file_path.ends_with(".rs.html") {
        PathBuf::from(&input_file_path[..input_file_path.len() - 5])
    } else {
        input_path.with_extension("rs")
    }
}

fn view_name(input_file_path: &str) -> String {
    let path = PathBuf::from(input_file_path);
    match path.file_stem() {
        Some(fs) => {
            let fs = fs.to_string_lossy().into_owned();
            let name = 
                if fs.ends_with(".rs") {
                    &fs[..fs.len() - 3]
                } else {
                    &fs[..]
                };

            let mut view_name = String::with_capacity(name.len());
            let mut capitilize = true;

            for c in name.chars() {
                match c {
                    '.' | '_' | '-' => capitilize = true,
                    _ => {
                        if capitilize {
                            for capital in c.to_uppercase() {
                                view_name.push(capital);
                            }
                        } else {
                            view_name.push(c);
                        }
                        capitilize = false
                    }
                }
            }

            view_name
        },
        None => {
            "View".to_string()
        }
    }
}

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
        Err(e) => panic!("Unable to get options: {}", e)
    };

    if matches.free.is_empty() {
        print_usage(&args[0][..], opts);
        return;
    }

    let input_file_name = &matches.free[0][..];
    let output_file_name = match matches.opt_str("o") {
        Some(ofn) => PathBuf::from(ofn),
        None => output_file_from_input(input_file_name)
    };

    let contents = match get_file_contents(input_file_name) {
        Ok(contents) => contents,
        Err(e) => panic!(e.to_string())
    };

    let mut parser = parser::Parser::new(&contents[..]);
    parser.parse();

    //for section in parser.sections.iter() {
    //    println!("{}", section);
    //}

    view_writer::write_view(&view_name(input_file_name)[..], output_file_name.as_path(), &parser.sections);
}
