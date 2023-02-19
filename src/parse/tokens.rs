use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum Keyword {
    Func,
    Return,
    Drop,
    End,
    Do,
    Then,

    If,
    While,
    For,
    Repeat,
    Until,

    // bool operators
    Not,
    And,
    Or,

    // constants
    True,
    False,
    Null,
}

pub struct ParseKeywordErr;

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Keyword::Func => "func",
            Keyword::Return => "return",
            Keyword::Drop => "drop",
            Keyword::End => "end",
            Keyword::Do => "do",
            Keyword::Then => "then",
            Keyword::If => "if",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Repeat => "repeat",
            Keyword::Until => "until",
            Keyword::Not => "not",
            Keyword::And => "and",
            Keyword::Or => "or",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null"
        })?;
        Ok(())
    }
}

impl std::str::FromStr for Keyword {
    type Err = ParseKeywordErr;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        match s {
            "func" => Ok(Keyword::Func),
            "return" => Ok(Keyword::Return),
            "drop" => Ok(Keyword::Drop),
            "end" => Ok(Keyword::End),
            "do" => Ok(Keyword::Do),
            "then" => Ok(Keyword::Then),
            "if" => Ok(Keyword::If),
            "while" => Ok(Keyword::While),
            "for" => Ok(Keyword::For),
            "repeat" => Ok(Keyword::Repeat),
            "until" => Ok(Keyword::Until),
            "not" => Ok(Keyword::Not),
            "and" => Ok(Keyword::And),
            "or" => Ok(Keyword::Or),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "null" => Ok(Keyword::Null),
            _ => Err(ParseKeywordErr)
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Symbol(char),
    Identifier(String),
    Keyword(Keyword),
    Number(f64),
    String(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Symbol(ch) => write!(f, "{}", ch),
            Token::Identifier(id) => write!(f, "{}", id),
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(str) => write!(f, "{}", str)
        }
    }
}

impl Token {
    pub fn typestr(&self) -> &str {
        match self {
            Token::Symbol(_) => "symbol",
            Token::Identifier(_) => "identifier",
            Token::Keyword(_) => "keyword",
            Token::Number(_) => "number",
            Token::String(_) => "string"
        }
    }
}

pub fn read_tokens(file_contents: &String, output: &mut Vec<Token>) {
    let file_bytes = file_contents.as_bytes();

    let mut buffer = String::new();
    let mut flush = false;
    let mut flush_ch: Option<char> = None;
    let mut read_str = false;

    for i in 0..(file_bytes.len() + 1) {
        if i < file_bytes.len() {
            let ch = file_bytes[i] as char;

            if read_str {
                if ch == '"' {
                    output.push(Token::String(buffer.clone()));
                    buffer.clear();
                    read_str = false;
                } else { // TODO escape characters
                    buffer.push(ch);
                }
            } else {
                match ch {
                    // if reached a symbol
                    '(' | ')' | ',' | '+' | '-' | '/' | '*' | ':' => {
                        flush = true;
                        flush_ch = Some(ch);
                    }

                    // if reached a quotation mark
                    '"' => {
                        flush = true;
                        read_str = true;
                    }

                    _ => {
                        if ch.is_whitespace() {
                            flush = true;
                        } else {
                            buffer.push(ch);
                        }
                    }
                }
            }
        } else {
            flush = true;
        }

        if flush {
            // flush identifier/keyword
            if buffer.len() > 0 {
                // if first character is a digit
                if (buffer.as_bytes()[0] as char).is_digit(10) {
                    output.push(Token::Number(buffer.parse::<f64>().unwrap())); // TODO error checking
                } else {
                    match Keyword::from_str(buffer.as_str()) {
                        Ok(kw) => output.push(Token::Keyword(kw)),
                        Err(_) => output.push(Token::Identifier(buffer.clone()))
                    }
                }

                buffer.clear();
            }

            // flush symbol
            if let Some(sym) = flush_ch {
                output.push(Token::Symbol(sym));
            }

            flush = false;
            flush_ch = None;
        }
    }
}