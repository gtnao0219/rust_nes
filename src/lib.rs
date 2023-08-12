extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

pub mod apu;
pub mod cartridge;
pub mod controller;
pub mod cpu;
pub mod dma;
pub mod interrupt;
pub mod nes;
pub mod ppu;
pub mod ram;
pub mod renderer;
pub mod rom;

pub type Byte = u8;
pub type Word = u16;
pub type Cycle = u32;

#[wasm_bindgen]
extern "C" {
    fn render_canvas(pixels: &[u8]);
}

#[wasm_bindgen]
pub struct WasmNES(nes::NES);

#[wasm_bindgen]
impl WasmNES {
    pub fn new(rom_data: &[u8]) -> Self {
        WasmNES(nes::NES::new(rom_data))
    }
    pub fn frame(&mut self) {
        self.0.frame();
    }
    pub fn key_down(&mut self, key: u8) {
        self.0.key_down(key);
    }
    pub fn key_up(&mut self, key: u8) {
        self.0.key_up(key);
    }
}

pub fn log(s: &str) {
    log_1(&JsValue::from(s));
}
