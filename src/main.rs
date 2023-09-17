mod components;
mod display;

use components::{Memory,CPU,Stack,Computer};
use display::Display;

use random::{Default, Source};

use std::{
    fmt::LowerHex,
    fs::File,
    io::{Read},
    u8,
};
use std::time::Duration;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::rand;
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
    fn executor(&mut self,keys : &[bool; 16]){
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
                    self.display.screen = [0;2048];
                    self.display.canvas.present();
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
                        self.cpu.registers[second_part as usize] = self.cpu.registers[third_part as usize].overflowing_sub(self.cpu.registers[second_part as usize]).0 ;
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
            12 => unsafe {
                let mut u : u8 = Default::new([2,1]).iter().next().unwrap();
                u = u & n2;
                self.cpu.registers[second_part as usize] = u;
            },
            13 =>  unsafe {
                    self.display.draw(self.cpu.registers[second_part as usize] ,self.cpu.registers[third_part as usize] ,fourth_part,&self.memory.buf,self.cpu.address_reg,self.cpu.registers);
            },
            14 => {
                if n2 == 0x9e{
                    if keys[self.cpu.registers[second_part as usize] as usize] == true {
                        self.cpu.program_counter += 2;
                    }
                }
                if n2 == 0xa1{
                    if keys[self.cpu.registers[second_part as usize] as usize] == false {
                        self.cpu.program_counter += 2;
                    }
                }
            },
            15 => {
                if n2 == 0x07{
                    self.cpu.registers[second_part as usize] = self.cpu.delay_timer
                }
                if n2 == 0x15{
                    self.cpu.delay_timer = self.cpu.registers[second_part as usize];
                }
                if n2 == 0x18{
                    self.cpu.sound_timer = self.cpu.registers[second_part as usize];
                }
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
                if n2 == 0x33 {
                    self.memory.buf[self.cpu.address_reg as usize] = self.cpu.registers[second_part as usize] / 100;
                    self.memory.buf[self.cpu.address_reg as usize + 1 as usize] = (self.cpu.registers[second_part as usize] / 10) % 10 ;
                    self.memory.buf[self.cpu.address_reg as usize + 2 as usize] = (self.cpu.registers[second_part as usize] % 100) % 10 ;
                }
                if n2 == 0x1e {
                    self.cpu.address_reg = self.cpu.address_reg.overflowing_add(self.cpu.registers[second_part as usize] as u16).0;
                }
                if n2 == 0x0a{
                    if !keys.contains(&true) {
                        self.cpu.program_counter -= 2;
                    }else{
                        for i in 0..15{
                            if keys.get(i).unwrap() == &true {
                                self.cpu.registers[second_part as usize] = i as u8;
                            }
                        }
                    }
                }

            },
            _ => println!("error"),
        }
    }
}



#[derive(Clone)]
struct CopiedData {
    data: Vec<u8>,
    volume: f32,
    pos: usize
}

impl AudioCallback for CopiedData {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
            // to both fill in the silence and scale the wav data accordingly. Filling the silence
            // once the wav is finished is trivial, applying the volume is more tricky. We need to:
            // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
            // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
            // * Move the resulting range to a range centered around the value 128, the final range
            //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
            //
            // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
            // would not give correct results.
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}




fn main() {
    let mut f = File::open("c8games/PONG").unwrap();

    let mut s = Vec::new();

    _ = f.read_to_end(&mut s).unwrap();

    let len: &usize = &s.len();


    let mut cpu = CPU{
        registers : [0;16],
        address_reg: 0,
        program_counter:512,
        delay_timer : 0,
        sound_timer : 0
    };

    let mut memory = Memory{
        buf : [0;4096]
    };

    let mut stack = Stack{
        buf : Vec::new()
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip8", 64*10, 32*10)
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

    let mut keys : [bool;16] = [false;16];

    let wav = sdl2::audio::AudioSpecWAV::load_wav("beep.wav").unwrap();
    let audio_system = sdl_context.audio().unwrap();
    let audio_spec = AudioSpecDesired{ freq: None, channels: None, samples: None };
    let copied_data = CopiedData{ data: wav.buffer().to_vec(), pos: 0, volume: 100.0 };
    let mut audio_device = audio_system.open_playback(None, &audio_spec, move |spec| {
        copied_data
    }).unwrap();




    'running: loop {
            i = (i + 1) % 255;
            computer.display.canvas.set_draw_color(Color::WHITE);
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                        keys[0] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                        keys[1] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                        keys[2] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                        keys[3] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                        keys[4] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        keys[5] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                        keys[6] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                        keys[7] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        keys[8] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        keys[9] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        keys[10] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                        keys[11] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                        keys[12] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                        keys[13] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                        keys[14] = true;
                    },
                    Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                        keys[15] = true;
                    }
                    Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                        keys[0] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                        keys[1] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                        keys[2] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
                        keys[3] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                        keys[4] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                        keys[5] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                        keys[6] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                        keys[7] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                        keys[8] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                        keys[9] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                        keys[10] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                        keys[11] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                        keys[12] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                        keys[13] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                        keys[14] = false;
                    },
                    Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                        keys[15] = false;
                    }
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            for y in 0..16{
               // println!("{} {}",y,keys[y])
            }

            for i in 0..10{
                if &computer.cpu.program_counter <= &((4096u16) - 2) {
                    computer.executor(&keys);
                    computer.next_operation();
                }
            }



            if computer.cpu.sound_timer == 1 {
                audio_device.resume();
                let cb = audio_device.close_and_get_callback();
                audio_device = audio_system.open_playback(None, &audio_spec, move |spec| {
                    cb
                }).unwrap();
            }

            std::thread::sleep(Duration::from_nanos(10));
            computer.cpu.tick_timers();







            std::thread::sleep(Duration::from_millis(5));

        }








}



