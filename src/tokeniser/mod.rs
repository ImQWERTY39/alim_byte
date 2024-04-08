#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Character(char),
    String(String),
}

pub fn tokenise(program: &str) -> Vec<Vec<Token>> {
    program
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .filter(|x| !x.starts_with(';'))
        .map(tokenise_line)
        .collect()
}

fn tokenise_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut argument = String::new();
    let mut in_string = false;
    let mut in_character = false;

    for chr in line.chars() {
        match chr {
            ';' => break,
            ' ' if !in_string => {
                if !argument.is_empty() {
                    tokens.push(argument.into());
                    argument = String::new();
                }
            }
            '"' if !in_string && !in_character => {
                argument.push('"');
                in_string = true;
            }
            '"' if in_string => {
                in_string = false;

                argument.push('"');
                tokens.push(argument.into());
                argument = String::new();
            }
            '\'' if !in_string && !in_character => {
                argument.push('\'');
                in_character = true;
            }
            '\'' if in_character => {
                in_character = false;

                argument.push('\'');
                tokens.push(argument.into());
                argument = String::new();
            }
            i => argument.push(i),
        }
    }

    if !argument.trim().is_empty() {
        tokens.push(argument.into());
    }

    tokens
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        if let Ok(i) = value.parse::<i64>() {
            return Self::Integer(i);
        }

        if let Ok(i) = value.parse::<f64>() {
            return Self::Float(i);
        }

        if value == "true" {
            return Self::Boolean(true);
        }

        if value == "false" {
            return Self::Boolean(false);
        }

        if value.starts_with('\'') && value.ends_with('\'') {
            return Self::Character(parse_character(value).unwrap());
        }

        if value.starts_with('\"') && value.ends_with('\"') {
            return Self::String(parse_string(value).unwrap());
        }

        if valid_identifier(&value) {
            return Self::Identifier(value);
        }

        panic!()
    }
}

fn parse_character(value: String) -> Result<char, ()> {
    if (value.len() != 3) && (value.len() == 4 && value.chars().nth(1).unwrap() != '\\') {
        return Err(());
    }

    if value.len() == 4 {
        match value.chars().nth(2).unwrap() {
            'n' => return Ok('\n'),
            't' => return Ok('\t'),
            'r' => return Ok('\r'),
            '\\' => return Ok('\\'),
            '\'' => return Ok('\''),
            '\"' => return Ok('\"'),
            '0' => return Ok('\0'),
            _ => return Err(()),
        }
    }

    Ok(value.chars().nth(1).unwrap())
}

fn parse_string(value: String) -> Result<String, ()> {
    let mut current_index = 1;
    let mut string = String::with_capacity(value.len() - 2);
    let mut characters = value.chars();
    characters.next();

    while current_index < value.len() - 1 {
        let current_chr = characters.next().ok_or(())?;

        if current_chr == '\\' {
            let next = characters.next().ok_or(())?;
            string.push(parse_character(format!("'\\{}'", next))?);
            current_index += 1;
        } else {
            string.push(current_chr);
        }

        current_index += 1;
    }

    Ok(string)
}

fn valid_identifier(value: &str) -> bool {
    let valid_first_character = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
    let valid_characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

    let mut characters = value.chars();

    if !valid_first_character.contains(characters.next().unwrap()) {
        return false;
    }

    for i in characters {
        if !valid_characters.contains(i) {
            return false;
        }
    }

    true
}
