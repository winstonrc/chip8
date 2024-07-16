use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use chip8_core::*;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

pub struct Display {
    pub canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.clear();
        canvas.present();

        Self { canvas }
    }

    pub fn draw_screen(&mut self, chip8: &Chip8) {
        // Clear canvas as black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let screen_buf = chip8.get_display();

        // Now set draw color to white then iterate through each point and see if it should be
        // drawn.
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                // Convert the 1D array's index into a 2D (x, y) position.
                let x = (i % SCREEN_WIDTH) as u32;
                let y = (i / SCREEN_WIDTH) as u32;

                // Draw a rectangle at (x, y), scaled up by the SCALE value
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present();
    }
}
