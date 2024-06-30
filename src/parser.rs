use crate::token::Token;

pub enum BinaryOp {
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Slash,
    Modul,
    Star,
    Caret,
}

pub enum UnaryOp {
    MinusMinus,
    PlusPlus,
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        left: Box<Expr>,
        op: UnaryOp,
    },
    Ident {
        name: String,
    },
    String {
        value: String,
    },
}

pub struct Parser<'a> {
    pub stream: &'a Vec<Token>,
    pub output: Vec<Expr>,
    token: &'a Token,
}

impl<'a> Parser<'a> {
    pub fn new(stream: &'a Vec<Token>) -> Self {
        Self {
            stream,
            output: Vec::<Expr>::new(),
            token: &stream[0],
        }
    }

    pub fn parse(&mut self) {}

    fn push_expr(self: &mut Self, expr: Expr) {
        self.output.push(expr);
    }

    fn parse_expr(&mut self) -> Expr {
        match self.token {
            Token::Ident { offset: _, value } => Expr::Ident {
                name: value.to_owned(),
            },
            Token::String { offset: _, value } => Expr::String {
                value: value.to_owned(),
            },
            _ => panic!("[DY4] Unexpected token"),
        }
    }
}
