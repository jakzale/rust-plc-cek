use rug::Integer;
use std::fmt;

/// Abstract syntax
#[derive(Debug, PartialEq)]
enum Builtin {
    AddInteger,
    SubtractInteger,
    MultiplyInteger,
    DivideInteger,
    QuotientInteger,
    RemainderInteger,
    ModInteger,
    LessThanInteger,
    LessThanEqInteger,
    GreaterThanInteger,
    GreaterThanEqInteger,
    EqInteger,
    Concatenate,
    TakeByteString,
    DropByteString,
    SHA2,
    SHA3,
    VerifySignature,
    EqByteString,
    LtByteString,
    GtByteString,
    IfThenElse,
    CharToString,
    Append,
    Trace,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Builtin::AddInteger => write!(f, "addInteger"),
            Builtin::SubtractInteger => write!(f, "subtractInteger"),
            Builtin::MultiplyInteger => write!(f, "multiplyInteger"),
            Builtin::DivideInteger => write!(f, "divideInteger"),
            Builtin::QuotientInteger => write!(f, "quotientInteger"),
            Builtin::RemainderInteger => write!(f, "remainderInteger"),
            Builtin::ModInteger => write!(f, "modInteger"),
            Builtin::LessThanInteger => write!(f, "lessThanInteger"),
            Builtin::LessThanEqInteger => write!(f, "lessThanEqualsInteger"),
            Builtin::GreaterThanInteger => write!(f, "greaterThanInteger"),
            Builtin::GreaterThanEqInteger => write!(f, "greaterThanEqualsInteger"),
            Builtin::EqInteger => write!(f, "equalsInteger"),
            Builtin::Concatenate => write!(f, "concatenate"),
            Builtin::TakeByteString => write!(f, "takeByteString"),
            Builtin::DropByteString => write!(f, "dropByteString"),
            Builtin::SHA2 => write!(f, "sha2_256"),
            Builtin::SHA3 => write!(f, "sha3_256"),
            Builtin::VerifySignature => write!(f, "verifySignature"),
            Builtin::EqByteString => write!(f, "equalsByteString"),
            Builtin::LtByteString => write!(f, "lessThanByteString"),
            Builtin::GtByteString => write!(f, "greaterThanByteString"),
            Builtin::IfThenElse => write!(f, "ifThenElse"),
            Builtin::CharToString => write!(f, "charToString"),
            Builtin::Append => write!(f, "append"),
            Builtin::Trace => write!(f, "trace"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Const {
    IntConst(Integer),
    ByteStringConst(Vec<u8>),
    StringConst(String),
    CharConst(char),
    UnitConst,
    BoolConst(bool),
}

// Smart Constructors for const
impl Const {
    pub fn integer<T: Into<Integer>>(x: T) -> Self {
        Self::IntConst(x.into())
    }

    pub fn bytestring<T: Into<Vec<u8>>>(x: T) -> Self {
        Self::ByteStringConst(x.into())
    }

    pub fn string<T: Into<String>>(x: T) -> Self {
        Self::StringConst(x.into())
    }

    pub fn char<T: Into<char>>(x: T) -> Self {
        Self::CharConst(x.into())
    }

    pub fn unit() -> Self {
        Self::UnitConst
    }

    pub fn bool<T: Into<bool>>(x: T) -> Self {
        Self::BoolConst(x.into())
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Const::IntConst(n) => write!(f, "(con integer {})", n),
            Const::ByteStringConst(s) => write!(f, "(con bytestring {:?})", s),
            Const::StringConst(s) => write!(f, "(con string \"{}\")", s),
            Const::CharConst(c) => write!(f, "(con char '{}')", c),
            Const::UnitConst => write!(f, "(con unit ())"),
            Const::BoolConst(b) => write!(f, "(con boolean {})", b),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Name {
    id: String,
    uniq: u32,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.id, self.uniq)
    }
}

#[derive(Debug, PartialEq)]
enum Term<N> {
    Constant(Const),
    Builtin(Builtin),
    Var(N),
    LamAbs(N, Box<Term<N>>),
    Apply(Box<Term<N>>, Box<Term<N>>),
    Delay(Box<Term<N>>),
    Force(Box<Term<N>>),
    Error,
}

impl<N: fmt::Display> fmt::Display for Term<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(c) => write!(f, "{}", c),
            Term::Builtin(b) => write!(f, "(builtin {})", b),
            Term::Var(v) => write!(f, "{}", v),
            Term::LamAbs(name, body) => write!(f, "(lam {}\n {}\n)", name, body),
            Term::Apply(t1, t2) => write!(f, "[\n{}\n{}\n]", t1, t2),
            Term::Delay(t) => write!(f, "(delay {})", t),
            Term::Force(t) => write!(f, "(force {})", t),
            Term::Error => write!(f, "(error)"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Program<N>(i32, i32, i32, Term<N>);

impl<N: fmt::Display> fmt::Display for Program<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program(v1, v2, v3, body) => write!(f, "(program {}.{}.{}\n{}\n)\n", v1, v2, v3, body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_spec() {
        // TODO:  Finish
        assert_eq!("addInteger", format!("{}", Builtin::AddInteger))
    }

    #[test]
    fn const_spec() {
        assert_eq!("(con integer 1)", format!("{}", Const::integer(1)));
        assert_eq!(
            "(con bytestring [])",
            format!("{}", Const::bytestring(vec![]))
        );
        assert_eq!(
            "(con bytestring [0, 1, 2])",
            format!("{}", Const::bytestring(vec![0, 1, 2]))
        );
        assert_eq!(
            "(con bytestring [0, 1, 2])",
            format!("{}", Const::bytestring(vec![0, 1, 2]))
        );
        assert_eq!(
            "(con string \"hello\")",
            format!("{}", Const::string("hello"))
        );
        assert_eq!("(con unit ())", format!("{}", Const::unit()));
        assert_eq!("(con boolean true)", format!("{}", Const::bool(true)));
    }
}
