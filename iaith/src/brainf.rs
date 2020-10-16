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

/// A container for all the state needed to execute a brainf*ck program.
pub struct Program {
    /// A vector of tokens, representing the executable source code of the program
    pub program: Vec<Token>,
    /// A 'pointer' into the vector of tokens representing where in the code execution
    /// is currently happening.
    pub index: usize,
    bracket_map: HashMap<usize, usize>,
    /// A map of cell number to value, representing the tape the program is manipulating
    pub tape: HashMap<i32, u8>,
    /// A 'pointer' represnting which cell in the tape the program is currently
    /// manipulating.
    pub pointer: i32,
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
        }
    }

    /// Execute the program, until it terminates.
    pub fn execute(&mut self) -> String {
        let mut output: Vec<u8> = Vec::new();

        while self.index < self.program.len() {
            let ins = self.program.get(self.index).unwrap();

            let val = match self.tape.get(&self.pointer) {
                Some(v) => *v,
                None => 0,
            };

            match ins {
                Token::ShiftR => self.pointer += 1,
                Token::ShiftL => self.pointer -= 1,
                Token::Increment => {
                    self.tape.insert(self.pointer, val + 1);
                }
                Token::Decrement => {
                    self.tape.insert(self.pointer, val - 1);
                }
                Token::Print => output.push(val),
                Token::LoopStart => {
                    if val == 0 {
                        self.index = *self.bracket_map.get(&self.index).unwrap();
                        continue;
                    }
                }
                Token::LoopEnd => {
                    if val != 0 {
                        self.index = *self.bracket_map.get(&self.index).unwrap();
                        continue;
                    }
                }
            }

            self.index += 1;
        }

        String::from(str::from_utf8(&output[..]).unwrap())
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
