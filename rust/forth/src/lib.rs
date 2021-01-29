use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<i32>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq)]
enum State {
    Normal,
    NewWordStart,
    NewWordProcessing(String),
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_ref()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut state = State::Normal;
        let mut word_stack = Vec::new();

        for operation in input.to_ascii_lowercase().split_ascii_whitespace() {
            let mut ops = self
                .words
                .get(operation)
                .unwrap_or(&vec![operation.to_string()])
                .clone();
            match &state {
                State::NewWordStart if operation.chars().all(|c| c.is_numeric()) => {
                    return Err(Error::InvalidWord);
                }
                State::NewWordStart => {
                    state = State::NewWordProcessing(operation.to_string());
                }
                State::NewWordProcessing(word) if ops[0] == ";" => {
                    self.words.insert(word.clone(), word_stack);
                    word_stack = Vec::new();
                    state = State::Normal;
                }
                State::NewWordProcessing(_) => {
                    word_stack.append(&mut ops);
                }
                State::Normal => {
                    for op in ops {
                        match op.as_str() {
                            ":" => {
                                state = State::NewWordStart;
                            }
                            "dup" | "drop" if self.stack.is_empty() => {
                                return Err(Error::StackUnderflow);
                            }
                            "+" | "-" | "*" | "/" | "swap" | "over" if self.stack.len() < 2 => {
                                return Err(Error::StackUnderflow);
                            }
                            "dup" => {
                                self.stack.push(*self.stack.last().unwrap());
                            }
                            "drop" => {
                                self.stack.pop();
                            }
                            "swap" => {
                                let l = self.stack.len();
                                self.stack.swap(l - 1, l - 2)
                            }
                            "over" => self.stack.push(self.stack[self.stack.len() - 2]),
                            "/" if self.stack.last().unwrap() == &0 => {
                                return Err(Error::DivisionByZero);
                            }
                            "+" | "-" | "*" | "/" => {
                                let n2 = self.stack.pop().unwrap();
                                let n1 = self.stack.pop().unwrap();
                                self.stack.push(match op.as_str() {
                                    "+" => n1 + n2,
                                    "-" => n1 - n2,
                                    "*" => n1 * n2,
                                    "/" => n1 / n2,
                                    _ => unreachable!(),
                                });
                            }
                            _ => {
                                if let Ok(n) = op.parse::<i32>() {
                                    self.stack.push(n)
                                } else {
                                    return Err(Error::UnknownWord);
                                }
                            }
                        }
                    }
                }
            }
        }
        if state == State::Normal {
            Ok(())
        } else {
            Err(Error::InvalidWord)
        }
    }
}
