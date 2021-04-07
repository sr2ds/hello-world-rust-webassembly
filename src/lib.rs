mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-world-rust-webassembly!");
}

#[wasm_bindgen]
pub fn calculate(number1: i32, number2: i32) {
    let new_number :i32 = number1 + number2;
    alert(&format!("O resultado da soma é: {}!", new_number));
}

#[wasm_bindgen]
pub fn make_array(lenght: i32) {
    let mut vec = Vec::new();
    let mut i: i32 = 1;
    loop {
        vec.push(i);
        i +=1;
        if i == lenght {
            break;
        }
    }
}