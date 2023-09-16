use crate::display::Display;
pub struct Computer{
    pub cpu : CPU,
    pub memory : Memory,
    pub stack : Stack,
    pub display : Display
}

pub struct CPU {
    pub registers: [u8;16],
    pub address_reg: u16,
    pub program_counter: u16
}


pub struct Memory{
    pub buf : [u8;4096]
}

pub struct Stack{
    pub buf : Vec<u16>
}

