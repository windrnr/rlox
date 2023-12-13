use colored::Colorize;
use std::{
    env::Args,
    error::Error,
    fmt::Debug,
    fs,
    io::{self, Write},
    usize,
};

pub struct Lox {
    pub had_error: bool,
}

pub fn start(lox: Lox, mut args: Args) -> Result<(), Box<dyn Error>> {
    args.next();
    match args.len() {
        2.. => Err("Uso: rjox <script>".into()),
        1 => Ok(run_file(
            lox,
            args.next().expect("No se ha encontrado el archivo"),
        )?),
        _ => Ok(run_prompt(lox)?),
    }
}

fn run_file(lox: Lox, file_path: String) -> Result<(), Box<dyn Error>> {
    let mut content = fs::read_to_string(file_path)?;
    run(&mut content);
    if lox.had_error {
        return Err("Existe un error en el contenido".into());
    }
    Ok(())
}

fn run(content: &mut str) {
    let mut scanner = Scanner::new(String::from(content));
    let tokens = scanner.scan_tokens();
    for token in tokens {
        dbg!("{:?}", &token);
    }
}

fn run_prompt(mut lox: Lox) -> Result<(), Box<dyn Error>> {
    loop {
        print_prompt();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        run(&mut buffer);
        lox.had_error = false;
    }
    Ok(())
}

fn print_prompt() {
    print!("{}", ">> ".bold().green());
    io::stdout().flush().unwrap();
}

fn report(mut lox: Lox, line: u32, place: String, message: String) {
    eprintln!("[{line}] | Error {place}: {message}");
    lox.had_error = true;
}

fn error(lox: Lox, line: u32, message: String) {
    report(lox, line, String::from(""), message);
}

// -----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Value(Option<LiteralDef>),
}

#[derive(Debug, Clone)]
pub enum LiteralDef {
    String(String),
    Float(f32),
    Integer(u32),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) {
        format!("{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal);
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while self.source.len() > self.current {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line,
        });
        let tokens = self.tokens.clone();
        return tokens;
    }

    fn scan_token(&mut self) {
        let chars = self.source.chars().collect::<Vec<_>>();
        for char in chars {
            self.current += 1;
            match char {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                _ => (),
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..=self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }
}
