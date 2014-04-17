use std::fmt;
use collections::deque::Deque;
use collections::dlist::DList;

use lexer;
//use token::{String, Whitespace, Operator, AtSymbol};

pub enum SectionType {
    Html(StrBuf),
    Code(StrBuf),
    Directive(StrBuf, StrBuf),
    Print(StrBuf),
    Comment(StrBuf)
}

impl fmt::Show for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Html(ref s) => write!(f.buf, "Html({})", *s),
            Code(ref s) => write!(f.buf, "Code({})", *s),
            Print(ref s) => write!(f.buf, "Print({})", *s),
            Comment(ref s) => write!(f.buf, "Comment({})", *s),
            Directive(ref s1, ref s2) => write!(f.buf, "Directive({}, {})", *s1, *s2)
        }
    }
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
        let mut lexer = lexer::HtmlLexer::new(source, line, column);

        match lexer.next_transition() {
            Some(index) => {
                sections.push_back(Html(source.slice_to(index).to_strbuf()));

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
        //let mut lexer = lexer::CodeLexer::new(source, line, column);

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
        let lexer = lexer::CodeLexer::new(source, line, column);

        match lexer.end_of_code_block() {
            None => fail!("Unterminated code block"),
            Some(index) => {
                sections.push_back(Code(source.slice_to(index).to_strbuf()));
                
                if index < source.len() {
                    sections.append(self.parse_html(source.slice_from(index + 1), 1, 1));
                }
            }
        };

        sections
    }

    fn parse_expression_block(&self, source: &str, line: int, column: int) -> DList<SectionType> {
        //let mut sections: DList<SectionType> = DList::new();
        let lexer = lexer::CodeLexer::new(source, line, column);

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
        let lexer = lexer::CodeLexer::new(source, line, column);

        sections
    }

    fn parse_keyword(&self, identifier: &str, source: &str, line: int, column: int) -> DList<SectionType> {
        let sections: DList<SectionType> = DList::new();
        let lexer = lexer::CodeLexer::new(source, line, column);

        sections
    }
}
