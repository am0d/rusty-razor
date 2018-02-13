use enhanced_chars::*;

pub fn nth_char(source: &str, index: usize) -> char {
    source.char_indices().nth(index).unwrap().1
}

#[derive(Copy, Clone, Debug)]
pub struct HtmlLexer<'a> {
    pub source: &'a str,
}

impl<'a> HtmlLexer<'a> {
    pub fn new(source: &'a str) -> HtmlLexer<'a> {
        HtmlLexer { source: source }
    }

    fn is_valid_email_char(c: char) -> bool {
        match c {
            'A'...'Z' | 'a'...'z' | '0'...'9' => true,
            _ => false,
        }
    }

    /// Checks to see if there is a valid transition to code at index
    /// A valid transition occurs after an `@`, except in the following cases:
    /// - An `@` in an email address (determined as an `@` preceded and followed by
    ///   valid email characters),
    /// - An `@` followed by another `@` (this is an escaped `@`)
    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    fn is_valid_transition(&self, values: EnhancedCharsValue) -> bool {
        match values {
            (_, _, _, None) => false, // No more characters, this isn't a valid transition
            (_, Some(prev_char), '@', Some(next_char))
                if HtmlLexer::is_valid_email_char(prev_char)
                    && HtmlLexer::is_valid_email_char(next_char) =>
            {
                false // This is an email address, not a valid transition
            }
            (_, _, '@', Some('@')) | (_, Some('@'), '@', _) => false, // This is an escaped '@', no transition
            (_, _, '@', _) => true, // This '@' marks a transition to code
            _ => false,
        }
    }

    pub fn next_transition(&self) -> Option<(usize, &'a str)> {
        self.source
            .enhanced_chars()
            .find(|values| self.is_valid_transition(*values))
            .map(|(index, _, _, _)| (index, &self.source[..index]))
    }
}
