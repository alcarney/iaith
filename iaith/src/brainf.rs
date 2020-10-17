//! A brainf*ck implementation.
//!
//! As described on the [EsoLang](https://esolangs.org/wiki/Brainfuck#Language_overview)
//! wiki.
//!
//! Brainf*ck operates on an array of memory cells, each initially set to zero. There is
//! a pointer, initially pointing at the first memory cell. The language contains the
//! following commands.
//!
//! | Command | Description |
//! |-|-|
//! | `>` | Move the pointer to the right |
//! | `<` | Move the pointer to the left |
//! | `+` | Increment the memory cell at the pointer |
//! | `-` | Decrement the memory cell at the pointer |
//! | `.` | Output the character signified by the cell at the pointer |
//! | `,` | Input a character and store it in the cell at the pointer (Not yet implemted) |
//! | `[` | Jump past the matching `]` if the cell at the pointer is `0` |
//! | `]` | Jump back to the matching `[` if the cell at the pointer is non-zero |
//!
//! All other characters are considered comments.
use std::collections::HashMap;
use std::str;

/// Represents a valid control character in a brainf*ck program.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Corresponds with the `[` character
    LoopStart,
    /// Corresponds with the `]` character
    LoopEnd,
    /// Corresponds with the `+` character
    Increment,
    /// Corresponds with the `-` character
    Decrement,
    /// Corresponds with the `>` character
    ShiftR,
    /// Corresponds with the `<` character
    ShiftL,
    /// Corresponds with the `.` character
    Print,
}

/// Used to represent the state the program is in.
#[derive(Debug, PartialEq)]
pub enum State {
    /// The program has not yet been executed.
    New,
    /// The program is currently being executed.
    Executing,
    /// The program has been executed.
    Terminated,
}

/// A container for all the state needed to execute a brainf*ck program.
pub struct Program {
    /// A vector of tokens, representing the executable source code of the program
    pub program: Vec<Token>,
    /// A 'pointer' into the vector of tokens representing where in the code execution
    /// is currently happening.
    pub index: usize,
    /// A map of cell number to value, representing the tape the program is manipulating
    pub tape: HashMap<i32, u8>,
    /// A 'pointer' represnting which cell in the tape the program is currently
    /// manipulating.
    pub pointer: i32,
    /// Represents the state of the program's execution
    pub state: State,

    bracket_map: HashMap<usize, usize>,
}

impl Program {
    /// Parse the given source code and construct an new instance of a program ready to
    /// be executed. Currently panics if the given source does not represent a valid
    /// program.
    pub fn new(source: &str) -> Program {
        let (program, bracket_map) = parse(source);
        Program {
            pointer: 0,
            index: 0,
            tape: HashMap::new(),
            program,
            bracket_map,
            state: State::New,
        }
    }

    /// Execute the next instruction in the program.
    pub fn step(&mut self) -> Option<String> {
        match self.state {
            State::New => self.state = State::Executing,
            State::Executing => {
                if self.index >= self.program.len() {
                    self.state = State::Terminated;
                    return None;
                }
            }
            State::Terminated => return None,
        }

        let instruction = self.program.get(self.index).unwrap();
        let value = match self.tape.get(&self.pointer) {
            Some(v) => *v,
            None => 0,
        };

        let mut out = String::new();
        match instruction {
            Token::ShiftR => self.pointer += 1,
            Token::ShiftL => self.pointer -= 1,
            Token::Increment => {
                self.tape.insert(self.pointer, value + 1);
            }
            Token::Decrement => {
                self.tape.insert(self.pointer, value - 1);
            }
            Token::LoopStart => {
                if value == 0 {
                    self.index = *self.bracket_map.get(&self.index).unwrap();
                    return Some(out);
                }
            }
            Token::LoopEnd => {
                if value != 0 {
                    self.index = *self.bracket_map.get(&self.index).unwrap();
                    return Some(out);
                }
            }
            Token::Print => {
                out += str::from_utf8(&[value]).unwrap();
            }
        }

        self.index += 1;
        Some(out)
    }

    /// Execute the program, until it terminates.
    pub fn execute(&mut self) -> String {
        let mut output = String::new();

        while self.state != State::Terminated {
            match self.step() {
                Some(s) => output += &s,
                None => (),
            }
        }

        output
    }
}

fn parse(source: &str) -> (Vec<Token>, HashMap<usize, usize>) {
    let mut brackets: Vec<usize> = Vec::new();
    let mut bracket_map = HashMap::new();

    let program = source
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '[' => {
                brackets.push(i);
                Ok(Token::LoopStart)
            }
            ']' => {
                match brackets.pop() {
                    Some(idx) => {
                        bracket_map.insert(idx, i);
                        bracket_map.insert(i, idx);
                    }
                    None => panic!("Unmatched brackets!"),
                };

                Ok(Token::LoopEnd)
            }
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '>' => Ok(Token::ShiftR),
            '<' => Ok(Token::ShiftL),
            '.' => Ok(Token::Print),
            _ => Err(()),
        })
        .filter_map(Result::ok)
        .collect();

    (program, bracket_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_program() {
        let mut prog = Program::new("++");
        assert_eq!(prog.index, 0);
        assert_eq!(prog.state, State::New);

        let ret = prog.step();
        assert_eq!(ret, Some(String::from("")));
        assert_eq!(prog.state, State::Executing);
        assert_eq!(prog.index, 1);
        assert_eq!(*prog.tape.get(&0).unwrap(), 1);

        let ret = prog.step();
        assert_eq!(ret, Some(String::from("")));
        assert_eq!(prog.state, State::Executing);
        assert_eq!(prog.index, 2);
        assert_eq!(*prog.tape.get(&0).unwrap(), 2);

        let ret = prog.step();
        assert_eq!(ret, None);
        assert_eq!(prog.state, State::Terminated);
        assert_eq!(prog.index, 2);
        assert_eq!(*prog.tape.get(&0).unwrap(), 2);
    }

    #[test]
    fn test_print_h() {
        let source = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
        let mut prog = Program::new(source);

        assert_eq!("H", prog.execute());
    }

    #[test]
    fn test_print_hello_world() {
        let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut prog = Program::new(source);

        assert_eq!("Hello World!\n", prog.execute());
    }
}
