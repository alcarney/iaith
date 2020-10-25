use iaith::brainf::Program;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &[u8]);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_message(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct VMState {
    pub pointer: i32,
    pub memory: Vec<u8>,
    pub stdout: String,
}

fn as_tape(prog: &Program) -> Vec<u8> {
    let memory = &prog.tape;

    let max_index = *memory.keys().max().unwrap() as usize;
    let mut tape = vec![0; max_index + 1];

    for i in 0..(max_index + 1) {
        tape[i] = *memory.get(&(i as i32)).unwrap_or(&0);
    }

    tape
}

#[wasm_bindgen]
pub fn execute(source: &str) -> JsValue {
    let mut prog = Program::new(source);
    let output = prog.execute();

    let state = VMState {
        stdout: output,
        memory: as_tape(&prog),
        pointer: prog.pointer,
    };

    JsValue::from_serde(&state).unwrap()
}
