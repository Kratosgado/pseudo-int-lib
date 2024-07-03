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
pub fn interpret(inputs: &str)-> Result<JsValue, JsError> {
    let mut lexer = Lexer::new(inputs);
    let tokens = lexer.tokenize().map_err(|e| JsError::from(e))?;

    let mut parser = Parser::new(tokens);
    let parsed_token = parser.parse().map_err(|e| JsError::from(e))?;

    let mut evaluator = Evaluator::new(parsed_token);

    match evaluator.evaluate() {
        Ok(output) => Ok(JsValue::from_str(output.as_str())),
        Err(e) => Err(JsError::from(e) )   
    }
}