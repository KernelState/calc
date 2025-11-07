use crate::lexer::Token;

#[derive(Clone, Eq, PartialEq)]
pub enum Oper {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Clone)]
pub enum Number {
    Int(u32, bool),
    Float(f64, bool),
}

impl Number {
    pub fn true_number(&self) -> f64 {
        match self {
            Number::Int(i, positive) => (*i as f64) * if *positive { 1.0 } else { -1.0 },
            Number::Float(f, positive) => *f * if *positive { 1.0 } else { -1.0 },
        }
    }
}

#[derive(Clone)]
pub struct Brkt {
    pub items: Vec<EqItem>,
}

#[derive(Clone, Default)]
pub enum EqItem {
    #[default]
    None,
    Number(Number),
    Brkt(Brkt),
}

#[derive(Default)]
pub struct EqPart {
    before: Option<Oper>,
    item: EqItem,
    after: Option<Oper>,
}
pub struct Eqt {
    parts: Vec<EqPart>,
    value: Option<f64>,
}

impl EqItem {
    pub fn true_number(&self) -> f64 {
        match self {
            EqItem::Number(n) => n.true_number(),
            _ => 0.0,
        }
    }
    pub fn numberfy(&mut self) {
        match self {
            EqItem::Brkt(b) => {
                let mut new = EqItem::Number(Number::Int(0, true));
                let mut nw = 0.0;
                for num in b.items.iter_mut() {
                    num.numberfy();
                    nw += if let EqItem::Number(nm) = num {
                        nm.true_number()
                    } else if let EqItem::Brkt(bk) = num {
                        num.clone().numberfy();
                        num.true_number()
                    } else {
                        panic!("Unreachable"); // I'm stoopid and I'm too lazy to refactor properly
                        0.0
                    };
                }
                new = EqItem::Number(if nw % 1.0 == 0.0 {
                    Number::Int(nw as u32, true)
                } else {
                    Number::Float(nw, true)
                });
                *self = new;
            }
            EqItem::Number(n) => *self = EqItem::Number(n.clone()),
            _ => panic!("Unreachable"), // perfecto, don't judge
        }
    }
}

impl EqPart {
    pub fn new(toks: Vec<Token>) -> Self {
        if (toks.len() > 3 || toks.len() == 0) {
            panic!("Invalid tokenized equation part");
            return Self::default();
        } else {
            let before = match toks.get(0) {
                Some(Token::Plus) => Some(Oper::Plus),
                Some(Token::Minus) => Some(Oper::Minus),
                _ => None,
            };
            let after = match toks.get(toks.len() - 1) {
                Some(Token::Plus) => Some(Oper::Plus),
                Some(Token::Minus) => Some(Oper::Minus),
                Some(Token::Mul) => Some(Oper::Mul),
                Some(Token::Div) => Some(Oper::Div),
                _ => None,
            };
            let item_tok = if before.is_some() && after.is_some() {
                &toks[1]
            } else if before.is_some() || after.is_some() {
                &toks[1.min(toks.len() - 1)]
            } else {
                &toks[0]
            };
            let item = match item_tok {
                Token::Number(n) => EqItem::Number(if n % 1.0 == 0.0 {
                    Number::Int(*n as u32, *n >= 0.0)
                } else {
                    Number::Float(*n, *n >= 0.0)
                }),
                _ => EqItem::default(),
            };
            Self {
                before,
                item,
                after,
            }
        }
    }
    pub fn truefy(&mut self) {
        self.item.numberfy();
        let positive = match &self.before {
            Some(Oper::Minus) => {
                self.before = None;
                false
            }
            _ => true,
        };
        let num = self.item.true_number();
        self.item = EqItem::Number(if num % 1.0 == 0.0 {
            Number::Int(num.abs() as u32, positive)
        } else {
            Number::Float(num.abs(), positive)
        });
    }
    pub fn clone(&self) -> Self {
        Self {
            before: match &self.before {
                Some(v) => Some(v.clone()),
                None => None,
            },
            item: self.item.clone(),
            after: match &self.after {
                Some(v) => Some(v.clone()),
                None => None,
            },
        }
    }
    pub fn calc(&self, oper: Oper, other: &EqPart) -> Self {
        let mut new_part = self.clone();
        let num1 = self.item.true_number();
        let num2 = other.item.true_number();
        let result = match oper {
            Oper::Mul => num1 * num2,
            Oper::Div => num1 / num2,
            _ => panic!("Unsupported operation in calc"),
        };
        new_part.item = EqItem::Number(if result % 1.0 == 0.0 {
            Number::Int(result as u32, result >= 0.0)
        } else {
            Number::Float(result, result >= 0.0)
        });
        new_part.after = other.after.clone();
        new_part
    }
}

impl Eqt {
    pub fn new(parts: Vec<EqPart>) -> Self {
        Self { parts, value: None }
    }

    pub fn pre_steps(&mut self) {
        let mut new_parts: Vec<EqPart> = Vec::new();
        for i in 0..self.parts.len() {
            let part = &self.parts[i];
            if let Some(v) = &part.after {
                if !(*v == Oper::Mul || *v == Oper::Div) {
                    new_parts.push(part.clone());
                    continue;
                } else {
                }
                if i + 1 > self.parts.len() - 1 {
                    panic!("Cannot do an operation without a following part");
                }
                new_parts.push(part.clone().calc(v.clone(), &self.parts[i + 1].clone()));
            } else {
                new_parts.push(part.clone());
            }
        }
        self.parts = new_parts;
    }

    pub fn eval(&mut self) -> f64 {
        self.pre_steps();
        let mut result = 0.0;
        let mut last: EqPart = EqPart::default();
        let mut first = true;
        for part in self.parts.iter_mut() {
            part.item.numberfy();
            part.truefy();
            let num = part.item.true_number();
            result += num;
            if first {
                first = false;
                last = part.clone();
                continue;
            }
            match part.after {
                Some(Oper::Plus) => result += num,
                Some(Oper::Minus) => result -= num,
                _ => panic!("Unsupported operation in evaluation"),
            }
        }
        self.value = Some(result);
        result
    }
}
