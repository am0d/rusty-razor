use std;

#[derive(Clone,Debug,PartialEq)]
pub enum Token {
    String(std::string::String),
    Whitespace(char),
    Operator(char),
    AtSymbol,
}
