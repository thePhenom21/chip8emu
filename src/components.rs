
#[derive(Clone)]
pub struct CPU {
    pub registers: Vec<u8>,
    pub address_reg: u16,
    pub program_counter: u8
}

impl CPU{}


#[derive(Clone)]
pub struct Memory{
    pub buf : Vec<u8>
}

impl Memory{}


#[derive(Clone)]
pub struct Stack{
    pub buf : Vec<u8>
}

impl Stack{}