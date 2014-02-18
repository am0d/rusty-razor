use lexer;
use token::{Token, String, Whitespace, Operator, AtSymbol};

pub enum SectionType {
    Html(~str),
    Rust(~str),
    Print(~str),
    Comment(~str)
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
        let mut seenWhitespace = false;
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

                    debug!("{}", token);

                    match token {
                        Whitespace(c) => {
                            seenWhitespace = true;
                            text.push_char(c);
                        },
                        AtSymbol => {
                            match state {
                                Text => {
                                    state = At;
                                    debug!("({}, {}) Switched to At", self.lexer.line, self.lexer.column);
                                },
                                At => {
                                    // output a single @ symbol
                                    text.push_char('@');
                                    state = Text;
                                    debug!("({}, {}) Switched to Text", self.lexer.line, self.lexer.column);
                                }
                            }
                        },
                        String(s) => {
                            text.push_str(s);
                            seenWhitespace = false;
                        },
                        Operator(s) => {
                            text.push_char(s);
                            seenWhitespace = false;
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
        let mut braceCount = 0;
        let mut sections: ~[SectionType] = ~[];
        let endToken: Token;

        let mut next_token = self.lexer.next();
        match next_token {
            None => {
                return sections;
            },
            Some(String(~"model")) => {
                endToken = Operator(';');
            },
            Some(String(~"use")) => {
                endToken = Operator(';');
            },
            _ => {
                endToken = Whitespace(' ');
            }
        }
        loop {
            let c = next_token.unwrap();

            match c {
                Operator('{') => {
                    code.push_char('{');
                    braceCount = braceCount + 1;
                    debug!("Saw `\\{`, braceCount = {}", braceCount);
                },
                Operator('}') => {
                    braceCount = braceCount - 1;
                    debug!("Saw `\\}`, braceCount = {}", braceCount);
                    code.push_char('}');
                    if (braceCount == 0) && (endToken == Operator('}')) {
                        debug!("Pushing section {}", code);
                        sections.push(Rust(code));
                        return sections;
                    }
                },
                String(s) => {
                    code.push_str(s);
                },
                Operator(c) => {
                    code.push_char(c);
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
