#[derive(Debug, Clone)]
pub enum Token {
    Plus,
    Minus,
    Div,
    Mul,
    OBrkt,
    CBrkt,
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub idx: usize,
    pub ch: char,
    pub toks: Vec<Token>,
    pub lexed: bool,
    buf: String,
}

impl Lexer {
    pub fn from_string(input: String) -> Self {
        Self {
            input: input,
            idx: 0,
            lexed: false,
            toks: Vec::<Token>::new(),
            ch: ' ',
            buf: String::new(),
        }
    }
    pub fn next(&mut self) {
        if self.idx > self.input.len() {
            self.lexed = true;
        } else {
            self.ch = self.input.chars().nth(self.idx).unwrap();
            if self.ch.is_ascii_whitespace() {
                self.next()
            }
            if self.ch.is_ascii_digit() {
                self.buf.push(self.ch);
                self.idx += 1;
                while self.idx < self.input.len() {
                    let next_ch = self.input.chars().nth(self.idx).unwrap();
                    if next_ch.is_ascii_digit() {
                        self.buf.push(next_ch);
                        self.idx += 1;
                    } else {
                        break;
                    }
                }
                let number = self.buf.parse::<f64>().unwrap();
                self.toks.push(Token::Number(number));
                self.buf.clear();
                return;
            }
        }
    }
    pub fn ch_into(&mut self) {
        self.next();
        match self.ch {
            '+' => self.toks.push(Token::Plus),
            '-' => self.toks.push(Token::Minus),
            '/' => self.toks.push(Token::Div),
            '*' => self.toks.push(Token::Mul),
            '(' => self.toks.push(Token::OBrkt),
            ')' => self.toks.push(Token::CBrkt),
            _ => {
                if self.ch.is_ascii_digit() {
                    self.toks
                        .push(Token::Number(self.ch.to_digit(10).unwrap() as f64))
                } else {
                    self.ch_into()
                }
            }
        }
    }
    pub fn lex(&mut self) {
        self.toks = Vec::<Token>::new();
        for _ in 0..self.input.len() {
            if self.idx > self.input.len() {
                break;
            }
            self.ch_into();
        }
    }
}
