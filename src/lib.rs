use wasm_bindgen::prelude::*;
use js_sys::Math;
use std::cell::RefCell;

thread_local! {
    static SECRET_NUMBER: RefCell<u32> = RefCell::new(((Math::random() * 100.0).floor() as u32) + 1);
}

#[wasm_bindgen]
pub fn check_guess(guess: u32) -> String {
    SECRET_NUMBER.with(|secret| {
        let number = *secret.borrow();
        if guess < number {
            "Too small!".to_string()
        } else if guess > number {
            "Too big!".to_string()
        } else {
            "Correct! 🎉".to_string()
        }
    })
}

#[wasm_bindgen]
pub fn reset_game() {
    SECRET_NUMBER.with(|secret| {
        *secret.borrow_mut() = ((Math::random() * 100.0).floor() as u32) + 1;
    });
}
