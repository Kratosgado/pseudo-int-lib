pub mod constants;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod std;
mod utils;

pub use evaluator::evaluator::Evaluator;
pub use lexer::lexer::Lexer;
pub use parser::parser::Parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, pseudo-int-lib!");
}
