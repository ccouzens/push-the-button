mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SIZE: usize = 10;

#[wasm_bindgen]
#[derive(Default)]
pub struct Game {
    leds: [bool; SIZE],
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.leds.len()
    }

    pub fn button_down(&mut self, button: u8) {
        for (led, i) in self.leds.iter_mut().zip(0..) {
            *led = button >= i;
        }
    }

    pub fn leds(&self) -> *const bool {
        self.leds.as_ptr()
    }
}
