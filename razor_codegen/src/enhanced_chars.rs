use std::str::Chars;

pub struct EnhancedCharIterator<'a> {
    iterator: Chars<'a>,
    previous_value: Option<char>,
    current_value: Option<char>,
    next_value: Option<char>,
    next_index: usize,
}

pub trait EnhancedChars<'a> {
    /// An enhanced character iterator
    /// Provides look-ahead and look-behind, as well as 
    /// enumeration, in one iterator.
    fn enhanced_chars(&self) -> EnhancedCharIterator<'a>;
}

impl<'a> EnhancedChars<'a> for &'a str {
    fn enhanced_chars(&self) -> EnhancedCharIterator<'a> {
        let mut chars = self.chars();
        let next_value = chars.next();
        EnhancedCharIterator {
            iterator: chars,
            previous_value: None,
            current_value: None,
            next_value,
            next_index: 0,
        }
    }
}
pub type EnhancedCharsValue = (usize, Option<char>, char, Option<char>);

impl<'a> Iterator for EnhancedCharIterator<'a> {
    type Item = EnhancedCharsValue;
    fn next(&mut self) -> Option<EnhancedCharsValue> {
        self.previous_value = self.current_value;
        self.current_value = self.next_value;
        self.next_value = self.iterator.next();
        if let Some(current_value) = self.current_value {
            self.next_index += 1;
            Some((
                self.next_index - 1,
                self.previous_value,
                current_value,
                self.next_value,
            ))
        } else {
            None
        }
    }
}