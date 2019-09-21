mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SIZE: usize = 10;

#[wasm_bindgen]
#[derive(Default)]
pub struct Game {
    mapping: [u8; SIZE],
    leds: [bool; SIZE],
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        set_panic_hook();
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.leds.len()
    }

    pub fn button_down(&mut self, button: u8) {
        match button {
            0 => {
                for led in self.leds.iter_mut() {
                    *led = false
                }
            }
            1 => {
                for (led, i) in self.leds.iter_mut().zip(0..) {
                    if button != i {
                        *led = !*led;
                    }
                }
            }
            2 => {
                if let Some(led) = self.leds.get_mut(button as usize) {
                    *led = !*led;
                }
            }
            3 => {
                for led in self.leds.iter_mut().skip(1).step_by(2) {
                    *led = true;
                }
            }
            4 => {
                for led in self.leds.iter_mut().skip(1).step_by(2) {
                    *led = false;
                }
            }
            5 => {
                for led in self.leds.iter_mut().step_by(2) {
                    *led = true;
                }
            }
            6 => {
                for led in self.leds.iter_mut().step_by(2) {
                    *led = false;
                }
            }
            7 => {
                for led in self.leds.iter_mut().take(1) {
                    *led = !*led;
                }
            }
            8 => {
                for led in self.leds.iter_mut().take(2) {
                    *led = !*led;
                }
            }
            9 => {
                for led in self.leds.iter_mut().take(3) {
                    *led = !*led;
                }
            }

            _ => {}
        }
    }

    pub fn leds(&self) -> *const bool {
        self.leds.as_ptr()
    }

    pub fn mapping(&self) -> *const u8 {
        self.mapping.as_ptr()
    }
}
