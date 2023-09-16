use std::io::Read;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};


pub struct Display{
    canvas : WindowCanvas,
    screen : [u8;2048]
}
impl Display{



    pub fn init_display() -> WindowCanvas{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 64*5, 32*5)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        //std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));



        return canvas;
    }

    pub fn real_draw(&mut self){
        for t in 0..64{
            for h in 0..32{
                self.canvas.draw_point(Point::new(t,h));
            }
        }

        self.canvas.present();
    }

    pub fn default() -> Display{
        let canvas = Self::init_display();
        return Display{canvas,screen:[0;2048]}
    }

    pub fn clear_display(&mut self){

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.canvas.present();
            //std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));


    }

    pub fn draw(&mut self,x : u8, y : u8, n : u8, buf : Vec<u8>,reg : u16){

            self.canvas.set_draw_color(Color::WHITE);


        for y_line in 0..n {

            let addr = reg + y_line as u16;
            let pixels = addr;

            for x_line in 0..8 {

                if (pixels & (0b1000_0000 >> x_line)) != 0 {

                    let x_r = (x + x_line) as usize % 32;
                    let y_r = (y + y_line) as usize % 64;

                    let idx = x_r + 32 * y_r;

                    self.screen[idx] ^= 1;
                }
            }
        }



            std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));


    }

}

