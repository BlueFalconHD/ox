// scanner.rs

#[derive(Debug, Clone)]
pub enum TokenType {
    // Keywords
    Fn,
    Let,
    Where,
    Switch,
    As,
    Default,
    Use,
    Struct,
    Mut,
    Range,
    Enum,
    Impl,

    // Operators and Delimiters
    Eq,
    Greater,
    Less,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Colon,
    Comma,
    Dot,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Bang,
    Hash,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    // 2 character operators
    Arrow,
    EqEq,
    Neq,
    Geq,
    Leq,
    And,
    Or,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    DoubleColon,
    DoubleDot,
    HashHash,

    // Literals
    Identifier,
    String,
    Float,
    Integer,

    // Special
    Eof,
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::Fn);
        m.insert("let", TokenType::Let);
        m.insert("where", TokenType::Where);
        m.insert("switch", TokenType::Switch);
        m.insert("as", TokenType::As);
        m.insert("default", TokenType::Default);
        m.insert("use", TokenType::Use);
        m.insert("struct", TokenType::Struct);
        m.insert("mut", TokenType::Mut);
        m.insert("range", TokenType::Range);
        m.insert("enum", TokenType::Enum);
        m.insert("impl", TokenType::Impl);
        m
    };
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: u32,
        column: u32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Token: {:?}, Lexeme: {}, Literal: {:?}, Line: {}, Column: {}",
            self.token_type, self.lexeme, self.literal, self.line, self.column
        )
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
            self.column,
        ));
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self
            .advance()
            .ok_or_else(|| self.error(self.line, "Unexpected end of input"))?;
        match c {
            '(' => self.add_token(TokenType::Lparen),
            ')' => self.add_token(TokenType::Rparen),
            '{' => self.add_token(TokenType::Lbrace),
            '}' => self.add_token(TokenType::Rbrace),
            '[' => self.add_token(TokenType::Lbracket),
            ']' => self.add_token(TokenType::Rbracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),

            '-' => {
                if self.match_char('=') {
                    self.add_token(TokenType::MinusEq);
                } else if self.match_char('-') {
                    self.add_token(TokenType::MinusMinus);
                } else if self.match_char('>') {
                    self.add_token(TokenType::Arrow);
                } else {
                    self.add_token(TokenType::Minus);
                }
            }

            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqEq);
                } else {
                    self.add_token(TokenType::Eq);
                }
            }

            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Neq);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }

            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Geq);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::Leq);
                } else {
                    self.add_token(TokenType::Less);
                }
            }

            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::And);
                } else {
                    self.add_token(TokenType::BitwiseAnd);
                }
            }

            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                } else {
                    self.add_token(TokenType::BitwiseOr);
                }
            }

            '+' => {
                if self.match_char('=') {
                    self.add_token(TokenType::PlusEq);
                } else if self.match_char('+') {
                    self.add_token(TokenType::PlusPlus);
                } else {
                    self.add_token(TokenType::Plus);
                }
            }

            '*' => {
                if self.match_char('=') {
                    self.add_token(TokenType::StarEq);
                } else {
                    self.add_token(TokenType::Star);
                }
            }

            '/' => {
                if self.match_char('=') {
                    self.add_token(TokenType::SlashEq);
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '%' => {
                if self.match_char('=') {
                    self.add_token(TokenType::PercentEq);
                } else {
                    self.add_token(TokenType::Percent);
                }
            }

            ':' => {
                if self.match_char(':') {
                    self.add_token(TokenType::DoubleColon);
                } else {
                    self.add_token(TokenType::Colon);
                }
            }

            '#' => {
                if self.match_char('#') {
                    self.add_token(TokenType::HashHash);
                } else {
                    self.add_token(TokenType::Hash);
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.column = 0;
            }

            '"' => self.string()?,

            _ if c.is_digit(10) => {
                if c == '0' {
                    if self.peek() == 'x' {
                        self.advance();
                        self.hex_number()?;
                    } else if self.peek() == 'b' {
                        self.advance();
                        self.binary_number()?;
                    } else {
                        self.number()?;
                    }
                } else {
                    self.number()?;
                }
            }

            _ => {
                if self.is_alphanumeric(c) {
                    self.identifier();
                } else {
                    return Err(self.error(self.line, &format!("Unexpected character: {}", c)));
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let c = self.source.chars().nth(self.current)?;
        self.current += 1;
        self.column += 1;
        Some(c)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, None, self.line, self.column));
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(
            token_type,
            text,
            Some(literal),
            self.line,
            self.column,
        ));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.error(self.line, "Unterminated string"));
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, value);
        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].to_string();
        self.add_token_literal(TokenType::Float, value);
        Ok(())
    }

    fn hex_number(&mut self) -> Result<(), String> {
        while self.peek().is_digit(16) {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].to_string();
        self.add_token_literal(
            TokenType::Integer,
            i64::from_str_radix(&value, 16)
                .map_err(|_| self.error(self.line, "Invalid hex number"))?
                .to_string(),
        );
        Ok(())
    }

    fn binary_number(&mut self) -> Result<(), String> {
        while self.peek() == '0' || self.peek() == '1' {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].to_string();
        self.add_token_literal(
            TokenType::Integer,
            i64::from_str_radix(&value, 2)
                .map_err(|_| self.error(self.line, "Invalid binary number"))?
                .to_string(),
        );
        Ok(())
    }

    fn identifier(&mut self) -> () {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        // Check if the identifier is a reserved keyword using the KEYWORDS hashmap
        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn error(&self, line: u32, message: &str) -> String {
        let mut error_message = format!("[line {}] Error: {}", line, message);
        if let Some(line_content) = self.source.lines().nth(line as usize - 1) {
            error_message.push_str(&format!("\n{} | {}", line, line_content));
            error_message.push_str(&format!("\n    | {:>1$}", "^", self.column as usize));
        }

        // println!("{}", error_message);
        println!("{}", error_message);

        error_message
    }
}
