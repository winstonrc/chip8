use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

use chip8_core::*;

mod keyboard;
use keyboard::key2btn;

#[wasm_bindgen]
pub struct Chip8Wasm {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Chip8Wasm, JsValue> {
        let chip8 = Chip8::new();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into().map_err(|_| ()).unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let chip8wasm = Self { chip8, ctx };

        Ok(chip8wasm)
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, event: KeyboardEvent, pressed: bool) {
        let key = event.key();
        if let Some(k) = key2btn(&key) {
            self.chip8.keypress(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        self.chip8.load_rom(&data.to_vec());
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        let display = self.chip8.get_display();
        for (i, _) in display
            .iter()
            .enumerate()
            .take(SCREEN_WIDTH * SCREEN_HEIGHT)
        {
            if display[i] {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;

                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                )
            }
        }
    }
}
