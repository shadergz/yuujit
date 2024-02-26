use std::{fmt::Debug, marker::PhantomData};

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
    Var(usize, PhantomData<T>),
}

impl<T: Type> From<u32> for Value<T> {
    fn from(value: u32) -> Self {
        assert!(value < T::bits() * 2);
        Self::Imm(value, PhantomData)
    }
}