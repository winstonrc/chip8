use std::env;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;

use chip8_core::*;
mod display;
use display::Display;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run path/to/game");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    display.canvas.clear();
    display.canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    let mut rom = File::open(&args[1]).expect("Unable to open file.");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load_rom(&buffer);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'gameloop;
            }
        }

        chip8.tick();
        display.draw_screen(&chip8);
    }
}
