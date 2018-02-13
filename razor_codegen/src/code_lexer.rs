pub struct CodeLexer<'a> {
    pub source: &'a str,
}

impl<'a> CodeLexer<'a> {
    pub fn new(source: &'a str) -> CodeLexer<'a> {
        CodeLexer { source: source }
    }

    pub fn is_keyword(&self, identifier: &str) -> bool {
        match identifier {
            "if" | "for" | "model" | "while" | "match" | "use" => true,
            _ => false,
        }
    }

    pub fn accept_identifier(&self, source: &str) -> String {
        source
            .chars()
            .enumerate()
            .take_while(|&(index, c)| match c {
                'A'...'Z' | 'a'...'z' | '_' => true,
                '0'...'9' if index > 0 => true,
                _ => false,
            })
            .map(|(_, c)| c)
            .collect::<String>()
    }

    pub fn end_of_block(&self, start_char: char, end_char: char) -> Option<usize> {
        let mut scope = 0i32;
        let mut in_quote: Option<char> = None;
        for (index, c) in self.source.chars().enumerate() {
            if c == '\'' || c == '"' {
                in_quote = match in_quote {
                    None => Some(c),
                    Some(q) if q == c => None,
                    _ => in_quote,
                };
            }

            if in_quote.is_none() {
                if c == start_char {
                    scope += 1;
                } else if c == end_char {
                    scope -= 1;
                    if scope <= 0 {
                        return Some(index);
                    }
                }
            };
        }

        None
    }

    pub fn next_instance_of(&self, search_char: char) -> Option<usize> {
        self.source.chars().position(|c| c == search_char)
    }

    pub fn end_of_code_block(&self) -> Option<usize> {
        self.end_of_block('{', '}')
    }

    pub fn end_of_code_statement(&self) -> Option<usize> {
        self.next_instance_of(';')
    }

    pub fn block_delimiters(&self) -> (Option<usize>, Option<usize>) {
        (self.next_instance_of('{'), self.end_of_block('{', '}'))
    }
}
