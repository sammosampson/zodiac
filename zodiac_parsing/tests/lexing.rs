extern crate zodiac_parsing;
use zodiac_parsing::lexing::{Lexer, LexerError, Token};

#[test]
fn single_control_produces_correct_tokens() {
    let mut lexer = Lexer::parse("<rect/>");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_whitespace_at_end() {
    let mut lexer = Lexer::parse("<rect />");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_carriage_returns_at_end() {
    let mut lexer = Lexer::parse("<rect
    />");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn incorrect_opening_character_produces_error_result() {
    let mut lexer = Lexer::parse("X");
    assert_eq!(Err(LexerError::could_not_find_start_tag(0, 'X')), lexer.next().unwrap());
}

#[test]
fn whiespace_after_token_opening_produces_error_result() {
    let mut lexer = Lexer::parse("< rect/>");
    assert_eq!(Err(LexerError::could_not_find_control_name(1, ' ')), lexer.next().unwrap());
}

#[test]
fn incorrect_closing_character_produces_error_result() {
    let mut lexer = Lexer::parse("<rect/X");
    lexer.next();
    assert_eq!(Err(LexerError::could_not_find_control_close_symbol(6, 'X')), lexer.next().unwrap());
}



#[test]
fn multiple_consecutive_controls_produces_correct_tokens() {
    let mut lexer = Lexer::parse("<rect/><circle/><line/>");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_nested_controls_produces_correct_tokens() {
    let mut lexer = Lexer::parse("<rect><circle><line/></circle></rect>");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_nested_controls_with_properties_produces_correct_tokens() {
    let mut lexer = Lexer::parse("<rect other><circle other></circle></rect>");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn control_with_incorrect_closing_produces_error_result() {
    let mut lexer = Lexer::parse("<rect><line></line></circle>");
    assert_eq!(Token::Control("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), lexer.next().unwrap().unwrap());
    assert_eq!(Err(LexerError::closing_wrong_tag(27, '>')), lexer.next().unwrap());
}

#[test]
fn control_with_incorrect_closing_final_bracket_produces_error_result() {
    let mut lexer = Lexer::parse("<rect></rect/>");
    assert_eq!(Token::Control("rect"),  lexer.next().unwrap().unwrap());
    assert_eq!(Err(LexerError::closing_wrong_tag(13, '>')), lexer.next().unwrap());
}

#[test]
fn property_without_value_produces_boolean_property_result_inside_control() {
    let mut lexer = Lexer::parse("<rect large-size />");
    assert_eq!(Token::Control("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn property_with_value_produces_property_and_value_result_inside_control() {
    let mut lexer = Lexer::parse("<rect large-size=\"true\" />");
    assert_eq!(Token::Control("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue("true"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_properties_without_value_produces_boolean_properties_result_inside_control() {
    let mut lexer = Lexer::parse("<rect large-size rounded-edges other />");
    assert_eq!(Token::Control("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("rounded-edges"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_properties_with_value_produces_properties_result_inside_control() {
    let mut lexer = Lexer::parse("<rect large-size=\"true\" rounded-edges=\"true\" other />");
    assert_eq!(Token::Control("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue("true"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("rounded-edges"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue("true"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), lexer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}