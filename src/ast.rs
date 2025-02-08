use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(lhs, op, rhs) => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Add => "+",
            Opcode::Sub => "-",
        };
        write!(f, "{}", x)
    }
}
