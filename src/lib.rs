use colored::Colorize;
use std::{
    collections::hash_map::HashMap,
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
            break;
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

fn report_error(line: usize, place: String, message: String, fallo: &mut bool) {
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

// #[derive(Debug, Clone)]
// pub enum Literal {
//     Value(Option<LiteralDef>),
// }
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    None,
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

fn load_keywords() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("class".to_string(), TokenType::Class);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("fun".to_string(), TokenType::Fun);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("nil".to_string(), TokenType::Nil);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("print".to_string(), TokenType::Print);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords.insert("super".to_string(), TokenType::Super);
    keywords.insert("this".to_string(), TokenType::This);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("var".to_string(), TokenType::Var);
    keywords.insert("while".to_string(), TokenType::While);
    keywords
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: load_keywords(),
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
        let c = chars[self.current];
        self.current += 1;
        match c {
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
                let c = if self.check('=', &chars) {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(c);
            }
            '=' => {
                let c = if self.check('=', &chars) {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(c);
            }
            '<' => {
                let c = if self.check('=', &chars) {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(c);
            }
            '>' => {
                let c = if self.check('=', &chars) {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(c);
            }
            '/' => {
                if self.check('/', &chars) {
                    while self.peek(&chars) != '\n' && self.source.len() > self.current {
                        let _ = chars[self.current];
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.handle_string(&chars, fallo),
            '0'..='9' => self.handle_number(&chars),
            'o' => {
                if self.check('r', &chars) {
                    self.add_token(TokenType::Or)
                }
            }
            _ => {
                if c.is_alphabetic() || c == '_' {
                    self.identifier(&chars)
                } else {
                    report_error(
                        self.line,
                        String::from(""),
                        String::from("Caracter desconocido"),
                        fallo,
                    )
                }
            }
        }
    }

    fn handle_number(&mut self, vec: &[char]) {
        while self.peek(vec).is_ascii_digit() {
            let _ = vec[self.current];
            self.current += 1;
        }
        if self.peek(vec) == '.' && self.peek_next(vec).is_ascii_digit() {
            let _ = vec[self.current];
            self.current += 1;
            while self.peek(vec).is_ascii_digit() {
                let _ = vec[self.current];
                self.current += 1;
            }
        }

        let string: String = vec[self.start..=self.current].iter().collect();
        let number = string
            .trim()
            .parse::<f64>()
            .expect("La conversión de string a float ha fallado");
        self.add_token_literal(TokenType::Number, Some(Literal::Number(number)))
    }

    fn handle_string(&mut self, vec: &[char], fallo: &mut bool) {
        while self.peek(vec) != '"' && self.source.len() > self.current {
            if self.peek(vec) == '\n' {
                self.line += 1;
            }
            let _ = vec[self.current];
            self.current += 1;
        }
        if self.source.len() <= self.current {
            report_error(
                self.line,
                String::from(""),
                String::from("String sin cerrar"),
                fallo,
            );
            return;
        }
        let _ = vec[self.current];
        self.current += 1;

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, Some(Literal::String(value.to_string())));
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

    fn check(&mut self, expected: char, vec: &[char]) -> bool {
        if self.source.len() <= self.current {
            return false;
        }
        if vec[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self, vec: &[char]) -> char {
        if self.source.len() <= self.current {
            return '\0';
        }
        vec[self.current]
    }

    fn peek_next(&self, vec: &[char]) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        vec[self.current + 1]
    }

    fn identifier(&mut self, vec: &[char]) {
        while self.peek(vec).is_alphanumeric() || self.peek(vec) == '_' {
            let _ = vec[self.current];
            self.current += 1;
        }
        let binding = vec[self.start..=self.current].iter().collect::<String>();
        let text = binding.trim();

        match self.keywords.get(text) {
            Some(t) => self.add_token(t.clone()),
            None => self.add_token(TokenType::Identifier),
        };
    }
}
