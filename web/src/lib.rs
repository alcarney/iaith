use iaith::brainf::Program;
use iaith::brainf::State;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &[u8]);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_message(s: &str);
}

fn as_tape(prog: &Program) -> (Vec<u8>, usize) {
    let memory = &prog.tape;

    let max_index = *memory.keys().max().unwrap() as usize;
    let mut tape = vec![0; max_index + 1];

    for i in 0..(max_index + 1) {
        tape[i] = *memory.get(&(i as i32)).unwrap_or(&0);
    }

    (tape, prog.pointer as usize)
}

fn set_cell(document: &Document, tape: &Element, index: usize, current: usize, value: &u8) {
    let cell = match tape
        .query_selector(&format!("[data-index=\"{}\"]", index))
        .expect("expected success")
    {
        Some(c) => c,
        None => {
            let c = document.create_element("div").expect("expected div");
            c.set_attribute("data-index", &format!("{}", index))
                .expect("expected success");
            tape.append_with_node_1(&c).expect("success");

            c
        }
    };

    let mut classes: String = "grid flex-shrink-0 w-12 h-12 p-2 bg-white place-center".into();

    if index == current {
        classes += " border-2 border-gray-600";
    } else {
        classes += " border";
    }

    cell.set_class_name(&classes);
    cell.set_text_content(Some(&format!("{}", value)))
}

fn render_tape(cells: &[u8], current: usize) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let tape = document
        .get_element_by_id("tape")
        .expect("document should have a body");

    for (idx, value) in cells.iter().enumerate() {
        set_cell(&document, &tape, idx, current, value);
    }
}

#[wasm_bindgen]
pub fn execute(source: &str) -> String {
    let mut prog = Program::new(source);
    let mut output = String::new();

    let mut count: usize = 0;

    while prog.state != State::Terminated {
        match prog.step() {
            Some(s) => output += &s,
            None => (),
        }

        let (cells, current) = as_tape(&prog);
        render_tape(&cells, current);
        count += 1;
    }
    log_message(&format!("Executed {} instructions", count));

    output
}
