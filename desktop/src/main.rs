use std::env;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;

use chip8_core::*;
mod display;
use display::Display;
mod keyboard;
use keyboard::key2btn;
use sdl2::keyboard::Keycode;

const TICKS_PER_FRAME: usize = 10;

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
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip8.keypress(k, false);
                    }
                }
                _ => (),
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        chip8.tick_timers();
        display.draw_screen(&chip8);
    }
}
