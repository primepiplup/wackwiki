use crate::{token::{BoldToken, CharToken, ItalicToken, LiteralToken, LinkToken, Token, TokenType, UnderlineToken}, paths::Paths};

pub fn line_parse_to_html(mut line: String, paths: &Paths, requestpath: &str) -> String {
    if line.starts_with('#') {
        line = parse_header(line);
    }

    let mut buffer: String = String::new();
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    let mut bold_open: bool = false;
    let mut italic_open: bool = false;
    let mut underline_open: bool = false;
    let mut hit_index: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            hit_index += 1;
            i += 1;
            if i >= chars.len() {
                break;
            }
            tokens.push(Box::new(CharToken::new(chars[i])));
        } else if chars[i] == '*' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(BoldToken::new(bold_open)));
            bold_open = !bold_open;
        } else if chars[i] == '_' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(ItalicToken::new(italic_open)));
            italic_open = !italic_open;
        } else if chars[i] == '~' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(UnderlineToken::new(underline_open)));
            underline_open = !underline_open;
        } else if chars[i] == ' ' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
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
        consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
    }

    for token in tokens {
        buffer += &token.add();
    }

    return buffer;
}

fn consume_literal(line: &String, tokens: &mut Vec<Box<dyn Token>>, hit_index: &mut usize, i: usize, paths: &Paths, requestpath: &str) -> () {
    let content = line.split_at(*hit_index).1.split_at(i - hit_index.to_owned()).0;
    if content == "" {
        *hit_index = i + 1;
        return;
    }
    println!("{}", content);

    let absolutepath: String;
    if content.starts_with("/") {
        absolutepath = content.to_string();
    } else {
        let relativepath = match requestpath.rsplit_once('/') {
            Some(path) => path,
            None   => return,
        }.0;
        println!("using relative path: {}", relativepath);
        absolutepath = relativepath.to_string() + "/" + content;
    }

    let mut close = false;
    if paths.contains(&absolutepath) {
        println!("inserting link to: {}", absolutepath);
        tokens.push(Box::new(LinkToken::new(absolutepath.clone(), false)));
        close = true;
    } else {
        print!("Paths did not contain {}", content);
    }

    tokens.push(Box::new(LiteralToken::new(content.to_owned())));
    *hit_index = i + 1;

    if close {
        tokens.push(Box::new(LinkToken::new(absolutepath.clone(), true)));
    }
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
