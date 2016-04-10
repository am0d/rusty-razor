//use std::str::Chars;

//use token::{Token, String, Whitespace, Operator, AtSymbol};

pub fn first_char(source: &str) -> char {
    source.char_indices().next().unwrap().1
}

pub fn nth_char(source: &str, index: usize) -> char {
    source.char_indices().nth(index).unwrap().1
}

pub struct CodeLexer<'a> {
    pub line: i32,
    pub column: i32,
    pub source: &'a str,
}

impl<'a> CodeLexer<'a> {
    pub fn new(source: &'a str, line: i32, column: i32) -> CodeLexer<'a> {
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

    pub fn accept_identifier(&self, source: &str) -> String {
        source.chars().enumerate().take_while(|&(index, c)| {
            match c {
                'A' ... 'Z' |
                'a' ... 'z' |
                '_' => {
                    true
                },
                '0' ... '9' if index > 0 => true,
                _ => false
            }
        }).map(|(_, c)| c).collect::<String>()
    }

    pub fn end_of_block(&self, start_char: char, end_char: char) -> Option<usize> {
        let mut scope = 0i32;
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

    pub fn next_instance_of(&self, search_char: char) -> Option<usize> {
        self.source.chars().position(|c| {
            c == search_char
        })
    }

    pub fn end_of_code_block(&self) -> Option<usize> {
        self.end_of_block('{', '}')
    }

    pub fn end_of_code_statement(&self) -> Option<usize> {
        self.next_instance_of(';')
    }

    pub fn block_delimiters(&self) -> (Option<usize>, Option<usize>) {
        (self.next_instance_of('{'), self.end_of_block('{', '}'))
    }
}

pub struct HtmlLexer<'a> {
    pub line: i32,
    pub column: i32,
    pub source: &'a str,
}

impl<'a> HtmlLexer<'a> {
    pub fn new(source: &'a str, line: i32, column: i32) -> HtmlLexer<'a> {
        HtmlLexer {
            line: line,
            column: column,
            source: source,
        }
    }

    fn is_valid_email_char(&self, c: char) -> bool {
        match c {
            'A' ... 'Z' |
            'a' ... 'z' |
            '0' ... '9' => true,
            _ => false
        }
    }

    fn is_valid_transition(&self, index: usize) -> bool {
        if index == 0 {
            return true;
        }

        if index == self.source.len() - 1 {
            return false;
        }

        if self.is_valid_email_char(nth_char(self.source, index - 1)) &&
            self.is_valid_email_char(nth_char(self.source, index + 1)) {
                return false;
        }

        //if self.source.char_at(index - 1) == '@' ||
        //    self.source.char_at(index + 1) == '@' {
        //        return false;
        //}

        true
    }

    pub fn next_transition(&mut self) -> Option<usize> {
        for (index, c) in self.source.chars().enumerate() {
            if c == '@' && self.is_valid_transition(index) {
                return Some(index)
            }
        }

        None
    }
}
