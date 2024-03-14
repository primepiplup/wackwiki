use crate::token::{BoldToken, CharToken, ItalicToken, LiteralToken, Token, TokenType};

pub fn line_parse_to_html(mut line: String) -> String {
    if line.starts_with('#') {
        line = parse_header(line);
    }

    let mut buffer: String = String::new();
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    let mut bold_open: bool = false;
    let mut italic_open: bool = false;
    let mut hit_index: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            hit_index += 1;
            i += 1;
            if i >= chars.len() {
                break;
            }
            tokens.push(Box::new(CharToken::new(chars[i])));
        } else if chars[i] == '*' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(BoldToken::new(bold_open)));
            bold_open = !bold_open;
        } else if chars[i] == '_' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(ItalicToken::new(italic_open)));
            italic_open = !italic_open;
        } else if chars[i] == ' ' {
            consume_literal(&line, &mut tokens, &mut hit_index, i);
            tokens.push(Box::new(CharToken::new(' ')));
        }

        i += 1;
    }

    if bold_open {
        remove_last(&mut tokens, TokenType::BOLD);
    }

    if italic_open {
        remove_last(&mut tokens, TokenType::ITALIC);
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

    // Here we now have a literal piece of content that is delimited by '*','_' or ' '
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

fn remove_last(tokens: &mut Vec<Box<dyn Token>>, tokentype: TokenType) -> () {
    for i in (0..tokens.len()).rev() {
        if tokens[i].tokentype() == &tokentype {
            tokens[i] = tokens[i].literal_replace();
            break;
        }
    }
}
