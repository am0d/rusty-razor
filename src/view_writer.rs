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

