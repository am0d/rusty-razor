/* Copyright 2014 Damien Schoof

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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
