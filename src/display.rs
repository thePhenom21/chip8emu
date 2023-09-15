use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};


pub struct Display{
    canvas : WindowCanvas,
}
impl Display{



    pub fn init_display() -> WindowCanvas{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));

        return canvas;
    }

    pub fn default() -> Display{
        let canvas = Self::init_display();
        return Display{canvas}
    }

    pub fn clear_display(&mut self){

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.canvas.present();
            std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));


    }

    pub fn draw(&mut self,x : u8, y : u8, n : u8, buf : Vec<u8>){

            self.canvas.set_draw_color(Color::WHITE);

            let mut c : usize = 0;

            while c < (n as usize) {
                self.canvas.fill_rect(Rect::new(x as i32, (y as usize +c) as i32, 8, 1)).unwrap();
                c+=1;
            }

            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));


    }

}

