use std::str::Chars;
use std::iter::Peekable;

use token::{Token, String, Whitespace, Operator, AtSymbol};

pub struct Lexer<'a> {
    line: int,
    column: int,
    source: &'a str,
    chars: Peekable<char, Chars<'a>>,
    peek: Option<Token>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            line: 1,
            column: 1,
            source: source,
            chars: source.chars().peekable(),
            peek: None
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        match self.peek {
            None => {
               // debug!("Using next_token() result");
                self.next_token()
            },
            Some(_) => {
                //debug!("Using self.peek value");
                let tmp = self.peek.clone();
                self.peek = None;
                tmp
            }
        }
    }

    pub fn unpeek(&mut self, token: Option<Token>) {
        self.peek = token.clone();
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut ret = ~"";
        let mut token: Option<Token>;
        let mut advance = true;
        
        loop {
            {
                let p = self.chars.peek();
                if p.is_none() {
                    if ret.len() > 0 {
                        return Some(String(ret));
                    } else {
                        return None;
                    }
                }
                let c = p.unwrap();
                match *c {
                      ' ' 
                    | '\n' 
                    | '\t' => {
                        if ret.len() > 0 {
                            token = Some(String(ret));
                            advance = false;
                        } else {
                            if *c == '\n' {
                                self.column = 0;
                                self.line += 1;
                            } else {
                                self.column += 1;
                            }
                            token = Some(Whitespace(*c));
                        }
                        break;
                    },
                      '{' 
                    | '}' 
                    | '.'
                    | ';' => {
                        if ret.len() > 0 {
                            token =  Some(String(ret));
                            advance = false;
                        } else {
                            self.column += 1;
                            token = Some(Operator(*c));
                        }
                        break;
                    },
                    '@' => {
                        if ret.len() > 0 {
                            token = Some(String(ret));
                            advance = false
                        } else {
                            self.column += 1;
                            token = Some(AtSymbol);
                        }
                        break;
                    },
                    c => {
                        self.column += 1;
                        ret.push_char(c);
                    }
                }
            }
            self.chars.next();
        }

        if advance {
            self.chars.next();
        }
        token
    }
}
