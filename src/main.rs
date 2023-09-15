mod components;
mod display;

use components::{Memory,CPU,Stack,Computer};
use display::Display;

use std::{
    fmt::LowerHex,
    fs::File,
    io::{Read},
    u8,
};


impl Computer{

    fn next_operation(&mut self){
        self.cpu.program_counter += 2;
    }
    fn executor(&mut self){
        let index = self.cpu.program_counter as usize;

        let first_part : u8 = self.memory.buf[index + 1]  >> 4;
        let second_part : u8  = (self.memory.buf[index + 1] << 4) >> 4;
        let third_part : u8 = self.memory.buf[index] >> 4;
        let fourth_part : u8 = (self.memory.buf[index] << 4) >> 4;


        let n2 = ((0 | third_part) << 4) | fourth_part;
        let n3 =  (0 | second_part as u16) << 8 | (0 | third_part as u16) << 4 | fourth_part as u16;

        let instruction = (0 | first_part as u16) << 12 | (0 | second_part as u16) << 8 | (0 | third_part as u16) << 4 | (0 | fourth_part as u16);

        match first_part {
            0 => {
                if instruction == 0x00E0 {
                    self.display.clear_display();
                }
                if instruction == 0x00EE {
                    self.cpu.program_counter = self.stack.buf.pop().unwrap() - 2;
                }
            },

            1 =>{
                self.cpu.program_counter = n3 - 2;
            },

            2 => {
                self.stack.buf.push(self.cpu.program_counter);
                self.cpu.program_counter = n3 - 2;
            },
            3 => {
                if (self.cpu.registers[second_part as usize] == n2){
                    self.cpu.program_counter += 2;
                }
            },
            4 => {
                if (self.cpu.registers[second_part as usize] != n2){
                    self.cpu.program_counter += 2;
                }
            },
            5 => {
                if(self.cpu.registers[second_part as usize] == self.cpu.registers[third_part as usize]){
                    self.cpu.program_counter += 2;
                }
            },
            6 => {
                self.cpu.registers[second_part as usize] = n2;
            },
            7 => {
                self.cpu.registers[second_part as usize] += n2;
            },
            8 => {
                match fourth_part {
                    0 => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize];
                    },
                    1 => {
                        self.cpu.registers[second_part as usize] |= self.cpu.registers[third_part as usize];
                    }
                    2 => {
                        self.cpu.registers[second_part as usize] &= self.cpu.registers[third_part as usize];
                    },
                    3 => {
                        self.cpu.registers[second_part as usize] ^= self.cpu.registers[third_part as usize];
                    }
                    4 => {
                        self.cpu.registers[second_part as usize] += self.cpu.registers[third_part as usize];
                    }
                    5 => {
                        self.cpu.registers[second_part as usize] -= self.cpu.registers[third_part as usize];
                    },
                    6 => {
                        let least_bit = (self.cpu.registers[second_part as usize] << 7) >> 7;
                        self.cpu.registers[15] = least_bit;
                        self.cpu.registers[second_part as usize] >>= 1;
                    }
                    7 => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize] - self.cpu.registers[second_part as usize];
                    }
                    8 => {
                        let most_bit = (self.cpu.registers[second_part as usize] >> 7) ;
                        self.cpu.registers[15] = most_bit;
                        self.cpu.registers[second_part as usize] >>= 1;
                    }
                    _ => {}
                }
            },
            9 => {
                if (self.cpu.registers[second_part as usize] != self.cpu.registers[third_part as usize]){
                    self.cpu.program_counter += 2;
                }
            },
            10 => {
                self.cpu.address_reg = n3;
            },
            11 => {
                self.cpu.program_counter = self.cpu.registers[0] as u16 + n3 - 2;
            },
            12 => (),
            13 => {

                let p = fourth_part as usize;
                let mut buf: [u8;8] = [0;8];
                for k  in buf{
                    let a = k as usize;
                    buf[a] = self.memory.buf[(self.cpu.address_reg+a as u16) as usize];
                }

                    self.display.draw(self.cpu.registers[second_part as usize],self.cpu.registers[third_part as usize],fourth_part,buf);


            },
            14 => (),
            15 => (),
            _ => println!("error"),
        }
    }
}



fn main() {
    let mut f = File::open("test_opcode.ch8").unwrap();

    let mut s = Vec::new();

    _ = f.read_to_end(&mut s).unwrap();

    let len: &usize = &s.len();


    let mut cpu = CPU{
        registers : [0;16],
        address_reg: 0,
        program_counter:0,
    };

    let mut memory = Memory{
        buf : [0;4096]
    };

    let mut stack = Stack{
        buf : Vec::new()
    };

    let mut display = Display::default();



    let mut computer =  Computer{
        cpu,
        memory,
        stack,
        display
    };

    let mut i : usize = 0;
    while i < *len {
        computer.memory.buf[i] = *s.get(i).unwrap();
        i+=1;
    }

    loop {
        while &computer.cpu.program_counter <= &((4096u16) - 2) {
            computer.executor();
            computer.next_operation();
        }
    }



}



