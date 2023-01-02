pub mod cpu;
pub mod display;
pub mod keypad;
extern crate rand;
extern crate sdl2;

use std::thread;
use std::time::Duration;
use std::env; 

const WIDTH: usize= 64;
const HEIGHT: usize = 32;
const REGISTER_COUNT: usize = 16;
const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const START_ADDR: usize = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET_START: usize = 0x50;
fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl  = sdl2::init().unwrap();

    let args: Vec<String>  = env::args().collect();
    if args.len() < 2 {
        panic!("No ROM name provided!");
    }
    let filename = &args[1];

    let mut window = display::DisplayWindow::new(&sdl);
    let mut input = keypad::Input::new(&sdl);
    let mut cpu = cpu::Chip::new();
    cpu.load_rom(filename);

    while let Ok(keypad) = input.poll() {
        let out = cpu.cycle(keypad);
        if out.video_change {
        window.draw(out.video);
        }
        thread::sleep(sleep_duration);
    }

}