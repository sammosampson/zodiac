use std::vec;
use std::iter::Enumerate;
use std::str::Chars;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceTokenPropertyValue<'a> {
    String(&'a str),
    Int(i128),
    UnsignedInt(u128),
    Float(f64),
    Tuple(&'a str)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SourceToken<'a> {
    Control(&'a str),
    EndControl(&'a str),
    Property(&'a str),
    PropertyValue(SourceTokenPropertyValue<'a>),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum SourceTokenError<'a> {
    ControlError(&'a str, usize, char),
    PropertyError(&'a str, usize, char),
    ValueError(&'a str, usize, &'a str)
}

impl<'a> SourceTokenError<'a> {
    pub fn could_not_find_start_tag(index: usize, character: char) -> Self {
        SourceTokenError::ControlError("could not find control start tag (<)", index, character)
    }

    pub fn could_not_find_control_name(index: usize, character: char) -> Self {
        SourceTokenError::ControlError("could not find control name", index, character)
    }

    pub fn could_not_find_property_start_symbol(index: usize, character: char) -> Self {
        SourceTokenError::PropertyError("could not find property start symbol (\")", index, character)
    }

    pub fn could_not_find_control_close_symbol(index: usize, character: char) -> Self {
        SourceTokenError::ControlError("could not find control close symbol (>)", index, character)
    }

    pub fn closing_wrong_tag(index: usize, character: char) -> Self {
        SourceTokenError::ControlError("trying to close the wrong control", index, character)
    }

    pub fn could_not_find_control_to_close(index: usize, character: char) -> Self {
        SourceTokenError::ControlError("could not find control to close", index, character)
    }

    pub fn could_not_parse_number_value(index: usize, value: &'a str) -> Self {
        SourceTokenError::ValueError("could not parse number value", index, value)
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
    InStringPropertyValue(usize),
    InUnsignedNumberPropertyValue(usize),
    InSignedNumberPropertyValue(usize),
    InTuplePropertyValue(usize),
    StartPropertyValue,
    InWhitespace
} 

pub type SourceTokenResult<'a> = Result<SourceToken<'a>, SourceTokenError<'a>>;
pub type SourceTokenOption<'a> = Option<SourceTokenResult<'a>>;

pub struct SourceTokenizer<'a> {
    input: &'a str,
    characters: Enumerate<Chars<'a>>,
    current_parent: Vec<&'a str>,
    state: State
}

impl <'a> Iterator for SourceTokenizer<'a> {
    type Item = SourceTokenResult<'a>;
    fn next(&mut self) -> SourceTokenOption<'a> {
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

impl<'a> SourceTokenizer<'a> {
    pub fn from_string(input: &'a str) -> Self {
        Self {
            input,
            characters: input.chars().enumerate(),
            state: State::Start,
            current_parent: vec![]
        }
    }

    fn transition(&mut self, index: usize, character: char) -> SourceTokenOption<'a> {
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
            State::InStringPropertyValue(start) => {
                self.handle_inside_string_property_value(start, index, character)
            },
            State::InUnsignedNumberPropertyValue(start) => {
                self.handle_inside_unsigned_number_property_value(start, index, character)
            },
            State::InSignedNumberPropertyValue(start) => {
                self.handle_inside_signed_number_property_value(start, index, character)
            },
            State::InTuplePropertyValue(start) => {
                self.handle_inside_tuple_property_value(start, index, character)
            },
            State::InWhitespace => {
                self.handle_inside_whitespace(index, character)
            }
        }
    }

    fn splice_input(&mut self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    fn start_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '<' {
            self.state = State::StartControl;
            return None;
        }
        if character.is_whitespace() {
            return None;
        }
        Some(Err(SourceTokenError::could_not_find_start_tag(index, character)))
    }
    
    fn start_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '/' {
            self.state = State::EndNestedControl(index + 1);
            return None;
        }
        if !character.is_whitespace() {
            self.state = State::InControl(index);
            return None;
        }
        Some(Err(SourceTokenError::could_not_find_control_name(index, character)))
    }

    fn produce_control_result(&mut self, start: usize, index: usize)  -> SourceTokenOption<'a> {
        let control_name = self.splice_input(start, index);
        self.current_parent.push(control_name);
        Some(Ok(SourceToken::Control(control_name)))
    }

    fn handle_inside_control(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
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

    fn produce_property_result(&mut self, start: usize, index: usize)  -> SourceTokenOption<'a> {
        Some(Ok(SourceToken::Property(self.splice_input(start, index))))
    }

    fn handle_inside_property(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
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
    
    fn start_property_value_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '"' {
            self.state = State::InStringPropertyValue(index + 1);
            return None;
        }
        if character.is_numeric() {
            self.state = State::InUnsignedNumberPropertyValue(index);
            return None;
        }
        if character == '-' {
            self.state = State::InSignedNumberPropertyValue(index);
            return None;
        }
        if character == '(' {
            self.state = State::InTuplePropertyValue(index);
            return None;
        }
        Some(Err(SourceTokenError::could_not_find_property_start_symbol(index, character)))
    }  

    fn produce_string_property_value_result(&mut self, start: usize, index: usize)  -> SourceTokenOption<'a> {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::String(self.splice_input(start, index)))))
    }

    fn produce_unsigned_number_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption<'a> {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<u128>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(value)))),
            Err(_) => return self.produce_float_property_value_result(raw_value, index)
        }
    }

    fn produce_signed_number_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption<'a> {
        let raw_value = self.splice_input(start, index);
        match raw_value.parse::<i128>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(value)))),
            Err(_) => return self.produce_float_property_value_result(raw_value, index)
        }
    }

    fn produce_float_property_value_result(&mut self, raw_value: &'a str, index: usize) -> SourceTokenOption<'a> {
        match raw_value.parse::<f64>() {
            Ok(value) => return Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(value)))),
            Err(_) => return Some(Err(SourceTokenError::could_not_parse_number_value(index, raw_value)))
        }
    }

    fn produce_tuple_property_value_result(&mut self, start: usize, index: usize) -> SourceTokenOption<'a> {
        Some(Ok(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(self.splice_input(start, index)))))
    }
    
    fn handle_inside_string_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '"' {
            self.state = State::InWhitespace;
            return self.produce_string_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_unsigned_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == ' ' || character.is_whitespace() {
            self.state = State::InWhitespace;
            return self.produce_unsigned_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_signed_number_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == ' ' || character.is_whitespace()  {
            self.state = State::InWhitespace;
            return self.produce_signed_number_property_value_result(start, index);
        }
        None
    }

    fn handle_inside_tuple_property_value(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == ')' {
            self.state = State::InWhitespace;
            return self.produce_tuple_property_value_result(start, index + 1);
        }
        None
    }

    fn end_control_if_possible(&mut self, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '>' {
            self.state = State::Start;
            match self.current_parent.pop() {
                Some(control_name) => return Some(Ok(SourceToken::EndControl(control_name))),
                None => return Some(Err(SourceTokenError::could_not_find_control_to_close(index, character)))
            };
        }
        Some(Err(SourceTokenError::could_not_find_control_close_symbol(index, character)))
    }

    fn end_nested_control_if_possible(&mut self, start: usize, index: usize, character: char)  -> SourceTokenOption<'a> {
        if character == '>' {
            self.state = State::Start;
            match self.current_parent.pop() {
                Some(control_name) => {
                    let closing_control_name = self.splice_input(start, index);
                    if closing_control_name == control_name {
                        return Some(Ok(SourceToken::EndControl(control_name)))
                    }
                    return Some(Err(SourceTokenError::closing_wrong_tag(index, character)))
                },
                None => return Some(Err(SourceTokenError::could_not_find_control_to_close(index, character)))
            };
        }
        None
    }

    fn handle_inside_whitespace(&mut self, index: usize, character: char)  -> SourceTokenOption<'a> {
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
}