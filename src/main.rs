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
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const FONTSET: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

impl Computer{


    fn next_operation(&mut self){
        self.cpu.program_counter += 2;
    }
    fn executor(&mut self){
        let index = self.cpu.program_counter as usize;

        let first_part : u8 = self.memory.buf[index]  >> 4;
        let second_part : u8  = (self.memory.buf[index] << 4) >> 4;
        let third_part : u8 = self.memory.buf[index+1] >> 4;
        let fourth_part : u8 = (self.memory.buf[index+1] << 4) >> 4;


        let n2 = ((0 | third_part) << 4) | fourth_part;
        let mut n3 =  (0 | second_part as u16) << 8 | (0 | third_part as u16) << 4 | fourth_part as u16;

        let instruction = (0 | first_part as u16) << 12 | (0 | second_part as u16) << 8 | (0 | third_part as u16) << 4 | (0 | fourth_part as u16);

        match first_part {
            0 => {
                if instruction == 0x00E0 {
                    self.display.clear_display();
                }
                if instruction == 0x00EE {
                    self.cpu.program_counter = self.stack.buf.pop().unwrap() ;
                }
            },

            1 =>{
                self.cpu.program_counter = n3 - 2 ;
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
                self.cpu.registers[second_part as usize] = self.cpu.registers[second_part as usize].overflowing_add(n2).0 ;
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
                        self.cpu.registers[second_part as usize] = self.cpu.registers[second_part as usize].overflowing_add(self.cpu.registers[third_part as usize]).0;
                    }
                    5 => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[second_part as usize].overflowing_sub(self.cpu.registers[third_part as usize]).0;
                    },
                    6 => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize];
                        self.cpu.registers[15] = self.cpu.registers[second_part as usize] & 0x01;
                        self.cpu.registers[second_part as usize] = self.cpu.registers[second_part as usize].overflowing_shr(1).0;
                        //self.cpu.registers[15] = least_bit;

                    }
                    7 => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize] - self.cpu.registers[second_part as usize];
                    }
                    0xe => {
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize];
                        self.cpu.registers[15] = self.cpu.registers[second_part as usize] & 0x80;
                        self.cpu.registers[second_part as usize] = self.cpu.registers[second_part as usize].overflowing_shl(1).0;
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
                self.cpu.address_reg =  n3;
            },
            11 => {
                self.cpu.program_counter = self.cpu.registers[0] as u16 + n3 ;
            },
            12 => (),
            13 =>  unsafe {
                    self.display.draw(self.cpu.registers[second_part as usize] ,self.cpu.registers[third_part as usize] ,fourth_part,&self.memory.buf,self.cpu.address_reg);
            },
            14 => (),
            15 => {
                if n2 == 0x29 {
                    match second_part {
                        0 => self.cpu.address_reg = 0,
                        1 => self.cpu.address_reg = 5,
                        2 => self.cpu.address_reg = 10,
                        3 => self.cpu.address_reg = 15,
                        4 => self.cpu.address_reg = 20,
                        5 => self.cpu.address_reg = 25,
                        6 => self.cpu.address_reg = 30,
                        7 => self.cpu.address_reg = 35,
                        8 => self.cpu.address_reg = 40,
                        9 => self.cpu.address_reg = 45,
                        10 => self.cpu.address_reg = 50,
                        11 => self.cpu.address_reg = 55,
                        12 => self.cpu.address_reg = 60,
                        13 => self.cpu.address_reg = 65,
                        14 => self.cpu.address_reg = 70,
                        15 => self.cpu.address_reg = 75,
                        _ => (),
                    }
                }
                if n2 == 0x55 {
                    for i in 0..second_part+1{
                        self.memory.buf[self.cpu.address_reg as usize + i as usize] = self.cpu.registers[i as usize];
                    }
                }
                if n2 == 0x65 {
                    for i in 0..second_part+1{
                        self.cpu.registers[i as usize] = self.memory.buf[self.cpu.address_reg as usize + i as usize];
                    }
                }

            },
            _ => println!("error"),
        }
    }
}



fn main() {
    let mut f = File::open("other_test.ch8").unwrap();

    let mut s = Vec::new();

    _ = f.read_to_end(&mut s).unwrap();

    let len: &usize = &s.len();


    let mut cpu = CPU{
        registers : [0;16],
        address_reg: 0,
        program_counter:512,
    };

    let mut memory = Memory{
        buf : [0;4096]
    };

    let mut stack = Stack{
        buf : Vec::new()
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 64*10, 32*10)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();


    let mut display = Display{
        canvas,
        screen:[0;2048]
    };


    let mut computer =  Computer{
        cpu,
        memory,
        stack,
        display
    };

    let mut t: usize = 0;
    while t < *len {
        computer.memory.buf[512 + t] = *s.get(t).unwrap();
        t += 1;
    }

    let mut a : usize = 0;
    while a < FONTSET.len() {
        computer.memory.buf[a] = *FONTSET.get(a).unwrap();
        a+=1;
    }



    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;


        'running: loop {
            i = (i + 1) % 255;
            computer.display.canvas.set_draw_color(Color::WHITE);
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...



            if &computer.cpu.program_counter <= &((4096u16) - 2) {
                computer.executor();
                computer.next_operation();
            }


            computer.display.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        }








}



