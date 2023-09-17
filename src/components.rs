use sdl2::audio::AudioSpecDesired;
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
    pub program_counter: u16,
    pub delay_timer : u8,
    pub sound_timer : u8,
}

impl CPU {
    pub fn tick_timers(&mut self) {

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if (self.sound_timer > 0) {
            if self.sound_timer == 1 {

            }
            self.sound_timer -= 1;
        }
    }
}


pub struct Memory{
    pub buf : [u8;4096]
}

pub struct Stack{
    pub buf : Vec<u16>
}

