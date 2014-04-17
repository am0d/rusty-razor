//use std::str::Chars;

//use token::{Token, String, Whitespace, Operator, AtSymbol};

pub struct CodeLexer<'a> {
    pub line: int,
    pub column: int,
    pub source: &'a str,
}

impl<'a> CodeLexer<'a> {
    pub fn new(source: &'a str, line: int, column: int) -> CodeLexer<'a> {
        CodeLexer {
            line: line,
            column: column,
            source: source,
        }
    }

    pub fn is_keyword(&self, identifier: &str) -> bool {
        match identifier {
            "if" |
            "for" |
            "model" |
            "while" => true,
            _ => false
        }
    }

    pub fn accept_identifier(&self, source: &str) -> StrBuf {
        let mut first = true;
        source.chars().take_while(|c| {
            match *c {
                'A' .. 'Z' |
                'a' .. 'z' |
                '_' => {
                    first = false;
                    true
                },
                '0' .. '9' if !first => true,
                _ => false
            }
        }).collect::<StrBuf>()
    }

    fn end_of_block(&self, start_char: char, end_char: char) -> Option<uint> {
        let mut scope = 0;
        let mut in_quote: Option<char> = None;
        for (index, c) in self.source.chars().enumerate() {
            if c == '\'' || c == '"' {
                in_quote = match in_quote {
                    None => Some(c),
                    Some(q) if q == c => None,
                    _ => in_quote
                };
            }

            match in_quote {
                None => {
                    if c == start_char {
                        scope += 1;
                    }
                    else if c == end_char {
                        scope -= 1;
                        if scope <= 0 {
                            return Some(index);
                        }
                    }
                },
                _ => ()
            };
        }

        None
    }

    pub fn next_instance_of(&self, search_char: char) -> Option<uint> {
        self.source.chars().position(|c| {
            c == search_char
        })
    }

    pub fn end_of_code_block(&self) -> Option<uint> {
        self.end_of_block('{', '}')
    }
}

pub struct HtmlLexer<'a> {
    pub line: int,
    pub column: int,
    pub source: &'a str,
}

impl<'a> HtmlLexer<'a> {
    pub fn new(source: &'a str, line: int, column: int) -> HtmlLexer<'a> {
        HtmlLexer {
            line: line,
            column: column,
            source: source,
        }
    }

    fn is_valid_email_char(&self, c: char) -> bool {
        match c {
            'A' .. 'Z' |
            'a' .. 'z' |
            '0' .. '9' => true,
            _ => false
        }
    }

    fn is_valid_transition(&self, index: uint) -> bool {
        if index == 0 {
            return true;
        }

        if index == self.source.len() - 1 {
            return false;
        }

        if self.is_valid_email_char(self.source.char_at(index - 1)) &&
            self.is_valid_email_char(self.source.char_at(index + 1)) {
                return false;
        }

        if self.source.char_at(index - 1) == '@' ||
            self.source.char_at(index + 1) == '@' {
                return false;
        }

        true
    }

    pub fn next_transition(&mut self) -> Option<uint> {
        for (index, c) in self.source.chars().enumerate() {
            if c == '@' && self.is_valid_transition(index) {
                return Some(index)
            }
        }

        None
    }
}
