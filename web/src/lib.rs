use iaith::Program;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn execute(source: &str) {
    let mut prog = Program::new(source);
    let output = prog.execute();
    alert(&output);
}
