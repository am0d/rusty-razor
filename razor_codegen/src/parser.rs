use code_lexer::CodeLexer;
use html_lexer::{nth_char, HtmlLexer};

pub fn first_char(source: &str) -> char {
    source.chars().next().unwrap()
}

#[derive(Debug)]
pub enum SectionType {
    Html(String),
    Code(String),
    Directive(String, String),
    Print(String), // Comment(String)
}

pub struct Parser<'a> {
    source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser { source: source }
    }

    pub fn parse(&self) -> Vec<SectionType> {
        let lexer = HtmlLexer::new(self.source);
        self.parse_html(lexer)
    }

    pub fn parse_html(&self, lexer: HtmlLexer) -> Vec<SectionType> {
        let mut pieces = Vec::new();

        match lexer.next_transition() {
            Some((index, html)) => {
                if !html.is_empty() {
                    pieces.push(SectionType::Html(String::from(html)));
                }

                if index < lexer.source.len() {
                    pieces.append(&mut self.parse_code(&lexer.source[index..]));
                }
            }
            None => {
                pieces.push(SectionType::Html(lexer.source.replace("@@", "@")));
            }
        };

        pieces
    }

    fn parse_code(&self, source: &'a str) -> Vec<SectionType> {
         if source.len() < 2 {
            return Vec::new();
        }

        let next = nth_char(source, 1);
        match next {
            '{' => self.parse_code_block(source),
            _ => self.parse_expression_block(source),
        }
    }

    fn parse_code_block(&self, source: &'a str) -> Vec<SectionType> {
        let mut sections: Vec<SectionType> = Vec::new();
        let lexer = CodeLexer::new(source);

        match lexer.end_of_code_block() {
            None => panic!("Unterminated code block"),
            Some(index) => {
                // skip the @{ by starting the slice at 2
                sections.push(SectionType::Code(String::from(&source[2..index])));

                if index < source.len() {
                    sections.append(&mut self.parse_html(HtmlLexer::new(&source[index + 1..])));
                }
            }
        };

        sections
    }

    fn parse_expression_block(&self, source: &str) -> Vec<SectionType> {
        let lexer = CodeLexer::new(source);

        let first_char = first_char(source);

        match first_char {
            '(' => self.parse_expression(source),
            '{' => self.parse_simple_block(source),
            _ => {
                let identifier = lexer.accept_identifier(&source[1..]);
                let identifier = &identifier[..];

                if lexer.is_keyword(identifier) {
                    self.parse_keyword(identifier, &source[1..])
                } else {
                    self.parse_expression(source)
                }
            }
        }
    }

    fn parse_expression(&self, source: &str) -> Vec<SectionType> {
        let mut sections: Vec<SectionType> = Vec::new();

        match self.read_expression(&source[1..]) {
            None => sections.append(&mut self.parse_html(HtmlLexer::new(source))),
            Some(expression) => {
                let len = expression.len();
                sections.push(SectionType::Print(expression));
                sections.append(&mut self.parse_html(HtmlLexer::new(&source[len + 1..])));
            }
        }

        sections
    }

    fn parse_keyword(&self, identifier: &str, source: &str) -> Vec<SectionType> {
        match identifier {
            "model" => self.parse_model(&source[identifier.len()..]),
            "use" => self.parse_use(&source[identifier.len()..]),
            "for" | "while" | "if" | "match" => self.parse_simple_block(source),
            _ => self.parse_html(HtmlLexer::new(source)),
        }
    }

    fn parse_model(&self, source: &str) -> Vec<SectionType> {
        let mut sections: Vec<SectionType> = Vec::new();
        let lexer = CodeLexer::new(source);

        match lexer.end_of_code_statement() {
            Some(index) => {
                // don't include the `;`
                sections.push(SectionType::Directive(
                    String::from("model"),
                    String::from(&source[..index]),
                ));
                // now skip the `;`
                sections.append(&mut self.parse_html(HtmlLexer::new(&source[index + 1..])));
                sections
            }
            None => panic!("Unable to find end of `model` directive"),
        }
    }

    fn parse_use(&self, source: &str) -> Vec<SectionType> {
        let mut sections: Vec<SectionType> = Vec::new();
        let lexer = CodeLexer::new(source);

        match lexer.end_of_code_statement() {
            Some(index) => {
                sections.push(SectionType::Directive(
                    String::from("use"),
                    String::from(&source[..index + 1]),
                ));
                // now skip the `;`
                sections.append(&mut self.parse_html(HtmlLexer::new(&source[index + 2..])));
                sections
            }
            None => panic!("Unable to find end of `use` directive "),
        }
    }

    fn parse_simple_block(&self, source: &str) -> Vec<SectionType> {
        let mut sections: Vec<SectionType> = Vec::new();
        let lexer = CodeLexer::new(source);

        match lexer.block_delimiters() {
            (Some(start), Some(end)) => {
                sections.push(SectionType::Code(String::from(&source[..start + 1])));
                sections.append(&mut self.parse_html(HtmlLexer::new(&source[start + 1..end])));
                sections.push(SectionType::Code(String::from("}")));
                sections.append(&mut self.parse_html(HtmlLexer::new(&source[end + 1..])));
                sections
            }
            (Some(_), None) => {
                panic!("Missing end `}}` for code block beginning");
            }
            (None, Some(_)) => {
                panic!("Missing start `{{` for code block beginning");
            }
            (None, None) => {
                panic!("Unable to find start `{` and end `}` for code block beginning");
            }
        }
    }

    fn read_expression(&self, source: &str) -> Option<String> {
        enum State {
            Identifier,
            LookingForBlock,
            LookingForPeriod,
            LookingForSecondIdentifier,
        }

        let mut current_state = State::Identifier;
        let mut end_of_expression: usize = 0;

        while end_of_expression < source.len() {
            let source = &source[end_of_expression..];
            let lexer = CodeLexer::new(source);
            // dump!(current_state, end_of_expression, source);

            match current_state {
                State::Identifier => {
                    let identifier = lexer.accept_identifier(source);

                    if identifier.is_empty() {
                        break;
                    }

                    // dump!(identifier.as_slice());
                    end_of_expression += identifier.len();
                    current_state = State::LookingForBlock;
                }
                State::LookingForBlock => {
                    let start_char = first_char(source);

                    match start_char {
                        '[' | '(' => {
                            let end_char = if start_char == '[' { ']' } else { ')' };

                            let end_index = lexer.end_of_block(start_char, end_char);

                            match end_index {
                                Some(index) => {
                                    end_of_expression += index;
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                        _ => {
                            current_state = State::LookingForPeriod;
                        }
                    }
                }
                State::LookingForPeriod => {
                    let start_char = first_char(source);

                    if start_char == '.' {
                        current_state = State::LookingForSecondIdentifier;
                        end_of_expression += 1;
                    } else {
                        break;
                    }
                }
                State::LookingForSecondIdentifier => {
                    let identifier = lexer.accept_identifier(source);

                    if !identifier.is_empty() {
                        current_state = State::Identifier;
                    } else {
                        break;
                    }
                }
            }
        }

        match end_of_expression {
            0 => None,
            _ => Some(String::from(&source[..end_of_expression])),
        }
    }
}
