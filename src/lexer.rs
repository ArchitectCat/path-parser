use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Token {
    Separator,
    WildChar,
    OpenBracket,
    CloseBracket,
    Numeric(String),
    Identifier(String),
}

fn get_numeric<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    let number = c.to_string().parse::<u32>().expect("Expecting numeric value");
    let mut result = number.to_string();

    while let Some(Ok(digit)) = iter.peek().map(|x| x.to_string().parse::<u32>()) {
        result.push(char::from_digit(digit, 10).unwrap());
        iter.next();
    }
    result
}

fn is_allowed_character(c: &char) -> bool {
    return match c {
        'a'..='z' | 'A'..='Z' | '-' | '_' | '.' => { true }
        _ => { false }
    }
}

fn get_allowed_characters<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Result<String, String> {
    let fst = is_allowed_character(&c);
    match fst {
        true => {
            let mut result = c.to_string();

            while let Some(ch) = iter.peek().filter(|x| is_allowed_character(x)) {
                result.push(ch.clone());
                iter.next();
            }
            Ok(result)
        }
        false => {
            return Err(format!("Failed to parse character: {}", c));
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '/' => {
                result.push(Token::Separator);
                chars.next();
            }
            '*' => {
                result.push(Token::WildChar);
                chars.next();
            }
            '{' => {
                result.push(Token::OpenBracket);
                chars.next();
            }
            '}' => {
                result.push(Token::CloseBracket);
                chars.next();
            }
            '0'..='9' => {
                chars.next();
                let num = get_numeric(c, &mut chars);
                result.push(Token::Numeric(num));
            }
            _ => {
                chars.next();
                let id = get_allowed_characters(c, &mut chars);
                match id {
                    Ok(v) => {
                        result.push(Token::Identifier(v));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }
    Ok(result)
}