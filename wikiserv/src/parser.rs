use crate::{token::{BoldToken, CharToken, ItalicToken, LiteralToken, LinkToken, Token, TokenType, StrikethroughToken, UnderlineToken, BraceToken}, paths::Paths};

pub fn line_parse_to_html(mut line: String, paths: &Paths, requestpath: &str) -> (String, Status) {
    let mut buffer: String = String::new();
    let mut status = Status::Paragraph;

    if line.trim().is_empty() {
        return (line, Status::Empty);
    } else if line.starts_with('#') {
        line = parse_header(&line);
        status = Status::Header;
    } else if line.trim_start().starts_with(">") {
        let mut counter = 0;
        for c in line.trim_start().chars() {
            if c == '>' {
                counter += 1;
            } else {
                break;
            }
        }
        line = remove_num_chars_from_start(&line, counter);
        status = Status::BlockQuote(counter);
    } else if line.trim_start().starts_with("-") {
        let mut counter = 0;
        for c in line.chars() {
            if c == ' ' {
                counter += 1;
            } else {
                break;
            }
        }
        line = remove_char_from_start(&line, '-');
        status = Status::UnorderedList(counter / 4);
        buffer += "<li>";
    } else if line.trim_start().chars().collect::<Vec<char>>()[0].is_numeric() {
        let mut counter = 0;
        for c in line.chars() {
            if c == ' ' {
                counter += 1;
            } else {
                break;
            }
        }
        line = remove_char_from_start(&line, '.');
        status = Status::OrderedList(counter / 4);
        buffer += "<li>";
    }

    let mut tokens: Vec<Box<dyn Token>> = Vec::new();

    let mut bold_open: bool = false;
    let mut italic_open: bool = false;
    let mut underline_open: bool = false;
    let mut strikethrough_open: bool = false;

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
        } else if chars[i] == '=' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(UnderlineToken::new(underline_open)));
            underline_open = !underline_open;
        } else if chars[i] == '~' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(StrikethroughToken::new(strikethrough_open)));
            strikethrough_open = !strikethrough_open;
        } else if chars[i] == ' ' || chars[i] == '(' || chars[i] == ')' || chars[i] == '{' || chars[i] == '}' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            tokens.push(Box::new(CharToken::new(chars[i])));
        } else if chars[i] == '[' {
            consume_literal(&line, &mut tokens, &mut hit_index, i, paths, requestpath);
            consume_link(&line, &mut tokens, &mut hit_index, &mut i, paths, requestpath);
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

    if let Status::UnorderedList(_) = status {
        buffer += "</li>";
    } else if let Status::OrderedList(_) = status {
        buffer += "</li>";
    }

    return (buffer, status);
}

fn consume_link(line: &str, tokens: &mut Vec<Box<dyn Token>>, hit_index: &mut usize, i: &mut usize, paths: &Paths, requestpath: &str) -> () {
    let (_, after) = line.split_at(*i);
    let bracket_end = *i + match after.find(']') {
        Some(res) => res,
        None      => {
            println!("Malformed link found while parsing - ignoring and continuing.");
            return;
        },
    };
    let content: &str = &line[(*i + 1)..(bracket_end)];
    if line.chars().collect::<Vec<char>>()[bracket_end + 1] != '(' {
        println!("Malformed link found while parsing - ignoring and continuing.");
        return;
    }
    let (_, after) = line.split_at(bracket_end + 2);
    let brace_end = 1 + bracket_end + match after.find(')') {
        Some(res) => res,
        None      => {
            println!("Malformed link found while parsing - ignoring and continuing.");
            return;
        },
    };
    let link: String = convert_link(&line[(bracket_end + 2)..(brace_end + 1)], paths, requestpath);
    *hit_index = 2 + brace_end;
    *i = 1 + brace_end;
    tokens.push(Box::new(BraceToken::new(content.to_string(), link)));
}

fn convert_link(link: &str, _paths: &Paths, requestpath: &str) -> String {
    if link.starts_with("http") { return link.to_string(); }
    if link.starts_with("/") {
        // Absolute Path
        let (relativepath, filename) = match link.rsplit_once("/") {
            Some(split) => split,
            None   => ("", requestpath),
        };
        return relativepath.to_string() + "/.link/" + filename;
    } else {
        // Relative Path
        let (relativepath, _) = match requestpath.rsplit_once("/") {
            Some(split) => split,
            None   => ("", requestpath),
        };
        let (partpath, filename) = match link.rsplit_once("/") {
            Some(split) => split,
            None   => ("", link),
        };
        return relativepath.to_string() + partpath + "/.link/" + filename;
    }
}

fn consume_literal(line: &str, tokens: &mut Vec<Box<dyn Token>>, hit_index: &mut usize, i: usize, paths: &Paths, requestpath: &str) -> () {
    let content = line.split_at(*hit_index).1.split_at(i - hit_index.to_owned()).0;
    if content == "" {
        *hit_index = i + 1;
        return;
    }

    let absolutepath: String;
    if content.starts_with("/") {
        absolutepath = content.to_string();
    } else {
        let relativepath = match requestpath.rsplit_once('/') {
            Some(path) => path,
            None   => return,
        }.0;
        absolutepath = relativepath.to_string() + "/" + content;
    }

    let mut close = false;
    if paths.contains(&absolutepath) {
        tokens.push(Box::new(LinkToken::new(absolutepath.clone(), false)));
        close = true;
    }

    tokens.push(Box::new(LiteralToken::new(content.to_owned())));
    *hit_index = i + 1;

    if close {
        tokens.push(Box::new(LinkToken::new(absolutepath.clone(), true)));
    }
}

fn parse_header(line: &str) -> String {
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

fn remove_char_from_start(line: &String, c: char) -> String {
    let line = line.trim_start();
    let (_, line) = match line.split_once(c) {
        Some(split) => split,
        None        => ("", line),
    };
    line.trim_start().to_string()
}

fn remove_num_chars_from_start(line: &String, counter: usize) -> String {
    let line = line.trim_start();
    let (_, line) = line.split_at(counter);
    line.trim_start().to_string()
}

fn remove_last(tokens: &mut Vec<Box<dyn Token>>, tokentype: TokenType) -> () {
    for i in (0..tokens.len()).rev() {
        if tokens[i].tokentype() == &tokentype {
            tokens[i] = tokens[i].literal_replace();
            break;
        }
    }
}

#[derive(PartialEq)]
pub enum Status {
    BlockQuote(usize),
    Empty,
    Header,
    OrderedList(usize),
    Paragraph,
    UnorderedList(usize),
}
