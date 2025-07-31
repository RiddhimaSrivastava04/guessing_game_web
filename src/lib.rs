use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlInputElement};
use js_sys::Math;

static mut SECRET_NUMBER: u32 = 0;

#[wasm_bindgen(start)]
pub fn start() {
    generate_secret();
}

fn generate_secret() {
    unsafe {
        SECRET_NUMBER = (Math::random() * 100.0).floor() as u32 + 1;
    }
}

#[wasm_bindgen]
pub fn check_guess() {
    let document = window().unwrap().document().unwrap();
    let input = document.get_element_by_id("guess").unwrap();
    let input: HtmlInputElement = input.dyn_into().unwrap();
    let value = input.value().parse::<u32>();

    let message = match value {
        Ok(guess) => unsafe {
            if guess < SECRET_NUMBER {
                "Too small!"
            } else if guess > SECRET_NUMBER {
                "Too big!"
            } else {
                "Correct!"
            }
        },
        Err(_) => "Please enter a valid number!",
    };

    let result = document.get_element_by_id("result").unwrap();
    result.set_inner_html(message);
}

#[wasm_bindgen]
pub fn reset_game() {
    generate_secret();
    let document = window().unwrap().document().unwrap();
    document.get_element_by_id("guess").unwrap().set_attribute("value", "").unwrap();
    document.get_element_by_id("result").unwrap().set_inner_html("New game started!");
}
