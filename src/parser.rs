use collections::deque::Deque;
use collections::dlist::DList;

use lexer::{HtmlLexer, CodeLexer};
//use token::{String, Whitespace, Operator, AtSymbol};

#[deriving(Show)]
pub enum SectionType {
    Html(StrBuf),
    Code(StrBuf),
    Directive(StrBuf, StrBuf),
    Print(StrBuf),
    Comment(StrBuf)
}

pub struct Parser<'a> {
    pub sections: DList<SectionType>,
    source: &'a str
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            sections: DList::new(),
            source: source
        }
    }

    pub fn parse(&mut self) {
        let sections = self.parse_html(self.source, 1, 1);

        self.sections = sections;
    }

    pub fn parse_html(&self, source: &'a str, line: int, column: int) -> DList<SectionType> {
        let mut sections = DList::new();
        let mut lexer = HtmlLexer::new(source, line, column);

        match lexer.next_transition() {
            Some(index) => {
                if index > 0 {
                    sections.push_back(Html(source.slice_to(index).to_strbuf()));
                }

                if index < source.len() {
                    sections.append(self.parse_code(source.slice_from(index), 1, 1));
                }
            },
            None => {
                sections.push_back(Html(source.to_strbuf()));
            }
        };

        sections
    }


    fn parse_code(&self, source: &'a str, line: int, column: int) -> DList<SectionType> {
        let sections: DList<SectionType> = DList::new();
        //let mut lexer = CodeLexer::new(source, line, column);

        if source.len() < 2 {
            return sections;
        }

        let first = source.char_at(0);
        match first {
            '@' => (),
            _ => return self.parse_html(source, line, column)
        };

        let next = source.char_at(1);
        match next {
            '{' => self.parse_code_block(source, line, column),
            _ => {
                self.parse_expression_block(source, line, column)
            }
        }
    }

    fn parse_code_block(&self, source: &'a str, line: int, column: int) -> DList<SectionType> {
        let mut sections: DList<SectionType> = DList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.end_of_code_block() {
            None => fail!("Unterminated code block"),
            Some(index) => {
                // skip the @{ by starting the slice at 2
                sections.push_back(Code(source.slice_chars(2, index).to_strbuf()));
                
                if index < source.len() {
                    sections.append(self.parse_html(source.slice_from(index + 1), 1, 1));
                }
            }
        };

        sections
    }

    fn parse_expression_block(&self, source: &str, line: int, column: int) -> DList<SectionType> {
        //let mut sections: DList<SectionType> = DList::new();
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
                let identifier = lexer.accept_identifier(source.slice_from(1));
                let identifier = identifier.as_slice();

                if lexer.is_keyword(identifier) {
                    return self.parse_keyword(identifier, source.slice_from(1), 1, 1);
                }
            },
            None => ()
        };

        self.parse_expression(source, line, column)
    }

    fn parse_expression(&self, source: &str, line: int, column: int) -> DList<SectionType> {
        let sections: DList<SectionType> = DList::new();
        let lexer = CodeLexer::new(source, line, column);

        sections
    }

    fn parse_keyword(&self, identifier: &str, source: &str, line: int, column: int) -> DList<SectionType> {
        //let sections: DList<SectionType> = DList::new();
        //let lexer = CodeLexer::new(source, line, column);

        match identifier {
            "model" => {
                self.parse_model(source.slice_from(identifier.len()), line, column)
            },
            "for" |
            "while" => {
                self.parse_simple_block(source, line, column)
            },
            _ => {
                dump!(identifier);
                self.parse_html(source, line, column)
            }
        }
    }

    fn parse_model(&self, source: &str, line: int, column: int) -> DList<SectionType> {
        let mut sections: DList<SectionType> = DList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.end_of_code_statement() {
            Some(index) => {
                // don't include the `;`
                sections.push_back(Directive(StrBuf::from_str("model"), source.slice_to(index).to_strbuf()));
                // now skip the `;`
                sections.append(self.parse_html(source.slice_from(index + 1), 1, 1));
                return sections
            },
            None => {
                fail!("Unable to find end of `model` directive at {}:{}", line, column)
            }
        };
    }

    fn parse_simple_block(&self, source: &str, line: int, column: int) -> DList<SectionType> {
        let mut sections: DList<SectionType> = DList::new();
        let lexer = CodeLexer::new(source, line, column);

        match lexer.block_delimiters() {
            (Some(start), Some(end)) => {
                sections.push_back(Code(source.slice_to(start + 1).to_strbuf()));
                sections.append(self.parse_html(source.slice_chars(start+1, end), 1, 1));
                sections.push_back(Code("}".to_strbuf()));
                sections.append(self.parse_html(source.slice_from(end+1), 1, 1));
                sections
            },
            (Some(_), None) => {
                fail!("Missing end `\\}` for code block beginning at {}:{}", line, column);
            },
            (None, Some(_)) => {
                fail!("Missing start `\\{` for code block beginning at {}:{}", line, column);
            },
            (None, None) => {
                fail!("Unable to find start `\\{` and end `\\}` for code block beginning at {}:{}", line, column);
            }
        }
    }
}
