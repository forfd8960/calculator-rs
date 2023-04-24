use std::{
    collections::{HashMap, VecDeque},
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq)]
enum TokenType {
    LeftParent(String),
    RightParent(String),
    Plus(String),
    Minus(String),
    Multiply(String),
    Div(String),
    Pow(String),
    Num(String, f64),
    Dot(String),
    Eof,
}

impl TokenType {
    fn get_string(&self) -> String {
        match self {
            Self::LeftParent(v) | Self::RightParent(v) => v.clone(),
            Self::Plus(v) | Self::Minus(v) => v.clone(),
            Self::Multiply(v) | Self::Div(v) => v.clone(),
            Self::Pow(v) | Self::Num(v, _) => v.clone(),
            Self::Dot(_) => ".".to_string(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    TokenNotMatch,
    InvalidExperssion,
    InvalidOperator,
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
    result_stack: VecDeque<f64>, // push_back , pop_back
    op_stack: VecDeque<String>,
    current: usize,
    priority_map: HashMap<String, u8>,
}

impl Execute {
    fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
            result_stack: VecDeque::<f64>::new(),
            op_stack: VecDeque::<String>::new(),
            priority_map: HashMap::from([
                (")".to_string(), 6),
                ("^".to_string(), 5),
                ("*".to_string(), 4),
                ("/".to_string(), 3),
                ("+".to_string(), 2),
                ("-".to_string(), 1),
                ("(".to_string(), 0),
            ]),
        }
    }

    fn compare_prec(&self, op1: String, op2: String) -> bool {
        false
    }

    fn run(&mut self) -> Result<f64, Error> {
        let tk_len = self.tokens.len();

        loop {
            let tk = self.tokens.get(self.current).unwrap();
            match tk {
                TokenType::Num(_, num) => {
                    self.result_stack.push_back(*num);
                    self.current += 1;
                }

                TokenType::LeftParent(_) => {
                    self.op_stack.push_back(tk.get_string());
                    self.current += 1;
                }

                TokenType::RightParent(_) => {
                    self.pop_util_left_parent()?;
                }

                _ => self.push_op(tk.get_string())?,
            };

            if self.current >= tk_len {
                break;
            }
        }

        self.pop_all_operators()?;

        if let Some(v) = self.result_stack.pop_back() {
            return Ok(v);
        }

        Err(Error::InvalidExperssion)
    }

    fn pop_util_left_parent(&mut self) -> Result<(), Error> {
        loop {
            let tk = self.op_stack.pop_back();
            if tk.is_none() {
                return Err(Error::TokenNotMatch);
            }

            let op = tk.unwrap();

            if op.as_str() == "(" {
                break;
            }

            if let Err(e) = self.calculate(op) {
                return Err(e);
            }
        }

        self.current += 1;
        Ok(())
    }

    fn push_op(&mut self, tk: String) -> Result<(), Error> {
        let op = tk;
        if let Some(p) = self.priority_map.get(&op) {
            // self.op_stack.push_back(op);

            let current_op = || -> &str {
                if self.op_stack.len() == 0 {
                    return "";
                }

                self.op_stack.get(self.op_stack.len() - 1).unwrap()
            }();

            if current_op == "" {
                self.current += 1;
                self.op_stack.push_back(op);
                return Ok(());
            }

            let cur_op_pri = self.priority_map.get(current_op).unwrap();
            if cur_op_pri >= p {
                self.calculate(current_op.to_string())?;
                self.op_stack.pop_back();
            }

            self.current += 1;
            self.op_stack.push_back(op);
            return Ok(());
        }

        Err(Error::TokenNotSupported(format!("{} is not supported", op)))
    }

    fn pop_all_operators(&mut self) -> Result<(), Error> {
        if self.op_stack.len() == 0 {
            return Ok(());
        }

        if let Some(op) = self.op_stack.pop_back() {
            let rs = self.calculate(op);
            return rs;
        }

        Ok(())
    }

    fn calculate(&mut self, op: String) -> Result<(), Error> {
        let num1 = self.result_stack.pop_back();
        if num1.is_none() {
            return Err(Error::InvalidExperssion);
        }

        let num2 = self.result_stack.pop_back();
        if num2.is_none() {
            return Err(Error::InvalidExperssion);
        }

        let (v1, v2) = (num1.unwrap(), num2.unwrap());
        let mut rs = Ok(());
        match op.as_str() {
            "^" => {
                self.result_stack.push_back(v1.powf(v2));
            }
            "*" => {
                self.result_stack.push_back(v1.mul(v2));
            }
            "/" => {
                self.result_stack.push_back(v1 / v2);
            }
            "+" => {
                self.result_stack.push_back(v1.add(v2));
            }
            "-" => {
                self.result_stack.push_back(v1 - v2);
            }
            _ => rs = Err(Error::InvalidOperator),
        }

        rs
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
            ' ' | '\r' | '\n' | '\t' => {}
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
