extern crate zodiac_parsing;
use zodiac_parsing::source_tokenization::{SourceTokenizer, SourceTokenError, Token, SourceTokenPropertyValue};

#[test]
fn single_control_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/>");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_whitespace_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect />");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_carriage_returns_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect
    />");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn incorrect_opening_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("X");
    assert_eq!(Err(SourceTokenError::could_not_find_start_tag(0, 'X')), tokenizer.next().unwrap());
}

#[test]
fn whitespace_after_token_opening_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("< rect/>");
    assert_eq!(Err(SourceTokenError::could_not_find_control_name(1, ' ')), tokenizer.next().unwrap());
}

#[test]
fn incorrect_closing_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/X");
    tokenizer.next();
    assert_eq!(Err(SourceTokenError::could_not_find_control_close_symbol(6, 'X')), tokenizer.next().unwrap());
}

#[test]
fn multiple_consecutive_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/><circle/><line/>");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect><circle><line/></circle></rect>");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_with_properties_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect other><circle other></circle></rect>");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("circle"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("circle"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn control_with_incorrect_closing_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect><line></line></circle>");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Control("line"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("line"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::closing_wrong_tag(27, '>')), tokenizer.next().unwrap());
}

#[test]
fn control_with_incorrect_closing_final_bracket_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect></rect/>");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::closing_wrong_tag(13, '>')), tokenizer.next().unwrap());
}

#[test]
fn property_without_value_produces_boolean_property_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_string_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::String("large")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_unsigned_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=10 />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::UnsignedInt(10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_number_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1
    />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::UnsignedInt(1)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10 />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
#[test]
fn property_with_int_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10
    />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_negative_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1.0 />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::Float(-1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_positive_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1.0 />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::Float(1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_tuple_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=(1.0, 1.0) />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::Tuple("(1.0, 1.0)")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_incorrect_unsigned_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1x />");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::could_not_parse_number_value(13, "1x")), tokenizer.next().unwrap());
}

#[test]
fn property_with_incorrect_signed_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1x />");
    assert_eq!(Token::Control("rect"),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::could_not_parse_number_value(14, "-1x")), tokenizer.next().unwrap());
}

#[test]
fn multiple_properties_without_value_produces_boolean_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size rounded-edges other />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("large-size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("rounded-edges"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_properties_with_value_produces_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" edges=\"round\" other />");
    assert_eq!(Token::Control("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("size"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::String("large")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("edges"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::PropertyValue(SourceTokenPropertyValue::String("round")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::Property("other"), tokenizer.next().unwrap().unwrap());
    assert_eq!(Token::EndControl("rect"), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}