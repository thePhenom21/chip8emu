mod components;
use components::{Memory,CPU,Stack};

use std::{
    fmt::LowerHex,
    fs::File,
    io::{Bytes, Read},
    u8,
};
use std::slice::Windows;

use wgpu::Instance;

fn main() {
    let mut f = File::open("ibm.ch8").unwrap();

    let mut s = Vec::new();

    _ = f.read_to_end(&mut s).unwrap();

    let len : &usize = &s.len();

    let disp = Instance::new(Default::default());
    unsafe { disp.create_surface(); }

    let mut cpu = CPU{
        registers : [0;16],
        address_reg: 0,
        program_counter: 0,
    };

    let mut memory = Memory{
        buf : [0;4096]
    };

    let mut stack = Stack{
        buf : [0;48]
    };

    while &cpu.program_counter < &(*len as u8) {
        let re = dis(&s, cpu, memory , stack);
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

    let mut second_part_s = second_part.to_string();
    let mut third_part_s = third_part.to_string();
    let fourth_part_s = fourth_part.to_string();
    let mut third_part2_s = third_part.to_string();

    third_part_s.push_str(fourth_part_s.as_str());

    second_part_s.push_str(third_part2_s.as_str());
    second_part_s.push_str(fourth_part_s.as_str());

    let n2 = &third_part_s.parse::<u8>().unwrap();
    let n3 = &second_part_s.parse::<u16>().unwrap();



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
                cpu.registers[second_part as usize] = *n2;
        },
        0x07 => {
                cpu.registers[second_part as usize] += n2;
        },
        0x08 => (),
        0x09 => (),
        0x0a => {
                cpu.address_reg = *n3;
        },
        0x0b => (),
        0x0c => (),
        0x0d => {
            draw(cpu.registers[second_part as usize],cpu.registers[third_part as usize],fourth_part);
        },
        0x0e => (),
        0x0f => (),
        _ => println!("error"),
    }

    return (cpu,memory, stack);
}

fn clear_display(){

}

fn draw(x : u8, y : u8, n : u8){

}
