use super::FONTSET_SIZE;

pub const FONTSET: [u8; FONTSET_SIZE] = [
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
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F

];
use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::{WIDTH,HEIGHT};

const SCALE_FACTOR: u32 = 20;

pub struct DisplayWindow {
    canvas: Canvas<Window>,
}
impl DisplayWindow {

    pub fn new(sdl: &sdl2::Sdl) -> DisplayWindow {
        let video = sdl.video().unwrap();
        let window = video.window(
            "CHIP-8 simulator",
            (WIDTH as u32) * SCALE_FACTOR,
            (HEIGHT as u32) * SCALE_FACTOR,

        ).position_centered().opengl().build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        DisplayWindow {canvas:canvas}
    }

    pub fn draw(&mut self, pixels: &[[u8;WIDTH]; HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present()
    }
 
}

   pub fn color(value: u8) -> pixels::Color {
        if value == 0 {
            pixels::Color::RGB(0,0,0)
        }
        else 
        {
            pixels::Color::RGB(0,250,0)
        }
    }
