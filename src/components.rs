use std::ops::Deref;

#[derive(Clone)]
pub struct CPU {
    pub registers: [u8;16],
    pub address_reg: u16,
    pub program_counter: u8
}


#[derive(Clone)]
pub struct Memory{
    pub buf : [u8;4096]
}

#[derive(Clone)]
pub struct Stack{
    pub buf : [u8;48]
}

impl Deref for CPU{
    type Target = CPU;

    fn deref(&self) -> &Self::Target {
        &self
    }
}
impl Deref for Memory{
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self
    }
}
impl Deref for Stack{
    type Target = Stack;

    fn deref(&self) -> &Self::Target {
        &self
    }
}