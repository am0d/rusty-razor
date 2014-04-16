use std::io::File;
use collections::dlist::DList;
use parser;

pub fn write_view(out_path: &Path, sections: &DList<parser::SectionType>) {
    let result = File::create(out_path).and_then(|file| {
        let mut file = file;
        let mut in_render = false;
        let mut model = StrBuf::new();

        for section in sections.iter() {
            match section {
                &parser::Html(ref s) => {
                    if in_render || s.as_slice().trim().len() > 0 {
                        if !in_render {
                            try!(writeln!(&mut file, "{}", prelude(&model)));
                        }
                        try!(writeln!(&mut file, "        out.write_string(r\\#\\#\\#\"{}\"\\#\\#\\#);", *s));
                        in_render = true;
                    }
                },
                &parser::Rust(ref s) => {
                    try!(writeln!(&mut file, "        {}", *s));
                },
                &parser::Directive(ref directive_name, ref directive_value) => {
                    match directive_name.as_slice() {
                        "model" => {
                            model = directive_value.clone();
                        },
                        _ => ()
                    };
                },
                _ => {}
            }
        }
        try!(writeln!(&mut file, "{}", postlude()));
        Ok(())
    });

    match result {
        Err(e) => println!("Error writing to file: {}", e),
        _ => ()
    }
}

fn prelude (model: &StrBuf) -> ~str {
    format!("extern crate extra;
extern crate http;

use super::super::\\{View, SafeHtmlString\\};
pub struct TodoIndexView<'a> \\{
    model: {0}
\\}

impl<'a> TodoIndexView<'a> \\{
    pub fn new(m: {0}) -> TodoIndexView<'a> \\{
        TodoIndexView \\{
            model: m//.clone()
        \\}
    \\}
\\}

impl<'a> Action for TodoIndexView<'a> \\{
    fn render(&self, out: &mut Writer) \\{", model.as_slice())
}

fn postlude() -> ~str {
    ~"    }
}"
}

