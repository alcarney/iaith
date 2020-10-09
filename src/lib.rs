use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq)]
enum Token {
    Increment,
    Decrement,
    ShiftR,
    ShiftL,
    Print,
}

pub struct Program {
    pointer: i32,
    tape: HashMap<i32, u8>,
    program: Vec<Token>,
    index: usize,
}

impl Program {
    pub fn new(source: &str) -> Program {
        Program {
            pointer: 0,
            index: 0,
            tape: HashMap::new(),
            program: parse(source),
        }
    }

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
            }

            self.index += 1;
        }

        String::from(str::from_utf8(&output[..]).unwrap())
    }
}

fn parse(source: &str) -> Vec<Token> {
    source
        .chars()
        .map(|c| match c {
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '>' => Ok(Token::ShiftR),
            '<' => Ok(Token::ShiftL),
            '.' => Ok(Token::Print),
            _ => Err(()),
        })
        .filter_map(Result::ok)
        .collect()
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
}
