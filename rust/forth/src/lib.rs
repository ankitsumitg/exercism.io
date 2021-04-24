use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stk: Vec<Value>,
    definitions: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stk: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stk
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input = input.to_ascii_uppercase();
        let mut input = input.split_ascii_whitespace();
        loop {
            let op = input.next();
            if op.is_none() {
                return Ok(());
            }
            let op = op.unwrap();
            if let Some(val) = Forth::to_value(op) {
                self.push(val);
            } else {
                match op {
                    name if self.definitions.contains_key(name) => {
                        if let Err(e) = self.eval(&self.definitions.get(name).unwrap().clone()) {
                            return Err(e);
                        }
                    }
                    "+" => {
                        let (a, b) = self.pop2_stack()?;
                        self.push(a + b)
                    }
                    "-" => {
                        let (a, b) = self.pop2_stack()?;
                        self.push(b - a)
                    }
                    "*" => {
                        let (a, b) = self.pop2_stack()?;
                        self.push(a * b)
                    }
                    "/" => {
                        let (a, b) = self.pop2_stack()?;
                        if a == 0 {
                            return Err(Error::DivisionByZero);
                        }
                        self.push(b / a)
                    }
                    "DUP" => {
                        let a = self.pop_stack()?;
                        self.push_many(&[a, a])
                    }
                    "DROP" => {
                        let _ = self.pop_stack()?;
                    }
                    "SWAP" => {
                        let (a, b) = self.pop2_stack()?;
                        self.push_many(&[a, b])
                    }
                    "OVER" => {
                        let (a, b) = self.pop2_stack()?;
                        self.push_many(&[b, a, b])
                    }
                    ":" => match input.next() {
                        Some(name) if Forth::is_valid_word(name) => {
                            let mut has_semicolon = false;
                            let args: Vec<_> = input
                                .by_ref()
                                .take_while(|&s| {
                                    has_semicolon = has_semicolon || (s == ";");
                                    !has_semicolon
                                })
                                .collect();
                            if !has_semicolon {
                                return Err(Error::InvalidWord);
                            }
                            let args: String = args
                                .into_iter()
                                .map(|w| {
                                    self.definitions
                                        .get(w)
                                        .cloned()
                                        .unwrap_or_else(|| w.to_string())
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            self.definitions.insert(name.to_string(), args);
                        }
                        _ => return Err(Error::InvalidWord),
                    },
                    _ => return Err(Error::UnknownWord),
                }
            }
        }
    }

    fn to_value(input: &str) -> Option<Value> {
        input.parse::<Value>().ok()
    }

    fn is_valid_word(word: &str) -> bool {
        !word.chars().next().unwrap().is_ascii_digit()
    }

    fn pop_stack(&mut self) -> Result<Value, Error> {
        self.stk.pop().ok_or(Error::StackUnderflow)
    }

    fn pop2_stack(&mut self) -> Result<(Value, Value), Error> {
        Ok((self.pop_stack()?, self.pop_stack()?))
    }

    fn push(&mut self, value: Value) {
        self.stk.push(value)
    }

    fn push_many(&mut self, values: &[Value]) {
        self.stk.extend_from_slice(values)
    }
}