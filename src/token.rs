use std;

#[deriving(Clone,Show,PartialEq)]
pub enum Token {
    String(std::string::String),
    Whitespace(char),
    Operator(char),
    AtSymbol
}
