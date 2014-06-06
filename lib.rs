    fn render(&self, &mut Writer);
}

pub struct SafeHtmlString {
    val: ~str
}

impl SafeHtmlString {
    pub fn new<'a>(v: &'a str) -> SafeHtmlString {
        SafeHtmlString {
            val: v.to_owned()
        }
    }

    #[inline]
    pub fn to_str(&self) -> ~str {
        return self.val.to_owned()
    }
}

pub trait AsSafeString {
    fn as_safe_string(&self) -> ~str;
}

impl<T:ToStr> AsSafeString for T {
    fn as_safe_string(&self) -> ~str {
        let v = format!("{}", *self);
        v
    }
}

/*impl AsSafeString for ~str {
    fn as_safe_string(&self) -> SafeHtmlString {
        use std::str;
        let mut buffer = str::with_capacity(self.char_len());

        for c in self.chars() {
            match c {
                '<' => buffer.push_str("&lt;"),
                '>' => buffer.push_str("&gt;"),
                '&' => buffer.push_str("&amp;"),
                _ => buffer.push_char(c)
            }
        }

        return SafeHtmlString {
            val: buffer
        }
    }
}
*/
