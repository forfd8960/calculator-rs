#[derive(Debug, PartialEq)]
enum TokenType {
    LeftParent(String),
    RightParent(String),
    Plus(String),
    Minus(String),
    Multiply(String),
    Div(String),
    Mod(String),
    Pow(String),
    Num(String, f64),
    Dot(String),
    Eof,
}

#[derive(Debug, PartialEq)]
enum Error {
    ParentNotMatch,
    ParseNumFailed(String),
    TokenNotSupported(String),
}

#[derive(Debug)]
struct Scanner {
    chars: Vec<char>,
    start: i32,
    current: i32,
    tokens: Vec<TokenType>,
}

struct Execute {
    tokens: Vec<TokenType>,
    result_stack: Vec<f64>,
    op_stack: Vec<TokenType>,
    current: i32,
}

impl Execute {
    fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
            result_stack: Vec::<f64>::new(),
            op_stack: Vec::<TokenType>::new(),
        }
    }

    fn run(&self) -> Result<i64, Error> {
        Ok(0)
    }
}

impl Scanner {
    fn new(chars: Vec<char>) -> Self {
        return Self {
            chars: chars,
            start: 0,
            current: 0,
            tokens: Vec::<TokenType>::new(),
        };
    }

    fn scan_tokens(&mut self) -> Result<(), Error> {
        while !self.is_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                return Err(e);
            }
        }

        self.add_token(TokenType::Eof);
        return Ok(());
    }

    fn is_end(&self) -> bool {
        self.current as usize >= self.chars.len()
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let ch = self.advance();
        let mut rs = Ok(());
        match ch {
            '(' => self.add_token(TokenType::LeftParent("(".to_string())),
            ')' => self.add_token(TokenType::RightParent(")".to_string())),
            '*' => self.add_token(TokenType::Multiply("*".to_string())),
            '/' => self.add_token(TokenType::Div("/".to_string())),
            '+' => self.add_token(TokenType::Plus("+".to_string())),
            '-' => self.add_token(TokenType::Minus("-".to_string())),
            '^' => self.add_token(TokenType::Pow("^".to_string())),
            '.' => self.add_token(TokenType::Dot(".".to_string())),
            _ => {
                if ch.is_digit(10) {
                    rs = match self.get_number_token() {
                        Ok(tk) => {
                            self.add_token(tk);
                            Ok(())
                        }
                        Err(e) => Err(e),
                    };
                } else {
                    rs = Err(Error::TokenNotSupported(format!("{} is not supported", ch)));
                }
            }
        }
        rs
    }

    fn add_token(&mut self, tk_type: TokenType) {
        self.tokens.push(tk_type)
    }

    fn advance(&mut self) -> char {
        let current_ch = self.chars.get(self.current as usize).unwrap();
        self.current += 1;
        *current_ch
    }

    fn is_digit(&self, ch: char) -> bool {
        ch.is_digit(10)
    }

    fn get_number_token(&mut self) -> Result<TokenType, Error> {
        while let Some(ch) = self.peek() {
            if !self.is_digit(ch) {
                break;
            }
            self.advance();
        }

        if let Some(ch) = self.peek() {
            if ch == '.' {
                let next_tk = self.peek_next();
                if next_tk.is_some() && self.is_digit(next_tk.unwrap()) {
                    self.advance();

                    while let Some(ch) = self.peek() {
                        if !self.is_digit(ch) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }

        let ches = &self.chars[(self.start as usize)..(self.current as usize)];
        let num_str = String::from_iter(ches);
        match num_str.parse::<f64>() {
            Ok(num) => Ok(TokenType::Num(num_str, num)),
            Err(err) => Err(Error::ParseNumFailed(err.to_string())),
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_end() {
            return None;
        }

        Some(self.chars[self.current as usize])
    }

    fn peek_next(&self) -> Option<char> {
        if (self.current + 1) as usize >= self.chars.len() {
            return None;
        }
        Some(self.chars[(self.current + 1) as usize])
    }

    fn parse_num(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tesst {
    use crate::{Error, Scanner, TokenType};

    #[test]
    fn scan_tokens() {
        let chars = vec!['+', '-', '*', '/'];
        let mut scanner = Scanner::new(chars);
        let rs = scanner.scan_tokens();
        assert_eq!(rs, Ok(()));

        assert_eq!(
            scanner.tokens,
            vec![
                TokenType::Plus("+".to_string()),
                TokenType::Minus("-".to_string()),
                TokenType::Multiply("*".to_string()),
                TokenType::Div("/".to_string()),
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn scan_num_tokens() {
        let chars = vec!['+', '-', '*', '/', '1', '2', '.', '8'];
        let mut scanner = Scanner::new(chars);
        let rs = scanner.scan_tokens();
        assert_eq!(rs, Ok(()));

        /*
        token: Plus("+")
        token: Minus("-")
        token: Multiply("*")
        token: Div("/")
        token: Num("12.8", 12.8)
        token: Eof
                */
        for tk in &scanner.tokens {
            println!("token: {:?}", tk);
        }

        assert_eq!(
            scanner.tokens,
            vec![
                TokenType::Plus("+".to_string()),
                TokenType::Minus("-".to_string()),
                TokenType::Multiply("*".to_string()),
                TokenType::Div("/".to_string()),
                TokenType::Num("12.8".to_string(), 12.8 as f64),
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn scan_tokens_not_support_token() {
        let chars = vec!['+', '-', '&', '/', '1', '2', '.', '8'];
        let mut scanner = Scanner::new(chars);
        let rs = scanner.scan_tokens();
        assert_eq!(
            rs,
            Err(Error::TokenNotSupported("& is not supported".to_string()))
        );
    }
}
