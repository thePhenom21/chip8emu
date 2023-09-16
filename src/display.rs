use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas};


pub struct Display{
    pub canvas : WindowCanvas,
    pub screen : [u8;2048]
}
impl Display{


    pub fn real_draw(&mut self){


        self.canvas.present();
    }


    pub fn clear_display(&mut self){

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

    }

    pub fn draw(&mut self, x : u8, y : u8, n : u8, ram : &[u8; 4096], reg : u16, mut registers: [u8;16]){


        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        registers[15] = 0;

        let mut flipped = 0;
        for y_line in 0..n {

            let addr = reg + y_line as u16;
            let pixels = *ram.get(addr as usize).unwrap();

            for x_line in 0..8 {

                if (pixels & (0b1000_0000 >> x_line)) != 0 {

                    let x_r = (x + x_line) as usize % 64;
                    let y_r = (y + y_line) as usize % 32;

                    let idx = x_r + (64 * y_r);

                    flipped |= self.screen[idx];
                    self.screen[idx] ^= 1;

                }
            }

        }

        if flipped == 1 {
            registers[15] = 1;
        } else {
            registers[15] = 0;
        }

        for (i, pixel) in self.screen.iter().enumerate(){
            if *pixel == 1 {
// Convert our 1D array's index into a 2D (x,y) position
                let x = (i % 64) as u32;
                let y = (i / 64) as u32;
// Draw a rectangle at (x,y), scaled up by our SCALE value
                let rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                self.canvas.fill_rect(rect).unwrap();
            }
        }





            //::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));


    }

}

