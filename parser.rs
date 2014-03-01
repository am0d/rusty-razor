use std::fmt;
use lexer;
use token::{String, Whitespace, Operator, AtSymbol};

pub enum SectionType {
    Html(~str),
    Rust(~str),
    Directive(~str, ~str),
    Print(~str),
    Comment(~str)
}

impl fmt::Show for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Html(ref s) => write!(f.buf, "Html({})", *s),
            Rust(ref s) => write!(f.buf, "Rust({})", *s),
            Print(ref s) => write!(f.buf, "Print({})", *s),
            Comment(ref s) => write!(f.buf, "Comment({})", *s),
            Directive(ref s1, ref s2) => write!(f.buf, "Directive({}, {})", *s1, *s2)
        }
    }
}

enum ParserState {
    Text,
    At
}

pub struct Parser<'a> {
    sections: ~[SectionType],
    lexer: &'a mut lexer::Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        Parser {
            sections: ~[],
            lexer: lexer
        }
    }

    pub fn parse(&mut self) {
        let sections = self.parse_html();

        self.sections = sections; 
    }

    pub fn parse_html(&mut self) -> ~[SectionType] {
        let mut text = ~"";
        let mut state = Text;
        let mut sections: ~[SectionType] = ~[];

        loop {
            match state {
                At => {
                    {
                        let c = self.lexer.next();
                        match c {
                            None => {
                                break;
                            },
                            Some(AtSymbol) => {
                                debug!("Found @@, switching to text");
                                state = Text;
                                text.push_str("@");
                                continue;
                            },
                            _ => {}
                        }

                        self.lexer.unpeek(c);
                    }
                    sections.push(Html(text));
                    text = ~"";

                    let code = self.parse_code();
                    sections.push_all_move(code);
                    state = Text;
                },
                Text => {
                    let token = self.lexer.next();
                    if token.is_none() {
                        break;
                    }
                    let token = token.unwrap();

                    //debug!("{}", token);

                    match token {
                        Whitespace(c) => {
                            text.push_char(c);
                        },
                        AtSymbol => {
                            match state {
                                Text => {
                                    state = At;
                                },
                                At => {
                                    // output a single @ symbol
                                    text.push_char('@');
                                    state = Text;
                                }
                            }
                        },
                        String(s) => {
                            text.push_str(s);
                        },
                        Operator(s) => {
                            text.push_char(s);
                        }
                    }
                }
            }
        }

        sections.push(Html(text));
        sections
    }


    fn parse_code(&mut self) -> ~[SectionType] {
        let mut code = ~"";
        let mut brace_count = 0;
        let mut include_last_token = true;
        let mut sections: ~[SectionType] = ~[];
        let mut is_directive = false;
        let mut directive_name = ~"";

        let mut next_token = self.lexer.next();
        let endToken = match next_token {
            None => {
                return sections;
            },
            Some(String(~"model")) => {
                is_directive = true;
                include_last_token = false;
                Operator(';')
            },
            Some(String(~"use")) => {
                Operator(';')
            },
            Some(String(~"for")) => {
                Operator('}')
            },
            Some(Operator('{')) => {
                include_last_token = false;
                Operator('}')
            },
            _ => {
                Whitespace(' ')
            }
        };

        if is_directive {
            directive_name = match next_token.unwrap() {
                String(s) => s,
                _ => fail!("BUG")
            };
            next_token = self.lexer.next();
        }

        loop {
            let c = next_token.unwrap();

            match c {
                Operator('{') => {
                    if include_last_token || brace_count > 0 {
                        code.push_char('{');
                    }
                    brace_count = brace_count + 1;
                },
                Operator('}') => {
                    brace_count = brace_count - 1;
                    if include_last_token || brace_count > 0 {
                        code.push_char('}');
                    }

                    if (brace_count == 0) && (endToken == c) {
                        let new_section = if is_directive {
                            Directive(directive_name, code)
                        } else {
                            Rust(code)
                        };
                        sections.push(new_section);
                        return sections;
                    }
                },
                Operator(op) => {
                    if c == endToken {
                        if include_last_token {
                            code.push_char(op);
                        }

                        let new_section = if is_directive {
                            Directive(directive_name, code)
                        } else {
                            Rust(code)
                        };
                        sections.push(new_section);
                        return sections;
                    } else {
                        code.push_char(op);
                    }
                },
                String(s) => {
                    code.push_str(s);
                },
                Whitespace(c) => {
                    code.push_char(c);
                },
                AtSymbol => {
                    code.push_char('@');
                }
            }
            next_token = self.lexer.next();
            if next_token.is_none() {
                break;
            }
        }
        sections
    }
}
