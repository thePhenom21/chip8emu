mod components;
use components::{Memory,CPU,Stack};

use std::{
    fmt::LowerHex,
    fs::File,
    io::{Bytes, Read},
    u8,
};

fn main() {
    let mut f = File::open("test_opcode.ch8").unwrap();

    let mut s = Vec::new();

    _ = f.read_to_end(&mut s).unwrap();

    let len : &usize = &s.len();

    let mut cpu = CPU{
        registers : Vec::with_capacity(16),
        address_reg: 0,
        program_counter: 0,
    };

    let mut memory = Memory{
        buf : Vec::with_capacity(4096)
    };

    let mut stack = Stack{
        buf : Vec::with_capacity(48)
    };

    while &cpu.program_counter < &(*len as u8) {
        let re = dis(&s, cpu.clone(), memory.clone() , stack.clone());
        cpu = re.0;
        memory = re.1;
        stack = re.2;
        cpu.program_counter += 2;
    }
}

fn dis(bytes: &Vec<u8>, mut cpu: CPU, mut memory : Memory, mut stack : Stack) -> (CPU, Memory, Stack){
    let index = cpu.program_counter as usize;

    let first_part : u8 = bytes.get(index + 1).unwrap() >> 4;
    let second_part : u8  = (bytes.get(index + 1).unwrap() << 4) >> 4;
    let third_part : u8 = bytes.get(index).unwrap() >> 4;
    let fourth_part : u8 = (bytes.get(index).unwrap() << 4) >> 4;

    let n2 = concat!(third_part.to_string(),fourth_part.to_string()).parse::<u8>().unwrap();
    let n3 = concat!(second_part.to_string(),third_part.to_string(),fourth_part.to_string()).parse::<u16>().unwrap();


    match first_part {
        0x00 => if third_part == 0x0E {
                clear_display();
        },

        0x01 =>{
                cpu.program_counter = (n3 - 2) as u8;
        }


        0x02 => (),
        0x03 => (),
        0x04 => (),
        0x05 => (),
        0x06 => {
                cpu.registers[second_part] = n2;
        },
        0x07 => {
                cpu.registers[second_part] += n2;
        },
        0x08 => (),
        0x09 => (),
        0x0a => {
                cpu.address_reg = n3;
        },
        0x0b => (),
        0x0c => (),
        0x0d => {
            draw(cpu.registers[second_part],cpu.registers[third_part],fourth_part);
        },
        0x0e => (),
        0x0f => (),
        _ => println!("error"),
    }

    return (*cpu,*memory, *stack);
}

fn clear_display(){

}

fn draw(x : u8, y : u8, n : u8){

}
