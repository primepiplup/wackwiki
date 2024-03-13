use crate::token::{TagToken, LiteralToken, Token};

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
            if i - hit_index > 0 {
                let content = line.split_at(hit_index).1.split_at(i - hit_index).0;
                tokens.push(Box::new(LiteralToken::new(content)));
                hit_index = i + 1;
            }
            tokens.push(Box::new(TagToken::new("b", bold_open)));
            bold_open = !bold_open;
        }

        if c == '_' {
            if i - hit_index > 0 {
                let content = line.split_at(hit_index).1.split_at(i - hit_index).0;
                tokens.push(Box::new(LiteralToken::new(content)));
                hit_index = i + 1;
            }
            tokens.push(Box::new(TagToken::new("i", italic_open)));
            italic_open = !italic_open;
        }
    }

    if line.len() - hit_index > 1 {
        let content = line.split_at(hit_index).1;
        tokens.push(Box::new(LiteralToken::new(content)));
    }

    for token in tokens {
        buffer += &token.add();
    }

    return buffer;
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

