use std::io;
use std::io::Write;
use std::collections::LinkedList;
use parser::SectionType;

pub fn write_view<T:Write>(view_name: &str, out: &mut T, sections: &LinkedList<SectionType>) -> io::Result<()> {
        let mut out = out;
        let mut in_render = false;
        let mut model = String::new();

        for section in sections.iter() {
            match section {
                &SectionType::Html(ref s) => {
                    if in_render || s[..].trim().len() > 0 {
                        if !in_render {
                            try!(writeln!(&mut out, "{}", prelude(view_name, &model)));
                        }
                        try!(writeln!(&mut out,
                                      "        try!(out.write_str(r###\"{}\"###));",
                                      *s));
                        in_render = true;
                    }
                }
                &SectionType::Code(ref s) => {
                    try!(writeln!(&mut out, "        {}", *s));
                }
                &SectionType::Directive(ref directive_name, ref directive_value) => {
                    match &directive_name[..] {
                        "model" => {
                            model = directive_value.clone();
                        }
                        _ => (),
                    };
                }
                &SectionType::Print(ref value) => {
                    try!(writeln!(&mut out,
                                  "        try!(write!(out, \"{{}}\", {}));",
                                  *value));
                }
                // _ => {
                // dump!(section);
                // }
            }
        }
        try!(writeln!(&mut out, "{}", postlude()));
        Ok(())
}

fn prelude(view_name: &str, model: &String) -> String {
    format!("use std::fmt;

pub struct {0} {{
    model: {1}
}}

impl {0} {{
    pub fn new(m: {1}) -> {0} {{
        {0} {{
            model: m
        }}
    }}
}}

impl fmt::Display for {0} {{
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {{",
            view_name,
            &model[..])
}

fn postlude() -> String {
    String::from("    Ok(())
    }
}")
}
