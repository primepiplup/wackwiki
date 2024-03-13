pub struct LiteralToken {
    content: String,
}

impl LiteralToken {
    pub fn new(content: String) -> LiteralToken {
        LiteralToken {
            content
        }
    }
}

impl Token for LiteralToken {
    fn add(&self) -> String {
        return self.content.to_owned();
    }
}

pub struct TagToken<'a> {
    tag: &'a str,
    closer: bool,
}

impl TagToken<'_> {
    pub fn new(tag: &str, closer: bool) -> TagToken {
        TagToken {
            tag,
            closer,
        }
    }
}

impl Token for TagToken<'_> {
    fn add(&self) -> String {
        if !self.closer {
            return format!("<{}>", self.tag);
        } else {
            return format!("</{}>", self.tag);
        }
    }
}

pub struct CharToken {
    c: char,
}

impl CharToken {
    pub fn new(c: char) -> CharToken {
        CharToken { c }
    }
}

impl Token for CharToken {
    fn add(&self) -> String {
        return self.c.to_string();
    }
}

pub trait Token {
    fn add(&self) -> String;
}
