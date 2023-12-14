use colored::Colorize;
use std::{
    env::Args,
    error::Error,
    fmt::Debug,
    fs,
    io::{self, Write},
    usize,
};

pub fn start(mut args: Args, fallo: bool) -> Result<(), Box<dyn Error>> {
    args.next();
    match args.len() {
        2.. => Err("Uso: rjox <script>".into()),
        1 => Ok(run_file(
            args.next().expect("No se ha encontrado el archivo"),
            fallo,
        )?),
        _ => Ok(run_prompt(fallo)?),
    }
}

fn run_file(file_path: String, fallo: bool) -> Result<(), Box<dyn Error>> {
    let mut content = fs::read_to_string(file_path)?;
    run(&mut content, &fallo);
    if fallo {
        return Err("Existe un error en el contenido".into());
    }
    Ok(())
}

fn run(content: &mut str, fallo: &bool) {
    let mut scanner = Scanner::new(String::from(content));
    let tokens = scanner.scan_tokens(*fallo);
    dbg!("{}", &tokens);
    for token in tokens {
        dbg!("{}", token);
    }
}

fn run_prompt(mut fallo: bool) -> Result<(), Box<dyn Error>> {
    loop {
        print_prompt();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break
        }
        run(&mut buffer, &fallo);
        fallo = false;
    }
    Ok(())
}

fn print_prompt() {
    print!("{}", ">> ".bold().green());
    io::stdout().flush().unwrap();
}

fn report_error (line: usize, place: String, message: String, fallo: &mut bool) {
    eprintln!("[{line}] | Error {place}: {message}");
    *fallo = true;
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
    For, If, Nil, Or,
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

    pub fn scan_tokens(&mut self, mut fallo: bool) -> Vec<Token> {
        while self.source.len() > self.current {
            self.start = self.current;
            self.scan_token(&mut fallo);
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line,
        });

        self.tokens.clone()
    }

    fn scan_token(&mut self, fallo: &mut bool) {
        let chars = self.source.chars().collect::<Vec<_>>();
        for char in &chars {
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
                '!' => {
                    let c = if self.check('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(c);
                },
                '=' => {
                    let c = if self.check('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(c);
                }
                '<' => {
                    let c = if self.check('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(c);
                }
                '>' => {
                    let c = if self.check('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(c);
                }
                '/' => {
                    if self.check('/') {
                        while self.peek() != '\n' && self.source.len() > self.current {
                            self.current += 1;
                            let _ = chars[self.current];
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    };
                }
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => self.line += 1,
                _ => report_error(self.line, String::from(""), String::from("Caracter desconocido"), fallo),
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

    fn check(&mut self, expected: char) -> bool {
        if self.source.len() <= self.current {
            return false;
        }
        let chars = self.source.chars().collect::<Vec<_>>();
        if chars[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.source.len() <= self.current {
            return '\0';
        }
        let chars = self.source.chars().collect::<Vec<_>>();
        chars[self.current]
    }
}
