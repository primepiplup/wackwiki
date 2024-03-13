use crate::token::{CharToken, TagToken, LiteralToken, Token};

pub fn line_parse_to_html(mut line: String) -> String {
    if line.starts_with('#') {
        line = parse_header(line);
    }

    let mut buffer: String = String::new();
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    let mut bold_open: bool = false;
    let mut italic_open: bool = false;
    let mut hit_index: usize = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '*' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(TagToken::new("b", bold_open)));
            bold_open = !bold_open;
        }

        if c == '_' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(TagToken::new("i", italic_open)));
            italic_open = !italic_open;
        }

        if c == ' ' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(CharToken::new(' ')));
        }
    }

    if line.len() - hit_index > 1 {
        let content = line.split_at(hit_index).1;
        tokens.push(Box::new(LiteralToken::new(content.to_owned())));
    }

    for token in tokens {
        buffer += &token.add();
    }

    return buffer;
}

fn consume_literal(line: &String, tokens: &mut Vec<Box<dyn Token>>, hit_index: &mut usize, i: usize) -> () {
    let content = line.split_at(*hit_index).1.split_at(i - hit_index.to_owned()).0;

    // here we now have a literal piece of content that is delimited by '*','_' or ' '
    // We can now check whether this matches any of the wiki entry paths we have.
    
    tokens.push(Box::new(LiteralToken::new(content.to_owned())));
    *hit_index = i + 1;
}

fn parse_header(line: String) -> String {
    let mut chars = line.chars();
    let mut pound_count = 0;

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };
        if c != '#' {
            break;
        }
        pound_count += 1;
    }

    let (_, mut header_content) = line.split_at(pound_count);
    header_content = header_content.trim();

    if pound_count > 6 {
        pound_count = 6;
    }

    return format!("<h{pound_count}>{header_content}</h{pound_count}>");
}

