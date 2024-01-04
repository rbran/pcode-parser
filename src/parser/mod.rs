/*
    Credits and Acknowledgments
    
    This Rust program was developed with the assistance and contributions of the following individuals and organizations:
    
    - @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
    - Michael Chesser and Rubens Brandao : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

    Their expertise and support have been invaluable.
*/

#![allow(dead_code)]

pub mod parser_impl;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Copy,
    IntAdd,
    BoolOr,
    Load,
    IntSub,
    FloatEqual,
    Store,
    IntCarry,
    FloatNotEqual,
    Branch,
    IntSCarry,
    FloatLess,
    CBranch,
    IntSBorrow,
    FloatLessEqual,
    BrandInd,
    Int2Comp,
    FloatAdd,
    Call,
    IntNegate,
    FloatSub,
    CallInd,
    IntXor,
    FloatMult,
    UserDefined,
    IntAnd,
    FloatDIV,
    Return,
    IntOr,
    FloatNeg,
    Piece,
    IntLeft,
    FloatAbs,
    Subpiece,
    IntRight,
    FloatSqrt,
    IntEqual,
    IntSright,
    FloatCell,
    IntNotEqual,
    IntMult,
    FloatFloor,
    IntLess,
    IntDiv,
    FloatRound,
    IntSLess,
    IntRem,
    FloatNaN,
    IntLessEqual,
    IntSdiv,
    Int2Float,
    IntSLessEqual,
    IntSRem,
    Float2Float,
    IntZExt,
    BoolNegate,
    Trunc,
    IntSExt,
    BoolXor,
    CPoolRef,
    BoolAnd,
    New,
    CallOther,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Size {
    Byte,
    Half,
    Word,
    Quad,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Varnode {
    pub var: Var,
    pub size: Size,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Var {
    Const(String),
    Unique(Addr),
    Register(Addr),
    Memory(Addr),
}

impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Var::Const(s) => {
                0.hash(state); 
                s.hash(state);
            },
            Var::Unique(addr) => {
                1.hash(state); 
                addr.hash(state);
            },
            Var::Register(addr) => {
                2.hash(state); 
                addr.hash(state);
            },
            Var::Memory(addr) => {
                3.hash(state); 
                addr.hash(state);
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Inst {
    pub opcode: Opcode,
    pub output: Option<Varnode>,
    pub inputs: Vec<Varnode>,
}

pub type Addr = u32;

#[derive(Debug)]
pub struct CodeListing(BTreeMap<Addr, Vec<Inst>>);

impl CodeListing {
    pub fn new() -> Self {
        CodeListing(BTreeMap::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub data: i32,
}

impl Value {
    pub fn from_quad(input: u64) -> Self {
        // Assuming the input is i32,
        Self { data: input as i32 }
    }

    pub fn from_word(input: u32) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_half(input: u16) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_byte(input: u8) -> Self {
        Self { data: input as i32 }
    }
}
