use crate::lexer::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidToken(usize),
    UnclosedBracket(usize),
    UnexpectedEndOfInput(usize),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidToken(pos) => write!(f, "Invalid token at position {}", pos),
            ParseError::UnclosedBracket(pos) => write!(f, "Unclosed bracket at position {}", pos),
            ParseError::UnexpectedEndOfInput(pos) => {
                write!(f, "Expected Token after {} but found end of input", pos)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseNode {
    Number(f64),
    Operator(char),
    Bracket(Vec<ParseNode>),
}

impl ParseNode {
    pub fn from_token(tok: &Token, pos: &usize) -> Result<Self, ParseError> {
        match tok {
            Token::Number(n) => Ok(ParseNode::Number(*n)),
            Token::Plus => Ok(ParseNode::Operator('+')),
            Token::Minus => Ok(ParseNode::Operator('-')),
            Token::Mul => Ok(ParseNode::Operator('*')),
            Token::Div => Ok(ParseNode::Operator('/')),
            _ => Err(ParseError::InvalidToken(*pos)),
        }
    }
}

pub struct Parser {
    pub toks: Vec<Token>,
    pub pos: usize,
    pub result: Option<f64>,
    complete: bool,
    buf: Vec<ParseNode>,
}

impl Parser {
    pub fn from_toks(toks: Vec<Token>) -> Self {
        println!("[PARSER_DEBUG] Original tokens: {:?}", toks);
        Parser {
            toks,
            pos: 0,
            result: None,
            buf: Vec::new(),
            complete: false,
        }
    }

    pub fn from_nodes(nodes: Vec<ParseNode>) -> Self {
        let toks = nodes
            .iter()
            .map(|node| match node {
                ParseNode::Number(n) => Token::Number(*n),
                ParseNode::Operator(op) => match op {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Mul,
                    '/' => Token::Div,
                    _ => panic!("Invalid operator"),
                },
                ParseNode::Bracket(_) => panic!("Cannot convert Bracket to Token directly"),
            })
            .collect();
        Parser::from_toks(toks)
    }

    pub fn eval(&mut self) -> Result<f64, ParseError> {
        self.result = Some(self.parse_expression()?);
        Ok(self.result.unwrap())
    }

    fn parse_expression(&mut self) -> Result<f64, ParseError> {
        println!(
            "[PARSER_DEBUG] Starting parse_expression with {} tokens",
            self.toks.len()
        );
        self.buf.clear();

        while self.pos < self.toks.len() {
            println!(
                "[PARSER_DEBUG] Parsing token at pos {}: {:?}",
                self.pos, self.toks[self.pos]
            );
            self.next()?;
        }

        println!("[PARSER_DEBUG] Initial buffer: {:?}", self.buf);

        // Unwrap brackets
        self.buf
            .iter_mut()
            .try_for_each(|node| -> Result<(), ParseError> {
                if let ParseNode::Bracket(contents) = node {
                    println!("[PARSER_DEBUG] Evaluating bracket at node: {:?}", contents);
                    let mut sub_parser = Parser::from_nodes(contents.clone());
                    let val = sub_parser.eval()?;
                    println!("[PARSER_DEBUG] Bracket value: {}", val);
                    *node = ParseNode::Number(val);
                }
                Ok(())
            })?;

        println!("[PARSER_DEBUG] Buffer after brackets: {:?}", self.buf);

        for i in 0..self.buf.len() {
            println!("[PARSER_DEBUG] combining separated numbers");
            if i + 1 >= self.buf.len() {
                continue;
            }
            let nc1 = match self.buf.get(i) {
                Some(v) => v,
                None => {
                    println!("bad luck with idx {}", i);
                    continue;
                }
            };
            let nc2 = match self.buf.get(i + 1) {
                Some(v) => v,
                None => {
                    println!("bad luck with idx {} (added 1)", i + 1);
                    continue;
                }
            };
            if let ParseNode::Number(n1) = nc1 {
                if let ParseNode::Number(n2) = nc2 {
                    println!("combining {:?} and {:?}", n1, n2);
                    self.buf[i] = ParseNode::Number(n2*n1);
                    self.buf.remove(i+1);
                }
            }
            println!("[PARSER_DEBUG] numbers combined '{:?}'", self.buf);
        }

        // Multiplication and division
        let mut i = 0;
        while i < self.buf.len() {
            if let ParseNode::Operator(op) = &self.buf[i] {
                if *op == '*' || *op == '/' {
                    if i == 0 || i + 1 >= self.buf.len() {
                        return Err(ParseError::UnexpectedEndOfInput(i));
                    }
                    let left = match self.buf[i - 1] {
                        ParseNode::Number(n) => n,
                        _ => return Err(ParseError::InvalidToken(i - 1)),
                    };
                    let right = match self.buf[i + 1] {
                        ParseNode::Number(n) => n,
                        _ => return Err(ParseError::InvalidToken(i + 1)),
                    };
                    let result = if *op == '*' {
                        left * right
                    } else {
                        left / right
                    };
                    println!("[PARSER_DEBUG] {} {} {} = {}", left, op, right, result);
                    self.buf.splice(i - 1..=i + 1, [ParseNode::Number(result)]);
                    i = i.saturating_sub(1);
                }
            }

            i += 1;
        }

        println!("[PARSER_DEBUG] Buffer after mul/div: {:?}", self.buf);

        // Addition and subtraction
        let mut i = 0;
        while self.buf.len() > 1 {
            while i < self.buf.len() {
                if let ParseNode::Operator(op) = &self.buf[i] {
                    if *op == '+' || *op == '-' {
                        if i == 0 || i + 1 >= self.buf.len() {
                            return Err(ParseError::UnexpectedEndOfInput(i));
                        }
                        let left = match self.buf[i - 1] {
                            ParseNode::Number(n) => n,
                            _ => return Err(ParseError::InvalidToken(i - 1)),
                        };
                        let right = match self.buf[i + 1] {
                            ParseNode::Number(n) => n,
                            _ => return Err(ParseError::InvalidToken(i + 1)),
                        };
                        let result = if *op == '+' {
                            left + right
                        } else {
                            left - right
                        };
                        println!("[PARSER_DEBUG] {} {} {} = {}", left, op, right, result);
                        self.buf.splice(i - 1..=i + 1, [ParseNode::Number(result)]);
                        i = i.saturating_sub(1);
                    }
                }
                i += 1;
            }
            if self.buf.len() <= 1 {
                break;
            }
            i = 0;
        }

        println!("[PARSER_DEBUG] Final buffer: {:?}", self.buf);

        if let Some(ParseNode::Number(n)) = self.buf.get(0) {
            println!("[PARSER_DEBUG] Final result: {}", n);
            return Ok(*n);
        }

        Err(ParseError::InvalidToken(self.pos))
    }

    fn next(&mut self) -> Result<(), ParseError> {
        println!(
            "[PARSER_DEBUG] Current pos: {}, token: {:?}",
            self.pos,
            self.toks.get(self.pos)
        );
        match &self.toks[self.pos] {
            Token::Number(n) => {
                self.buf.push(ParseNode::Number(*n));
                self.pos += 1;
            }
            Token::OBrkt => {
                self.pos += 1; // Skip '('
                let mut contents = Vec::<ParseNode>::new();
                while &self.toks[self.pos] != &Token::CBrkt {
                    if self.pos >= self.toks.len() {
                        // Prevent out-of-bounds
                        return Err(ParseError::UnclosedBracket(self.pos));
                    }
                    contents.push(ParseNode::from_token(&self.toks[self.pos], &self.pos)?);
                    self.pos += 1;
                }
                self.buf.push(ParseNode::Bracket(contents));
            }
            Token::CBrkt => {
                // Should not happen here
                self.pos += 1;
            }
            Token::Plus | Token::Minus | Token::Mul | Token::Div => {
                let op_node = ParseNode::from_token(&self.toks[self.pos], &self.pos)?;
                self.buf.push(op_node);
                self.pos += 1;
            }
        }
        self.complete = true;
        Ok(())
    }
}
