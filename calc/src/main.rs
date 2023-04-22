#[derive(Debug)]
enum TokenType {
    LeftParent(String),
    RightParent(String),
    Plus(String),
    Minus(String),
    Multiply(String),
    Div(String),
    Mod(String),
    Pow(String),
    Num(String, i64),
}

#[derive(Debug)]
enum Error {
    ParentNotMatch,
    ParseNumFailed(String),
}

#[derive(Debug)]
struct Scanner {
    chars: Vec<char>,
    start: i32,
    current: i32,
}

struct Execute {
    tokens: Vec<TokenType>,
    result_stack: Vec<i64>,
    op_stack: Vec<TokenType>,
    current: i32,
}

impl Execute {
    fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
            result_stack: Vec::<i64>::new(),
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
        };
    }

    fn scan_tokens(&mut self) -> Vec<TokenType> {
        let mut tokens = Vec::<TokenType>::new();
        return tokens;
    }

    fn parse_num(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
