use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
}

impl Literal {
    pub fn greater(self, other: Self) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a > b),
            (left, right) => panic!("cannot apply '>'"),
        }
    }
    pub fn greater_or_equal(self, other: Self) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a >= b),
            (left, right) => panic!("Cant apply '>='"),
        }
    }
    pub fn less(self, other: Self) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a < b),
            (_, _) => panic!("Cant apply '<'"),
        }
    }
    pub fn less_or_equal(self, other: Self) -> Self {
        match (self, other) {
            (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a <= b),
            (_, _) => panic!("Cant apply '<='"),
        }
    }
    pub fn is_truthy(self) -> bool {
        match self {
            Literal::None => false,
            Literal::Bool(b) => b,
            Literal::Str(_) | Literal::Number(_) => true,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::None => write!(f, "nil"),
        }
    }
}

impl Neg for Literal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Literal::None => Literal::None,
            Literal::Bool(b) => Literal::Bool(!b),
            Literal::Str(s) => Literal::Str(s),
            Literal::Number(f) => Literal::Number(-f),
        }
    }
}

impl Not for Literal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Literal::None => Literal::Bool(true),
            Literal::Bool(b) => Literal::Bool(!b),
            Literal::Str(_) | Literal::Number(_) => Literal::Bool(false),
        }
    }
}

impl Sub for Literal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Literal::None => panic!("Cant subtract with Nil"),
            Literal::Number(n) => match other {
                Literal::Number(other_n) => Literal::Number(n - other_n),
                Literal::None => panic!("Cant subtract with nil"),
                Literal::Str(_) => panic!("Cant subtract with string"),
                Literal::Bool(_) => panic!("Cant subtract with bool"),
            },
            Literal::Bool(_) => panic!("Cant subtract with bool"),
            Literal::Str(_) => panic!("Cant subtract with string"),
        }
    }
}

impl Add for Literal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Literal::None => panic!("cant add with nil"),
            Literal::Number(n) => match other {
                Literal::Number(other_n) => Literal::Number(n + other_n),
                Literal::None => panic!("Cant add with nil"),
                Literal::Str(_) => panic!("Cant add a string to a number"),
                Literal::Bool(_) => panic!("Cant add with bool"),
            },
            Literal::Str(s) => match other {
                Literal::Number(_) => panic!("Cant add a number to a string"),
                Literal::None => panic!("Cant add with nil"),
                Literal::Str(other_s) => Literal::Str(s + &other_s),
                Literal::Bool(_) => panic!("Cant add with bool"),
            },
            Literal::Bool(_) => panic!("Cant add with bool"),
        }
    }
}

impl Mul for Literal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Literal::None => panic!("Cant multiply with nil"),
            Literal::Number(n) => match other {
                Literal::Number(other_n) => Literal::Number(n * other_n),
                Literal::None => panic!("Cant multiply with nil"),
                Literal::Bool(_) => panic!("Cant multiply with bool"),
                Literal::Str(_) => panic!("Cant multiply with string"),
            },
            Literal::Str(_) => panic!("Cant multiply with string"),
            Literal::Bool(_) => panic!("Cant multiply with bool"),
        }
    }
}

impl Div for Literal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Literal::Bool(_) => panic!("Cant divide with bool"),
            Literal::Str(_) => panic!("Cant divide with string"),
            Literal::None => panic!("Cant divide with nil"),
            Literal::Number(n) => match other {
                Literal::Number(other_n) => Literal::Number(n / other_n),
                Literal::None => panic!("Cant divide with nil"),
                Literal::Str(_) => panic!("Cant divide with string"),
                Literal::Bool(_) => panic!("Cant divide with bool"),
            },
        }
    }
}
