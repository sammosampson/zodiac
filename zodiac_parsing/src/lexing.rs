use std::vec;
use std::iter::Enumerate;
use std::str::Chars;

pub struct Lexer<'a> {
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    current_parent: Vec<&'a str>,
    state: State
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Control(&'a str),
    EndControl(&'a str),
    Property(&'a str),
    PropertyValue(&'a str),
}

#[derive(PartialEq, Eq, Debug)]
pub struct LexerError<'a>(&'a str, usize, char);

impl<'a> LexerError<'a> {
    pub fn could_not_find_start_tag(index: usize, character: char) -> Self {
        LexerError("could not find control start tag (<)", index, character)
    }

    pub fn could_not_find_control_name(index: usize, character: char) -> Self {
        LexerError("could not find control name", index, character)
    }

    pub fn could_not_find_property_start_symbol(index: usize, character: char) -> Self {
        LexerError("could not find property start symbol (\")", index, character)
    }

    pub fn could_not_find_control_close_symbol(index: usize, character: char) -> Self {
        LexerError("could not find control close symbol (>)", index, character)
    }

    pub fn closing_wrong_tag(index: usize, character: char) -> Self {
        LexerError("trying to close the wrong control", index, character)
    }

    pub fn could_not_find_control_to_close(index: usize, character: char) -> Self {
        LexerError("could not find control to close", index, character)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Start,
    StartControl,
    InControl(usize),
    EndControl,
    EndNestedControl(usize),
    InProperty(usize),
    InPropertyValue(usize),
    StartPropertyValue,
    InWhitespace
}

impl<'a> Lexer<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: State::Start,
            current_parent: vec![]
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '<' {
            self.state = State::StartControl;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
        Some(Err(LexerError::could_not_find_start_tag(index, character)))
    }
    
    fn start_control_if_possible(&mut self, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '/' {
            self.state = State::EndNestedControl(index + 1);
            return None;
        }
        if !character.is_whitespace() {
            self.state = State::InControl(index);
            return None;
        }
        Some(Err(LexerError::could_not_find_control_name(index, character)))
    }

    fn produce_control_result(&mut self, start: usize, index: usize)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        let control_name = self.splice_input(start, index);
        self.current_parent.push(control_name);
        Some(Ok(Token::Control(control_name)))
    }

    fn handle_inside_control(&mut self, start: usize, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character.is_whitespace() {
            self.state = State::InWhitespace;
            return self.produce_control_result(start,index);
        }
        if character == '>' {
            self.state = State::Start;
            return self.produce_control_result(start,index);
        }
        if character == '/' {
            self.state = State::EndControl;
            return self.produce_control_result(start,index);
        }
        None
    }

    fn produce_property_result(&mut self, start: usize, index: usize)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        Some(Ok(Token::Property(self.splice_input(start, index))))
    }

    fn handle_inside_property(&mut self, start: usize, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character.is_whitespace() {
            self.state = State::InWhitespace;
            return self.produce_property_result(start,index);
        }
        if character == '/' {
            self.state = State::EndControl;
            return self.produce_property_result(start,index);
        }
        if character == '=' {
            self.state = State::StartPropertyValue;
            return self.produce_property_result(start,index);
        }
        if character == '>' {
            self.state = State::Start;
            return self.produce_property_result(start,index);
        }
        None
    }
    
    fn start_property_value_if_possible(&mut self, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '"' {
            self.state = State::InPropertyValue(index + 1);
            return None;
        }
        Some(Err(LexerError::could_not_find_property_start_symbol(index, character)))
    }  

    fn produce_property_value_result(&mut self, start: usize, index: usize)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        Some(Ok(Token::PropertyValue(self.splice_input(start, index))))
    }

    fn handle_inside_property_value(&mut self, start: usize, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '"' {
            self.state = State::InWhitespace;
            return self.produce_property_value_result(start, index);
        }
        None
    }

    fn end_control_if_possible(&mut self, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '>' {
            self.state = State::Start;
            match self.current_parent.pop() {
                Some(control_name) => return Some(Ok(Token::EndControl(control_name))),
                None => return Some(Err(LexerError::could_not_find_control_to_close(index, character)))
            };
        }
        Some(Err(LexerError::could_not_find_control_close_symbol(index, character)))
    }

    fn end_nested_control_if_possible(&mut self, start: usize, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '>' {
            self.state = State::Start;
            match self.current_parent.pop() {
                Some(control_name) => {
                    let closing_control_name = self.splice_input(start, index);
                    if closing_control_name == control_name {
                        return Some(Ok(Token::EndControl(control_name)))
                    }
                    return Some(Err(LexerError::closing_wrong_tag(index, character)))
                },
                None => return Some(Err(LexerError::could_not_find_control_to_close(index, character)))
            };
        }
        None
    }

    fn handle_inside_whitespace(&mut self, index: usize, character: char)  -> Option<Result<Token<'a>, LexerError<'a>>> {
        if character == '/' {
            self.state = State::EndControl;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
                  
        self.state = State::InProperty(index);
        None
    }    

    fn transition(&mut self, index: usize, character: char) -> Option<Result<Token<'a>, LexerError<'a>>> {
        match self.state {
            State::Start => {
                self.start_if_possible(index, character)
            },
            State::StartControl => {
                self.start_control_if_possible(index, character)
            },
            State::InControl(start) => {
                self.handle_inside_control(start, index, character)
            },
            State::EndControl => {
                self.end_control_if_possible(index, character)
            },
            State::EndNestedControl(start) => {
                self.end_nested_control_if_possible(start, index, character)
            },
            State::InProperty(start) => {
                self.handle_inside_property(start, index, character)
            },
            State::StartPropertyValue => {
                self.start_property_value_if_possible(index, character)
            },
            State::InPropertyValue(start) => {
                self.handle_inside_property_value(start, index, character)
            },
            State::InWhitespace => {
                self.handle_inside_whitespace(index, character)
            }
        }
    }
}

impl <'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError<'a>>;
    fn next(&mut self) -> Option<Result<Token<'a>, LexerError<'a>>> {
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