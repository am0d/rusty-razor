pub enum SectionType {
    Html(~str),
    Rust(~str),
    Print(~str),
    Comment(~str)
}

enum LexerState {
    Text,
    At,
    Code
}

pub struct Lexer {
    line: int,
    column: int,
    state: LexerState,
    sections: ~[SectionType]
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            line: 1,
            column: 0,
            state: Text,
            sections: ~[]
        }
    }

    pub fn lex(&mut self, source: &str) {
        let mut seenWhitespace = false;
        let mut braceCount = 0;
        let mut text = ~"";

        for c in source.chars() {
            self.column = self.column + 1;
            match c {
                '\n' => {
                    self.line = self.line + 1;
                    self.column = 0;
                    seenWhitespace = true;
                    text.push_char(c);
                },
                '@' => {
                    match self.state {
                        Text => {
                            self.state = At;
                            debug!("Switched to At");
                        },
                        At => {
                            // output a single @ symbol
                            text.push_char(c);
                            self.state = Text;
                            debug!("Switched to Text");
                        },
                        Code => {
                            // output the code
                            text.push_char(c);
                        }
                    }
                },
                '{' => {
                    match self.state {
                        Text => {
                            text.push_char(c);
                        },
                        At => {
                            text = self.push_section(Text, text);
                            self.state = Code;
                            braceCount = braceCount + 1;
                            debug!("Switched to Code");
                        },
                        Code => {
                            text.push_char(c);
                            braceCount = braceCount + 1;
                        }
                    }
                },
                '}' => {
                    match self.state {
                        Text => {
                            text.push_char(c);
                        },
                        At => {
                            text.push_char(c);
                        },
                        Code => {
                            braceCount = braceCount - 1;
                            if (braceCount == 0) {
                                text = self.push_section(Code, text);
                                self.state = Text;
                                seenWhitespace = false;
                            } else {
                                text.push_char(c);
                            }
                        }
                    }
                },
                ' ' => {
                    text.push_char(c);
                    seenWhitespace = true;
                },
                '\t' => {
                    text.push_char(c);
                    seenWhitespace = true;
                },
                _ => {
                    if (seenWhitespace) {
                        text.push_char(c);
                        seenWhitespace = false;
                    } else {
                        text.push_char(c);
                    }
                }
            }
        }

        self.push_section(self.state, text);
    }

    fn push_section(&mut self, state: LexerState, text: &str) -> ~str {
        match state {
            At => {
                fail!("push_section called while in At state");
            },
            Code => {
                self.sections.push(Rust(text.to_owned()));
            },
            Text => {
                self.sections.push(Html(text.to_owned()));
            }
        }
        ~""
    }
}
