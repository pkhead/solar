use std::collections::VecDeque;
use super::tokens::{Token, Keyword};

#[derive(Debug)]
pub enum AstExpression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    Neg(Box<AstExpression>),
    Add(Box<AstExpression>, Box<AstExpression>),
    Sub(Box<AstExpression>, Box<AstExpression>),
    Mul(Box<AstExpression>, Box<AstExpression>),
    Div(Box<AstExpression>, Box<AstExpression>),
    Join(Box<AstExpression>, Box<AstExpression>),

    Not(Box<AstExpression>),
    And(Box<AstExpression>, Box<AstExpression>),
    Or(Box<AstExpression>, Box<AstExpression>)
}

#[derive(Debug)]
pub enum AstStatement {
    Drop(AstExpression),
    Return(AstExpression),
    Do(AstBlock),
    If(AstExpression, AstBlock),
    While(AstExpression, AstBlock),
    // TODO For
    // TODO Repeat
}

#[derive(Debug)]
pub struct AstBlock {
    statements: Vec<AstStatement>
}

#[derive(Debug)]
pub struct AstFunc {
    id: String,
    block: AstBlock,    
}

#[derive(Debug)]
pub struct AstProgram {
    functions: Vec<AstFunc>
}

// TODO line number, offset number
pub struct ParseError {
    pub msg: String
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        ParseError {
            msg
        }
    }
}

macro_rules! tokexpect {
    ($tok:expr, $search:pat, $cmd:expr) => {
        match $tok {
            Some($search) => $cmd,
            Some(tok) => return Err(ParseError::new(format!("unexpected {}", tok))),
            None => return Err(ParseError::new("unexpected eof".to_string()))
        }
    };

    ($tok:expr, $search:pat, $cmd:block) => {
        match $tok {
            Some($search) => $block,
            Some(tok) => return Err(ParseError::new(format!("unexpected {}", tok))),
            None => return Err(ParseError::new("unexpected eof".to_string()))
        }
    };
}

fn parse_factor(tokens: &mut VecDeque<Token>) -> Result<AstExpression, ParseError> {
    match tokens.pop_front() {
        None => return Err(ParseError::new("unexpected eof".to_string())),

        // if token is the open paren
        Some(Token::Symbol('(')) => {
            let exp = parse_expr(tokens); // parse expression inside parens
            tokexpect!(tokens.pop_front(), Token::Symbol(')'), {});
            return exp;
        },

        // if token is the unary negation operator
        Some(Token::Symbol('-')) => {
            return Ok(AstExpression::Neg(Box::new(parse_factor(tokens)?)));
        },

        // if token is the not operator
        Some(Token::Keyword(Keyword::Not)) => return Ok(AstExpression::Not(Box::new(parse_factor(tokens)?))),
        
        // if token is a number
        Some(Token::Number(num)) => return Ok(AstExpression::Number(num)),

        // if token is a string
        Some(Token::String(string)) => return Ok(AstExpression::String(string)),

        // if token is a boolean
        Some(Token::Keyword(Keyword::True)) => return Ok(AstExpression::Boolean(true)),
        Some(Token::Keyword(Keyword::False)) => return Ok(AstExpression::Boolean(false)),

        // if token is null
        Some(Token::Keyword(Keyword::Null)) => return Ok(AstExpression::Null),

        // unexpected token
        Some(tok) => return Err(ParseError::new(format!("unexpected {}", tok)))
    }
}

fn parse_term(tokens: &mut VecDeque<Token>) -> Result<AstExpression, ParseError> {
    let mut factor = parse_factor(tokens)?;

    // check next token, but don't pop it off yet
    'outer: loop {
        match tokens.get(0) {
            Some(Token::Symbol('*')) => {
                tokens.pop_front(); // pop multiply symbol
                factor = AstExpression::Mul(Box::new(factor), Box::new(parse_factor(tokens)?));
            },

            Some(Token::Symbol('/')) => {
                tokens.pop_front(); // pop division symbol
                factor = AstExpression::Div(Box::new(factor), Box::new(parse_factor(tokens)?));
            }

            _ => break 'outer
        }
    }

    Ok(factor)
}

fn parse_expr(tokens: &mut VecDeque<Token>) -> Result<AstExpression, ParseError> {
    let mut term = parse_term(tokens)?;

    // check next token, but don't pop it off yet
    'outer: loop {
        match tokens.get(0) {
            Some(Token::Symbol('+')) => {
                tokens.pop_front(); // pop plus symbol
                term = AstExpression::Add(Box::new(term), Box::new(parse_term(tokens)?));
            },

            Some(Token::Symbol('-')) => {
                tokens.pop_front(); // pop minus sign
                term = AstExpression::Sub(Box::new(term), Box::new(parse_term(tokens)?));
            },

            // TODO join

            _ => break 'outer
        }
    }

    return Ok(term);
}

fn parse_block(tokens: &mut VecDeque<Token>) -> Result<AstBlock, ParseError> {
    let mut statements: Vec<AstStatement> = Vec::new();

    // TODO
    // if first token is a ":", then block only contains one statement
    // otherwise, continue until "end" keyword is reached
    loop {
        // pop token
        match tokens.pop_front() {
            // if token exists
            Some(tok) => {
                match tok {
                    // "end" keyword marks end of block
                    Token::Keyword(Keyword::End) => {
                        break;
                    },

                    // return <expr>
                    Token::Keyword(Keyword::Return) => {
                        let statement = AstStatement::Return(match parse_expr(tokens) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        });

                        statements.push(statement);
                    },

                    // drop <expr>
                    Token::Keyword(Keyword::Drop) => {
                        let statement = AstStatement::Drop(match parse_expr(tokens) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        });

                        statements.push(statement);
                    },

                    // if <expr> then <block>
                    // TODO if <expr>: <statement>
                    Token::Keyword(Keyword::If) => {
                        let cond = parse_expr(tokens)?;
                        tokexpect!(tokens.pop_front(), Token::Keyword(Keyword::Then), {});
                        let block = parse_block(tokens)?;

                        statements.push(AstStatement::If(cond, block));
                    }

                    tok => return Err(ParseError::new(format!("expected statement, got \"{}\"", tok))),
                }
            },

            // if reached eof
            None => return Err(ParseError::new("unexpected eof".to_string())),
        }
    }

    Ok(AstBlock {
        statements
    })
}

// parse program
pub fn parse_ast(tokens: &mut VecDeque<Token>) -> Result<AstProgram, ParseError> {
    let mut functions = Vec::<AstFunc>::new();

    loop {
        // if there is a token
        if let Some(tok) = tokens.pop_front() {
            match tok {
                Token::Keyword(Keyword::Func) => {
                    // pop an identifier
                    let func_id = tokexpect!(tokens.pop_front(), Token::Identifier(v), v);
 
                    tokexpect!(tokens.pop_front(), Token::Symbol('('), {}); // pop open paren
                    tokexpect!(tokens.pop_front(), Token::Symbol(')'), {}); // pop closed paren
                    
                    let block = match parse_block(tokens) {
                        Ok(v) => v,
                        Err(v) => return Err(v),
                    };

                    functions.push(AstFunc {
                        id: func_id,
                        block: block
                    });
                },

                tok => return Err(ParseError::new(format!("unexpected {} \"{}\"", tok.typestr(), tok)))
            }
        } else {
            break;
        } 
    }

    Ok(AstProgram {
        functions
    })
}