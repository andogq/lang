use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            current: None,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.current = self.chars.next();
        self.current
    }

    pub fn peek_next(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    pub fn take_while_config<F, S>(&mut self, mut state: S, retake: bool, f: F) -> Vec<char>
    where
        F: Fn(char, S) -> (bool, S),
    {
        let mut chars = if let (true, Some(c)) = (retake, self.current) {
            // Initialise chars with previous `c`
            vec![c]
        } else {
            Vec::new()
        };

        while let Some(c) = self.peek_next() {
            let (valid, next_state) = f(c, state);

            if !valid {
                break;
            } else {
                // Save char
                chars.push(c);

                // Advance pointer
                self.next();

                // Move to next state
                state = next_state;
            }
        }

        chars
    }

    pub fn take_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        self.take_while_config((), false, |c, _| (f(c), ()))
    }

    pub fn retake_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        self.take_while_config((), true, |c, _| (f(c), ()))
    }

    pub fn skip_while<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        self.take_while(f);
    }
}
