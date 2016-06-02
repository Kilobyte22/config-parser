use super::lexer;
use super::lexer::{TokenType, Token};
use super::config::ConfigBlock;
use super::error::{Result, ErrorType, Error, CodePos};

macro_rules! expect_token {
    ($state:expr) => {
        match next($state) {
            Some(t) => t,
            None => return fail($state, ErrorType::UnexpectedEOF)
        }
    };
    ($state:expr, $ty:expr) => {
        match next($state) {
            Some(Token {token_type: $ty(..), ..} @ t) => t,
            Some(t) => return Err(Error::new(state.last_token.line, state.last_token.col, ErrorType::Unexpected(t)))
            None => return fail($state, ErrorType::UnexpectedEOF)
        }
    }
}

struct ParseState {
    tokens: Box<Iterator<Item=lexer::Token>>,
    last_token: Option<Token>,
    force_next: Option<Token>,
    done: bool
}

impl CodePos for ParseState {
    fn location(&self) -> (u32, u16) {
        match self.last_token {
            Some(ref t) => (t.line, t.col),
            None => (0, 0)
        }
    }
}

pub fn run(tokens: Box<Iterator<Item=lexer::Token>>) -> Result<ConfigBlock> {
    let mut state = ParseState {
        tokens: tokens,
        last_token: None,
        force_next: None,
        done: false
    };

    parse_block(&mut state, false, String::from(""), vec![])
}

fn parse_block(state: &mut ParseState, inner: bool, name: String, options: Vec<String>) -> Result<ConfigBlock> {
    let mut ret = ConfigBlock::new(name, options, vec![]);
    loop {
        let tok = if inner {
            expect_token!(state)
        } else {
            match next(state) {
                Some(t) => t,
                None => return Ok(ret)
            }
        };
        match tok.clone().token_type {
            TokenType::RawLiteral(option_name) => {
                let params = try!(parse_params(state));
                let t = expect_token!(state);
                match t.token_type {
                    TokenType::OpenBrace => {
                        // Block follows
                        ret.add_block(try!(parse_block(state, true, option_name, params)));
                    },
                    _ => {
                        // No block. In strict mode this will only ever execute for 
                        // TokenType::Semicolon as parse_params() will already have
                        // returned an error for other types
                        ret.add_block(ConfigBlock::new(option_name, params, vec![]))
                    }
                }
            },
            TokenType::CloseBrace if inner => break,
            _ => return fail(state, ErrorType::Unexpected(tok.clone()))
        }
    }
    Ok(ret)
}

fn parse_params(state: &mut ParseState) -> Result<Vec<String>> {
    let mut ret = vec![];
    println!("Parsing params");
    loop {
        let opt_t = lookahead(state);
        match opt_t {
            Some(t) => {
                match t.token_type {
                    TokenType::StringLiteral(s) => {
                        ret.push(s);
                        pop(state);
                    },
                    TokenType::RawLiteral(s) => {
                        ret.push(s);
                        pop(state);
                    },
                    TokenType::OpenBrace => break,
                    TokenType::Semicolon => break,
                    _ => {
                        if cfg!(feature = "nonstrict") {
                            break;
                        } else {
                            println!("Errored params");
                            return fail(state, ErrorType::Unexpected(t)) 
                        }
                    }
                }
            },
            None => return fail(&state, ErrorType::UnexpectedEOF)
        }
    }
    println!("Exited params");
    Ok(ret)
}

fn next(state: &mut ParseState) -> Option<lexer::Token> {
    let v = match &state.force_next {
        &Some(ref t) => Some(t.clone()),
        &None => state.tokens.next()
    };
    state.force_next = None;
    println!("Token {:?}", &v);
    match v.clone() {
        Some(_) => {},
        None => {
            if state.done {
                unreachable!("Tried to get another token after end of stream");
            } else {
                state.done = true;
            }
        }
    }
    v
}

fn pop(state: &mut ParseState) {
    println!("popping...");
    if state.force_next.is_some() {
        state.force_next = None;
    } else {
        state.tokens.next();
    }
}

fn lookahead(state: &mut ParseState) -> Option<lexer::Token> {
    let r = match state.force_next.clone() {
        Some(t) => Some(t),
        None => {
            let t = state.tokens.next();
            state.force_next = t.clone();
            t
        }
    };
    println!("Lookahead Token {:?}", &r);
    r
}

fn fail<T>(state: &ParseState, error_type: ErrorType) -> Result<T> {
    Err(Error::from_state(state, error_type))
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::lexer::{Token, TokenType};
    use super::super::config::ConfigBlock;

    #[test] 
    fn test_it_parsing_the_most_basic_option() {
        assert_eq!(
            run(Box::new(vec![
                tok(TokenType::RawLiteral(String::from("test"))),
                tok(TokenType::Semicolon)
            ].into_iter())),
                Ok(ConfigBlock::new(
                    String::new(),
                    vec![],
                    vec![
                        ConfigBlock::new(
                            String::from("test"),
                            vec![],
                            vec![]
                        )
                    ]
            )));
    }

    #[test]
    fn test_it_parsing_a_typical_example() {
        assert_eq!(
            run(Box::new(vec![
                tok(TokenType::RawLiteral(String::from("option"))),
                tok(TokenType::RawLiteral(String::from("param1"))),
                tok(TokenType::OpenBrace),
                tok(TokenType::RawLiteral(String::from("inner"))),
                tok(TokenType::StringLiteral(String::from("value"))),
                tok(TokenType::Semicolon),
                tok(TokenType::CloseBrace),
            ].into_iter())),
                Ok(ConfigBlock::new(
                String::new(),
                vec![],
                vec![
                    ConfigBlock::new(
                        String::from("option"),
                        vec![String::from("param1")],
                        vec![
                            ConfigBlock::new(
                                String::from("inner"),
                                vec![String::from("value")],
                                vec![]
                            )
                        ]
                    )
                ]
            )));
    }

    fn tok(ty: TokenType) -> Token {
        Token::new(0, 0, ty)
    }
}
