use std::vec;
use std::iter::Enumerate;
use std::str::Chars;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TupleValue<'a> {
    String(&'a str)
}

#[derive(PartialEq, Eq, Debug)]
enum TupleState {
    Start,
    StartTuple
}

#[derive(PartialEq, Eq, Debug)]
pub enum TupleLexerError<'a> {
    TupleError(&'a str, usize, char),
}

pub struct TupleLexer<'a>{
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    current_parent: Vec<&'a str>,
    state: TupleState
}

impl<'a> TupleLexer<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: TupleState::Start,
            current_parent: vec![]
        }
    }

    fn start_if_possible(&mut self, index: usize, character: char)  -> Option<Result<TupleValue<'a>, TupleLexerError<'a>>> {
        if character == '(' {
            self.state = TupleState::StartTuple;
            return None;
        }
        None
    }

    fn transition(&mut self, index: usize, character: char) -> Option<Result<TupleValue<'a>, TupleLexerError<'a>>> {
        match self.state {
            TupleState::Start => {
                self.start_if_possible(index, character)
            },
            TupleState::StartTuple => {
                self.start_if_possible(index, character)
            },
        }
    }
}

impl <'a> Iterator for TupleLexer<'a> {
    type Item = Result<TupleValue<'a>, TupleLexerError<'a>>;
    fn next(&mut self) -> Option<Result<TupleValue<'a>, TupleLexerError<'a>>> {
        loop {
            return match self.characters.next() {
                Some((index, c)) => match self.transition(index, c) {
                    None => continue,
                    result => result
                },
                None => {
                    None
                },
            }
        }
    }
}