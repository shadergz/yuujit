use std::{fmt::{self, Debug}, marker::PhantomData};

pub trait Type: Clone + Copy + Debug {
    fn bits() -> u32;
}

#[derive(Clone, Copy, Debug)]
pub struct U1;

impl Type for U1 {
    fn bits() -> u32 {
        1
    }
}

#[derive(Clone, Copy, Debug)]
pub struct U8;

impl Type for U8 {
    fn bits() -> u32 {
        8
    }
}

#[derive(Clone, Copy, Debug)]
pub struct U16;

impl Type for U16 {
    fn bits() -> u32 {
        16
    }
}

#[derive(Clone, Copy, Debug)]
pub struct U32;

impl Type for U32 {
    fn bits() -> u32 {
        32
    }
}

#[derive(Clone, Copy)]
pub enum Value<T: Type> {
    Imm(u32, PhantomData<T>),
    Var(u32, PhantomData<T>),
}

impl<T: Type> Value<T> {
    pub fn from_imm(value: u32) -> Self {
        assert!((value as u64) < ((1 as u64) << T::bits()));
        Self::Imm(value, PhantomData)
    }

    pub fn from_var(id: u32) -> Self {
        Self::Var(id, PhantomData)
    }
}

impl<T: Type> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Imm(imm, _) => write!(f, "{:#x}", imm),
            Self::Var(id, _) => write!(f, "v{id}"),
        }
    }
}