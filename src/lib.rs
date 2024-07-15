pub mod constants;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod std;
mod utils;

use constants::error_handler::PseudoError;
pub use evaluator::evaluator::Evaluator;
pub use lexer::lexer::Lexer;
pub use parser::parser::Parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // #[wasm_bindgen(constructor)]
    // fn new(message: &str) -> WebPseudoError;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Into<JsValue> for PseudoError {
    fn into(self) -> JsValue {
        JsValue::from(self.to_string())
    }
}

/// Executes pseudocode and returns print statements
///
/// # Errors
/// Returns error of type PseudoError
/// This function will return an error if .
#[no_mangle]
#[wasm_bindgen]
pub fn interpret(inputs: &str) -> Result<JsValue, PseudoError> {
    let mut lexer = Lexer::new(inputs);
    let tokens = lexer.tokenize().map_err(|_e| _e)?;

    let mut parser = Parser::new(tokens);
    let parsed_token = parser.parse()?;

    let mut evaluator = Evaluator::new(parsed_token);

    match evaluator.evaluate() {
        Ok(output) => Ok(JsValue::from_str(output.as_str())),
        Err(e) => Err(e),
    }
}
