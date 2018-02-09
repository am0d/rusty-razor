pub fn nth_char(source: &str, index: usize) -> char {
    source.char_indices().nth(index).unwrap().1
}

#[derive(Copy, Clone, Debug)]
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

    fn is_valid_email_char(c: char) -> bool {
        match c {
            'A'...'Z' | 'a'...'z' | '0'...'9' => true,
            _ => false,
        }
    }

    fn is_valid_transition(&self, index: usize) -> bool {
        if index == 0 {
            return true;
        }

        if index == self.source.len() - 1 {
            return false;
        }

        if HtmlLexer::is_valid_email_char(nth_char(self.source, index - 1))
            && HtmlLexer::is_valid_email_char(nth_char(self.source, index + 1))
        {
            return false;
        }

        // if self.source.char_at(index - 1) == '@' ||
        //    self.source.char_at(index + 1) == '@' {
        //        return false;
        // }

        true
    }

    pub fn next_transition(&self) -> Option<usize> {
        for (index, c) in self.source.chars().enumerate() {
            if c == '@' && self.is_valid_transition(index) {
                return Some(index);
            }
        }

        None
    }
}
