extern crate zodiac_parsing;
use zodiac_parsing::tuple_lexing::{TupleLexer, TupleLexerError, TupleValue};

#[test]
fn empty_parentheses_produces_no_tuple_values() {
    let mut lexer = TupleLexer::parse("()");
    assert_eq!(None, lexer.next());
}

#[test]
fn no_start_parentheses_produces_error() {
    let mut lexer = TupleLexer::parse("<");
    assert_eq!(Err(TupleLexerError::could_not_find_opening_parentheses(0, '<')), lexer.next().unwrap());
}

#[test]
fn no_end_parentheses_produces_error() {
    let mut lexer = TupleLexer::parse("(>");
    assert_eq!(Err(TupleLexerError::could_not_find_closing_parentheses(1, '>')), lexer.next().unwrap());
}

#[test]
fn single_signed_float_value_produces_single_float_tuple_value() {
    let mut lexer = TupleLexer::parse("(-1.0)");
    assert_eq!(TupleValue::Float(-1.0), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn single_float_value_produces_single_float_tuple_value() {
    let mut lexer = TupleLexer::parse("(1.0)");
    assert_eq!(TupleValue::Float(1.0), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_float_values_produce_multiple_float_tuple_values() {
    let mut lexer = TupleLexer::parse("(1.0,2.0)");
    assert_eq!(TupleValue::Float(1.0), lexer.next().unwrap().unwrap());
    assert_eq!(TupleValue::Float(2.0), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_float_values_with_whitespace_produce_multiple_float_tuple_values() {
    let mut lexer = TupleLexer::parse("(1.0, 2.0)");
    assert_eq!(TupleValue::Float(1.0), lexer.next().unwrap().unwrap());
    assert_eq!(TupleValue::Float(2.0), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn single_signed_int_value_produces_single_int_tuple_value() {
    let mut lexer = TupleLexer::parse("(-10)");
    assert_eq!(TupleValue::Int(-10), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_signed_int_values_produce_multiple_int_tuple_values() {
    let mut lexer = TupleLexer::parse("(-10,-20)");
    assert_eq!(TupleValue::Int(-10), lexer.next().unwrap().unwrap());
    assert_eq!(TupleValue::Int(-20), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_signed_int_values_with_whitespace_produce_multiple_int_tuple_values() {
    let mut lexer = TupleLexer::parse("(-10, -20)");
    assert_eq!(TupleValue::Int(-10), lexer.next().unwrap().unwrap());
    assert_eq!(TupleValue::Int(-20), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn single_int_value_produces_single_int_tuple_value() {
    let mut lexer = TupleLexer::parse("(1)");
    assert_eq!(TupleValue::UnsignedInt(1), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn multiple_int_values_produces_multiple_int_tuple_values() {
    let mut lexer = TupleLexer::parse("(1,2)");
    assert_eq!(TupleValue::UnsignedInt(1), lexer.next().unwrap().unwrap());
    assert_eq!(TupleValue::UnsignedInt(2), lexer.next().unwrap().unwrap());
    assert_eq!(None, lexer.next());
}

#[test]
fn malformed_number_value_produces_error() {
    let mut lexer = TupleLexer::parse("(-1.x)");
    assert_eq!(Err(TupleLexerError::could_not_parse_number_value(5, "-1.x")), lexer.next().unwrap());
}