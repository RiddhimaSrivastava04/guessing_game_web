use wasm_bindgen::prelude::*;
use js_sys::Math;
use web_sys::HtmlInputElement;
use web_sys::window;

static mut SECRET_NUMBER: u32 = 0;

#[wasm_bindgen(start)]
pub fn start() {
    set_secret_number();
}

#[wasm_bindgen]
pub fn check_guess() {
    let document = window().unwrap().document().unwrap();
    let input = document.get_element_by_id("guess").unwrap();
    let input = input.dyn_into::<HtmlInputElement>().unwrap();

    let guess: u32 = input.value().parse().unwrap_or(0);

    let result = document.get_element_by_id("result").unwrap();

    unsafe {
        if guess == SECRET_NUMBER {
            result.set_inner_html("🎉 Correct! You guessed it!");
        } else if guess < SECRET_NUMBER {
            result.set_inner_html("🔽 Too small!");
        } else {
            result.set_inner_html("🔼 Too big!");
        }
    }
}

#[wasm_bindgen]
pub fn reset_game() {
    set_secret_number();
    let document = window().unwrap().document().unwrap();

    let input = document.get_element_by_id("guess").unwrap();
    let input = input.dyn_into::<HtmlInputElement>().unwrap();
    input.set_value("");

    let result = document.get_element_by_id("result").unwrap();
    result.set_inner_html("Game reset! Try again.");
}

fn set_secret_number() {
    let random_number = (Math::random() * 100.0).floor() as u32 + 1;
    unsafe {
        SECRET_NUMBER = random_number;
    }
}
