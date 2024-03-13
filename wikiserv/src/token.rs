pub struct LiteralToken<'a> {
    content: &'a str,
}

impl LiteralToken<'_> {
    pub fn new(content: &str) -> LiteralToken {
        LiteralToken {
            content
        }
    }
}

impl Token for LiteralToken<'_> {
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

pub trait Token {
    fn add(&self) -> String;
}
