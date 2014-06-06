#[deriving(Clone,Show,PartialEq)]
pub enum Token {
    String(String),
    Whitespace(char),
    Operator(char),
    AtSymbol
}
