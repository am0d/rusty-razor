extern crate getopts;

use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};

pub use parser::Parser;

mod lexer;
mod parser;
mod token;
pub mod view_writer;

pub fn get_file_contents(file_path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(file_path);

    File::open(path).and_then(|file| {
        let mut file = file;
        let mut s = String::new();
        try!(file.read_to_string(&mut s));
        Ok(s)
    })
}

pub fn output_file_from_input(input_file_path: &str) -> PathBuf {
    let input_path = PathBuf::from(input_file_path);

    if input_file_path.ends_with(".rs.html") {
        PathBuf::from(&input_file_path[..input_file_path.len() - 5])
    } else {
        input_path.with_extension("rs")
    }
}

pub fn view_name(input_file_path: &str) -> String {
    let path = PathBuf::from(input_file_path);
    match path.file_stem() {
        Some(fs) => {
            let fs = fs.to_string_lossy().into_owned();
            let name = if fs.ends_with(".rs") {
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
        }
        None => "View".to_string(),
    }
}

// #[proc_macro_derive(RazorView)]
// pub fn derive_razor_view(input: TokenStream) -> TokenStream {
//     // Construct a string representation of the type definition
//     let s = input.to_string();
    
//     // Parse the string representation
//     let ast = syn::parse_macro_input(&s).unwrap();

//     // Build the impl
//     let gen = impl_hello_world(&ast);
    
//     // Return the generated impl
//     gen.parse().unwrap()
// }