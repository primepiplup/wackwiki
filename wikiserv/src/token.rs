#[derive(Clone)]
pub struct LiteralToken {
    content: String,
    tokentype: TokenType,
}

impl LiteralToken {
    pub fn new(content: String) -> LiteralToken {
        LiteralToken {
            content,
            tokentype: TokenType::LITERAL,
        }
    }
}

impl Token for LiteralToken {
    fn add(&self) -> String {
        return self.content.to_owned();
    }

    fn tokentype(&self) -> &TokenType {
        return &self.tokentype;
    }

    fn literal_replace(&self) -> Box<dyn Token> {
        return Box::new(self.to_owned());
    }
}

pub struct BoldToken {
    closer: bool,
    tokentype: TokenType,
}

impl BoldToken {
    pub fn new(closer: bool) -> BoldToken {
        BoldToken {
            closer,
            tokentype: TokenType::BOLD,
        }
    }
}

impl Token for BoldToken {
    fn add(&self) -> String {
        if !self.closer {
            return format!("<b>");
        } else {
            return format!("</b>");
        }
    }

    fn tokentype(&self) -> &TokenType {
        return &self.tokentype;
    }

    fn literal_replace(&self) -> Box<dyn Token> {
        return Box::new(CharToken::new('*'));
    }
}

pub struct ItalicToken {
    closer: bool,
    tokentype: TokenType,
}

impl ItalicToken {
    pub fn new(closer: bool) -> ItalicToken {
        ItalicToken {
            closer,
            tokentype: TokenType::ITALIC,
        }
    }
}

impl Token for ItalicToken {
    fn add(&self) -> String {
        if !self.closer {
            return format!("<i>");
        } else {
            return format!("</i>");
        }
    }

    fn tokentype(&self) -> &TokenType {
        return &self.tokentype;
    }

    fn literal_replace(&self) -> Box<dyn Token> {
        return Box::new(CharToken::new('_'));
    }
}

#[derive(Clone)]
pub struct CharToken {
    c: char,
    tokentype: TokenType,
}

impl CharToken {
    pub fn new(c: char) -> CharToken {
        CharToken {
            c,
            tokentype: TokenType::CHAR,
        }
    }
}

impl Token for CharToken {
    fn add(&self) -> String {
        return self.c.to_string();
    }

    fn tokentype(&self) -> &TokenType {
        return &self.tokentype;
    }

    fn literal_replace(&self) -> Box<dyn Token> {
        return Box::new(self.to_owned());
    }
}

pub trait Token {
    fn add(&self) -> String;
    fn tokentype(&self) -> &TokenType;
    fn literal_replace(&self) -> Box<dyn Token>;
}

#[derive(PartialEq, Clone)]
pub enum TokenType {
    BOLD,
    CHAR,
    ITALIC,
    LITERAL,
}
