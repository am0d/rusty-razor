#[deriving(Clone,Show,Eq)]
pub enum Token {
    String(StrBuf),
    Whitespace(char),
    Operator(char),
    AtSymbol
}
