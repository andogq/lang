use std::{fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub struct Position {
    line: usize,
    character: usize,
}
impl Position {
    pub fn new() -> Self {
        Self {
            line: 0,
            character: 0,
        }
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }

    pub fn next_character(&mut self) {
        self.character += 1;
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.character + 1)
    }
}

pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
    position: Position,
}

#[allow(unused)]
pub enum TakeOption {
    Take,
    Skip,
    Stop,
    TakeAndStop,
    SkipAndStop,
}
impl From<bool> for TakeOption {
    fn from(b: bool) -> Self {
        if b {
            Self::Take
        } else {
            Self::Stop
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            current: None,
            position: Position::new(),
        }
    }

    pub fn next(&mut self) -> Option<(char, Position)> {
        self.current = self.chars.next();

        if self.current == Some('\n') {
            self.position.next_line();
        } else {
            self.position.next_character();
        }

        self.current.map(|c| (c, self.position.clone()))
    }

    pub fn peek_next(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    pub fn take_while_config<F, S>(&mut self, mut state: S, retake: bool, f: F) -> Vec<char>
    where
        F: Fn(char, S) -> (TakeOption, S),
    {
        let mut chars = if let (true, Some(c)) = (retake, self.current) {
            // Initialise chars with previous `c`
            vec![c]
        } else {
            Vec::new()
        };

        while let Some(c) = self.peek_next() {
            let (take_option, next_state) = f(c, state);

            match take_option {
                TakeOption::Take => {
                    // Save char
                    chars.push(c);

                    // Advance pointer
                    self.next();

                    // Move to next state
                    state = next_state;
                }
                TakeOption::Skip => {
                    // Advance pointer
                    self.next();

                    // Move to next state
                    state = next_state;
                }
                TakeOption::Stop => break,
                TakeOption::TakeAndStop => {
                    // Save char
                    chars.push(c);

                    // Advance pointer
                    self.next();

                    break;
                }
                TakeOption::SkipAndStop => {
                    // Advance pointer
                    self.next();

                    break;
                }
            };
        }

        chars
    }

    pub fn take_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        self.take_while_config((), false, |c, _| (TakeOption::from(f(c)), ()))
    }

    pub fn retake_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        self.take_while_config((), true, |c, _| (TakeOption::from(f(c)), ()))
    }

    pub fn skip_while<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        self.take_while(f);
    }
}
