use std::collections::HashMap;
use std::str;

#[derive(Debug, PartialEq)]
enum Token {
    LoopStart,
    LoopEnd,
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
    bracket_map: HashMap<usize, usize>,
    index: usize,
}

impl Program {
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
