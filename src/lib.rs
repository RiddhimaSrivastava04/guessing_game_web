use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static SECRET_NUMBER: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
}

#[wasm_bindgen]
pub fn reset_game() {
    use rand::Rng;
    let secret = rand::thread_rng().gen_range(1..=100);
    SECRET_NUMBER.with(|s| *s.borrow_mut() = secret);
}

#[wasm_bindgen]
pub fn check_guess(guess: u32) -> String {
    SECRET_NUMBER.with(|s| {
        let secret = *s.borrow();
        if guess < secret {
            "Too small!".to_string()
        } else if guess > secret {
            "Too big!".to_string()
        } else {
            "You guessed it!".to_string()
        }
    })
}
