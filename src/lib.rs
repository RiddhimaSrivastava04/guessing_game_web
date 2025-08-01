use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlInputElement};
use js_sys::Math;

static mut SECRET: u32 = 0;

#[wasm_bindgen(start)]
pub fn start() {
    unsafe {
        SECRET = (Math::random() * 100.0).floor() as u32 + 1;
    }
}

#[wasm_bindgen]
pub fn check_guess(guess: u32) -> String {
    unsafe {
        if guess < SECRET {
            "Too small!".into()
        } else if guess > SECRET {
            "Too big!".into()
        } else {
            "Correct!".into()
        }
    }
}

#[wasm_bindgen]
pub fn reset_game() {
    unsafe {
        SECRET = (Math::random() * 100.0).floor() as u32 + 1;
    }
}
