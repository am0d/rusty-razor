use std::fmt;

#[deriving(Clone)]
pub enum Token {
    String(StrBuf),
    Whitespace(char),
    Operator(char),
    AtSymbol
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            String(ref s) => write!(f.buf, "String({})", *s),
            Whitespace(ref c) => write!(f.buf, "Whitespace({})", *c),
            Operator(ref c) => write!(f.buf, "Operator({})", *c),
            AtSymbol => write!(f.buf, "AtSymbol")
        }
    }
}

impl Eq for Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other) {
            (&String(ref s1), &String(ref s2)) if s1 == s2 => true,
            (&Whitespace(ref c1), &Whitespace(ref c2)) if c1 == c2 => true,
            (&Operator(ref c1), &Operator(ref c2)) if c1 == c2 => true,
            (&AtSymbol, &AtSymbol) => true,
            _ => false
        }
    }
}
