/// Abstract syntax

use std::fmt;
use rug::Integer;

#[derive(Debug,PartialEq)]
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
            Builtin::AddInteger =>
                write!(f, "addInteger"),
            Builtin::SubtractInteger =>
                write!(f, "subtractInteger"),
            Builtin::MultiplyInteger =>
                write!(f, "multiplyInteger"),
            Builtin::DivideInteger =>
                write!(f, "divideInteger"),
            Builtin::QuotientInteger =>
                write!(f, "quotientInteger"),
            Builtin::RemainderInteger =>
                write!(f, "remainderInteger"),
            Builtin::ModInteger =>
                write!(f, "modInteger"),
            Builtin::LessThanInteger =>
                write!(f, "lessThanInteger"),
            Builtin::LessThanEqInteger =>
                write!(f, "lessThanEqualsInteger"),
            Builtin::GreaterThanInteger =>
                write!(f, "greaterThanInteger"),
            Builtin::GreaterThanEqInteger =>
                write!(f, "greaterThanEqualsInteger"),
            Builtin::EqInteger =>
                write!(f, "equalsInteger"),
            Builtin::Concatenate =>
                write!(f, "concatenate"),
            Builtin::TakeByteString =>
                write!(f, "takeByteString"),
            Builtin::DropByteString =>
                write!(f, "dropByteString"),
            Builtin::SHA2 =>
                write!(f, "sha2_256"),
            Builtin::SHA3 =>
                write!(f, "sha3_256"),
            Builtin::VerifySignature =>
                write!(f, "verifySignature"),
            Builtin::EqByteString =>
                write!(f, "equalsByteString"),
            Builtin::LtByteString =>
                write!(f, "lessThanByteString"),
            Builtin::GtByteString =>
                write!(f, "greaterThanByteString"),
            Builtin::IfThenElse =>
                write!(f, "ifThenElse"),
            Builtin::CharToString =>
                write!(f, "charToString"),
            Builtin::Append =>
                write!(f, "append"),
            Builtin::Trace =>
                write!(f, "trace"),
        }
    }
}

#[derive(Debug,PartialEq)]
enum Const {
    IntConst(Integer),
    ByteStringConst(Vec<u8>),
    StringConst(String),
    CharConst(char),
    UnitConst,
    BoolConst(bool),
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Const::IntConst(n) =>
                write!(f, "(con integer {})", n),
            Const::ByteStringConst(s) =>
                write!(f, "(con bytestring {:?})", s),
            Const::StringConst(s) =>
                write!(f, "(con string \"{}\")", s),
            Const::CharConst(c) =>
                write!(f, "(con char '{}')", c),
            Const::UnitConst =>
                write!(f, "(con unit ())"),
            Const::BoolConst(b) =>
                write!(f, "(con boolean {})", b)
        }
    }
}

#[derive(Debug,PartialEq)]
struct Name {
    id: String,
    uniq: u32,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.id, self.uniq)
    }
}

#[derive(Debug,PartialEq)]
enum Term<N> {
    Constant(Const),
    Builtin(Builtin),
    Var(N),
    LamAbs(N,Box<Term<N>>),
    Apply(Box<Term<N>>, Box<Term<N>>),
    Delay(Box<Term<N>>),
    Force(Box<Term<N>>),
    Error
}

impl<N: fmt::Display> fmt::Display for Term<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(c) =>
                write!(f, "{}", c),
            Term::Builtin(b) =>
                write!(f, "(builtin {})", b),
            Term::Var(v) =>
                write!(f, "{}", v),
            Term::LamAbs(name, body) => 
                write!(f, "(lam {}\n {}\n)", name, body),
            Term::Apply(t1, t2) =>
                write!(f, "[\n{}\n{}\n]", t1, t2),
            Term::Delay(t) =>
                write!(f, "(delay {})", t),
            Term::Force(t) =>
                write!(f, "(force {})", t),
            Term::Error => 
                write!(f, "(error)"),
        }
    }
}

#[derive(Debug,PartialEq)]
struct Program<N>(i32,i32,i32,Term<N>);

impl<N: fmt::Display> fmt::Display for Program<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program(v1,v2,v3,body) =>
                write!(f, "(program {}.{}.{}\n{}\n)\n", v1, v2, v3, body),
        }
    }
}