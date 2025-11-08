#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    OBrkt,
    CBrkt,
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub input: String,
    pub toks: Vec<Token>,
}

impl Lexer {
    pub fn from_string(input: String) -> Self {
        Self {
            input,
            toks: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        let chars: Vec<char> = self.input.chars().collect();
        let mut idx = 0;

        while idx < chars.len() {
            let ch = chars[idx];
            match ch {
                ' ' | '\t' | '\n' => {
                    idx += 1; // skip whitespace
                }
                '+' => {
                    self.toks.push(Token::Plus);
                    idx += 1;
                }
                '-' => {
                    self.toks.push(Token::Minus);
                    idx += 1;
                }
                '*' => {
                    self.toks.push(Token::Mul);
                    idx += 1;
                }
                '/' => {
                    self.toks.push(Token::Div);
                    idx += 1;
                }
                '(' => {
                    self.toks.push(Token::OBrkt);
                    idx += 1;
                }
                ')' => {
                    self.toks.push(Token::CBrkt);
                    idx += 1;
                }
                '0'..='9' => {
                    let mut buf = String::new();
                    while idx < chars.len() && (chars[idx].is_ascii_digit() || chars[idx] == '.') {
                        buf.push(chars[idx]);
                        idx += 1;
                    }
                    let num = buf.parse::<f64>().unwrap();
                    self.toks.push(Token::Number(num));
                }
                _ => panic!("Invalid character '{}' at position {}", ch, idx),
            }
        }
    }
}
