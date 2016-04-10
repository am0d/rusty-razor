//use std::collections::Deque;
use std::collections::LinkedList;

use lexer::{HtmlLexer, CodeLexer, first_char, nth_char};
//use token::{String, Whitespace, Operator, AtSymbol};

#[derive(Debug)]
pub enum SectionType {
    Html(String),
    Code(String),
    Directive(String, String),
    Print(String),
//    Comment(String)
}

pub struct Parser<'a> {
    pub sections: LinkedList<SectionType>,
    source: &'a str
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            sections: LinkedList::new(),
            source: source
        }
    }

    pub fn parse(&mut self) {
        let sections = self.parse_html(self.source, 1, 1);

        self.sections = sections;
    }

    pub fn parse_html(&self, source: &'a str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections = LinkedList::new();
        let mut lexer = HtmlLexer::new(source, line, column);

        match lexer.next_transition() {
            Some(index) => {
                if index > 0 {
                    sections.push_back(SectionType::Html(String::from(&source[..index])));
                }

                if index < source.len() {
                    sections.append(&mut self.parse_code(&source[index..], 1, 1));
                }
            },
            None => {
                sections.push_back(SectionType::Html(String::from(source)));
            }
        };

        sections
    }


    fn parse_code(&self, source: &'a str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        //let mut lexer = CodeLexer::new(source, line, column);

        if source.len() < 2 {
            return sections;
        }

        let first = first_char(source);
        match first {
            '@' => (),
            _ => return self.parse_html(source, line, column)
        };

        let next = nth_char(source, 1);
        match next {
            '{' => self.parse_code_block(source, line, column),
            '@' => {
                sections.push_back(SectionType::Html("@".to_string()));
                sections.append(&mut self.parse_html(&source[2..], line, column));
                sections
            },
            _ => {
                self.parse_expression_block(source, line, column)
            }
        }
    }

    fn parse_code_block(&self, source: &'a str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.end_of_code_block() {
            None => panic!("Unterminated code block"),
            Some(index) => {
                // skip the @{ by starting the slice at 2
                sections.push_back(SectionType::Code(String::from(&source[2..index])));
                
                if index < source.len() {
                    sections.append(&mut self.parse_html(&source[index + 1..], 1, 1));
                }
            }
        };

        sections
    }

    fn parse_expression_block(&self, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        //let mut sections: LinkedList<SectionType> = LinkedList::new();
        let lexer = CodeLexer::new(source, line, column);

        let next_brace = lexer.next_instance_of('{');
        let next_parenthese = lexer.next_instance_of('(');

        let next_transition = match (next_brace, next_parenthese) {
            (Some(b), Some(p)) => Some(::std::cmp::min(b, p)),
            (Some(b), _) => Some(b),
            (_, Some(p)) => Some(p),
            _ => None
            };

        match next_transition {
            Some(_) => {
                let identifier = lexer.accept_identifier(&source[1..]);
                let identifier = &identifier[..];

                if lexer.is_keyword(identifier) {
                    return self.parse_keyword(identifier, &source[1..], 1, 1);
                }
            },
            None => ()
        };

        self.parse_expression(source, line, column)
    }

    fn parse_expression(&self, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        //let lexer = CodeLexer::new(source, line, column);

        match self.read_expression(&source[1..], line, column) {
            None => sections.append(&mut self.parse_html(source, line, column)),
            Some(expression) => {
                let len = expression.len();
                sections.push_back(SectionType::Print(expression));
                sections.append(&mut self.parse_html(&source[len + 1..], line, column));
            }
        }
        
        sections
    }

    fn parse_keyword(&self, identifier: &str, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        //let sections: LinkedList<SectionType> = LinkedList::new();
        //let lexer = CodeLexer::new(source, line, column);

        match identifier {
            "model" => {
                self.parse_model(&source[identifier.len()..], line, column)
            },
            "use" => {
                self.parse_use(&source[identifier.len()..], line, column)
            },
            "for" |
            "while" => {
                self.parse_simple_block(source, line, column)
            },
            _ => {
                self.parse_html(source, line, column)
            }
        }
    }

    fn parse_model(&self, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.end_of_code_statement() {
            Some(index) => {
                // don't include the `;`
                sections.push_back(SectionType::Directive(String::from("model"), String::from(&source[..index])));
                // now skip the `;`
                sections.append(&mut self.parse_html(&source[index + 1..], 1, 1));
                return sections
            },
            None => {
                panic!("Unable to find end of `model` directive at {}:{}", line, column)
            }
        };
    }

    fn parse_use(&self, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.end_of_code_statement() {
            Some(index) => {
                sections.push_back(SectionType::Directive(String::from("use"), String::from(&source[..index+1])));
                // now skip the `;`
                sections.append(&mut self.parse_html(&source[index + 2..], 1, 1));
                return sections
            },
            None => {
                panic!("Unable to find end of `use` directive at {}:{}", line, column)
            }
        };
    }


    fn parse_simple_block(&self, source: &str, line: i32, column: i32) -> LinkedList<SectionType> {
        let mut sections: LinkedList<SectionType> = LinkedList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.block_delimiters() {
            (Some(start), Some(end)) => {
                sections.push_back(SectionType::Code(String::from(&source[..start + 1])));
                sections.append(&mut self.parse_html(&source[start+1..end], 1, 1));
                sections.push_back(SectionType::Code(String::from("}")));
                sections.append(&mut self.parse_html(&source[end+1..], 1, 1));
                sections
            },
            (Some(_), None) => {
                panic!("Missing end `}}` for code block beginning at {}:{}", line, column);
            },
            (None, Some(_)) => {
                panic!("Missing start `{{` for code block beginning at {}:{}", line, column);
            },
            (None, None) => {
                panic!("Unable to find start `{{` and end `}}` for code block beginning at {}:{}", line, column);
            }
        }
    }

    fn read_expression(&self, source: &str, line: i32, column: i32) -> Option<String> {
        enum State {
            Identifier,
            LookingForBlock,
            LookingForPeriod,
            LookingForSecondIdentifier
        }

        let mut current_state = State::Identifier;
        let mut end_of_expression: usize = 0;

        while end_of_expression < source.len() {
            let source = &source[end_of_expression..];
            let lexer = CodeLexer::new(source, line, column);
            //dump!(current_state, end_of_expression, source);

            match current_state {
                State::Identifier => {
                    let identifier = lexer.accept_identifier(source);

                    if identifier.len() == 0 {
                        break;
                    }

                    //dump!(identifier.as_slice());
                    end_of_expression += identifier.len();
                    current_state = State::LookingForBlock;
                },
                State::LookingForBlock => {
                    let start_char = first_char(source);

                    match start_char {
                        '[' |
                        '(' => {
                            let end_char = if start_char == '[' { ']' } else { ')' };

                            let end_index = lexer.end_of_block(start_char, end_char);

                            match end_index {
                                Some(index) => {
                                    end_of_expression += index;
                                },
                                None => {
                                    break;
                                }
                            }
                        },
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
                },
                State::LookingForSecondIdentifier => {
                    let identifier = lexer.accept_identifier(source);

                    if identifier.len() > 0 {
                        current_state = State::Identifier;
                    } else {
                        break;
                    }
                }
            }
        }

        match end_of_expression {
            0 => None,
            _ => Some(String::from(&source[..end_of_expression]))
        }
    }
}
