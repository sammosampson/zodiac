use zodiac_source::tokenization::source::*;

#[test]
fn single_control_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_whitespace_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn single_control_produces_correct_tokens_with_carriage_returns_at_end() {
    let mut tokenizer = SourceTokenizer::from_string("<rect
    />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn incorrect_opening_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("X");
    assert_eq!(Err(SourceTokenError::CouldNotFindStartTag(0)), tokenizer.next().unwrap());
}

#[test]
fn whitespace_after_token_opening_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("< rect/>");
    assert_eq!(Err(SourceTokenError::CouldNotFindControlName(1)), tokenizer.next().unwrap());
}

#[test]
fn incorrect_closing_character_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/X");
    tokenizer.next();
    assert_eq!(Err(SourceTokenError::CouldNotFindControlCloseSymbol(6)), tokenizer.next().unwrap());
}

#[test]
fn multiple_consecutive_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<rect/><circle/><line/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas><circle><line/></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_with_valueless_properties_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas other><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_nested_controls_with_properties_with_values_produces_correct_tokens() {
    let mut tokenizer = SourceTokenizer::from_string("<canvas offset=(200, 100)><circle other></circle></canvas>");
    assert_eq!(SourceToken::Control(String::from("canvas")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("offset")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(String::from("(200, 100)"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("circle")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("circle")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("canvas")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn control_with_incorrect_closing_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect><line></line></circle>");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Control(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("line")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::ClosingWrongTag(27)), tokenizer.next().unwrap());
}

#[test]
fn control_with_incorrect_closing_final_bracket_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect></rect/>");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::ClosingWrongTag(13)), tokenizer.next().unwrap());
}

#[test]
fn property_without_value_produces_boolean_property_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_string_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("large"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_unsigned_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_number_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::UnsignedInt(1)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_int_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}
#[test]
fn property_with_int_value_followed_by_carriage_return_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-10
    />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Int(-10)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_negative_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1.0 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(-1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_positive_float_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1.0 />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Float(1.0)), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_tuple_value_produces_property_and_value_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=(1.0, 1.0) />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::Tuple(String::from("(1.0, 1.0)"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn property_with_incorrect_unsigned_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(13)), tokenizer.next().unwrap());
}

#[test]
fn property_with_incorrect_signed_number_value_produces_error_result() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=-1x />");
    assert_eq!(SourceToken::Control(String::from("rect")),  tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(Err(SourceTokenError::CouldNotParseNumberValue(14)), tokenizer.next().unwrap());
}

#[test]
fn multiple_properties_without_value_produces_boolean_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect large-size rounded-edges other />");
    assert_eq!(SourceToken::Control(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("large-size")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("rounded-edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("other")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from("rect")), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}

#[test]
fn multiple_properties_with_value_produces_properties_result_inside_control() {
    let mut tokenizer = SourceTokenizer::from_string("<rect size=\"large\" edges=\"round\" other />");
    assert_eq!(SourceToken::Control(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from(String::from("size"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from(String::from("large")))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from("edges")), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::PropertyValue(SourceTokenPropertyValue::String(String::from("round"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::Property(String::from(String::from("other"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(SourceToken::EndControl(String::from(String::from("rect"))), tokenizer.next().unwrap().unwrap());
    assert_eq!(None, tokenizer.next());
}