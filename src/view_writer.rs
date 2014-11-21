use std::io::File;
use collections::dlist::DList;
use parser::SectionType;

pub fn write_view(view_name: &str, out_path: &Path, sections: &DList<SectionType>) {
    let result = File::create(out_path).and_then(|file| {
        let mut file = file;
        let mut in_render = false;
        let mut model = String::new();

        for section in sections.iter() {
            match section {
                &SectionType::Html(ref s) => {
                    if in_render || s.as_slice().trim().len() > 0 {
                        if !in_render {
                            try!(writeln!(&mut file, "{}", prelude(view_name, &model)));
                        }
                        try!(writeln!(&mut file, "        try!(out.write_str(r###\"{}\"###));", *s));
                        in_render = true;
                    }
                },
                &SectionType::Code(ref s) => {
                    try!(writeln!(&mut file, "        {}", *s));
                },
                &SectionType::Directive(ref directive_name, ref directive_value) => {
                    match directive_name.as_slice() {
                        "model" => {
                            model = directive_value.clone();
                        },
                        _ => ()
                    };
                },
                &SectionType::Print(ref value) => {
                    try!(writeln!(&mut file, "        try!(write!(out, \"{{}}\", {}));", *value));
                },
                /*_ => {
                    dump!(section);
                }*/
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

fn prelude (view_name: &str, model: &String) -> String {
    String::from_str(format!("
use std::io::IoResult;
use super::Action;
pub struct {0}<'a> {{
    model: {1}
}}

impl<'a> {0}<'a> {{
    pub fn new(m: {1}) -> {0}<'a> {{
        {0} {{
            model: m
        }}
    }}
}}

impl<'a> Action for {0}<'a> {{
    fn render(&self, out: &mut Writer) -> IoResult<()> {{
        let ref model = self.model;", view_name, model.as_slice()).as_slice())
}

fn postlude() -> String {
    String::from_str("    Ok(())
    }
}")
}

