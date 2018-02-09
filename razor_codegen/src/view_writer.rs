use std::io::{self, Write};
use parser::SectionType;

fn get_model(sections: &[SectionType]) -> Option<String> {
    for section in sections {
        if let SectionType::Directive(ref directive_name, ref value) = *section {
            if directive_name == "model" {
                return Some(value.to_string());
            }
        }
    }
    None
}

pub fn write_view<T: io::Write>(
    view_name: &str,
    out: &mut T,
    sections: &[SectionType],
) -> io::Result<()> {
    let mut out = out;
    let mut in_render = false;
    let model = get_model(sections);
    let model = model.as_ref().map(String::as_str);

    for section in sections.iter() {
        match *section {
            SectionType::Html(ref s) => {
                if in_render || !s[..].trim().is_empty() {
                    if !in_render {
                        writeln!(&mut out, "{}", prelude(view_name, model))?;
                    }
                    writeln!(&mut out, "        _out.write_str(r###\"{}\"###)?;", *s)?;
                    in_render = true;
                }
            }
            SectionType::Code(ref s) => {
                writeln!(&mut out, "        {}", *s)?;
            }
            SectionType::Directive(ref directive_name, ref _directive_value) => {
                match &directive_name[..] {
                    //"model" => (), // was moved to the get_model function for now
                    _ => (),
                };
            }
            SectionType::Print(ref value) => {
                writeln!(
                    &mut out,
                    "        write!(_out, \"{{}}\", {});",
                    *value
                )?;
            }
        }
    }
    if !in_render {
        writeln!(&mut out, "{}", prelude(view_name, model))?;
    }

    writeln!(&mut out, "{}", postlude())?;
    Ok(())
}

fn prelude(view_name: &str, model: Option<&str>) -> String {
    use std::fmt::Write;
    let mut prelude = String::new();
    write!(
        prelude,
        "use std::fmt;

pub struct {}",
        view_name
    ).unwrap();
    if let Some(model) = model {
        write!(
            prelude,
            "{{
    model: {}
}}",
            model
        ).unwrap();
    } else {
        write!(prelude, ";").unwrap();
    }

    write!(
        prelude,
        "

impl {} {{
    pub fn new(",
        view_name
    ).unwrap();
    if let Some(model) = model {
        write!(prelude, "m: {}", model).unwrap();
    }
    write!(
        prelude,
        ") -> {0} {{
        {0}",
        view_name
    ).unwrap();
    if model.is_some() {
        write!(
            prelude,
            " {{
            model: m
        }}"
        ).unwrap();
    }
    write!(
        prelude,
        "        
    }}
}}

impl fmt::Display for {} {{
    fn fmt(&self, _out: &mut fmt::Formatter) -> fmt::Result {{",
        view_name,
    ).unwrap();
    prelude
}

fn postlude() -> String {
    String::from(
        "    Ok(())
    }
}",
    )
}
