extern crate interpreter;
use interpreter::*;

#[test]
fn it_adds_two_one_digit_numbers() {
    let interpreter = Interpreter { text: &"2+3" };
    let result = interpreter.evaluate();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
}

#[test]
fn it_subtructs_two_one_digit_numbers() {
    let interpreter = Interpreter { text: &"3-1" };
    let result = interpreter.evaluate();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn it_returns_error_msg_for_unpermitted_operations() {
    let interpreter = Interpreter { text: &"2w3" };
    let result = interpreter.evaluate();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), OperationError::OperationNotFound('w'));
}

#[test]
fn it_adds_two_multiple_digit_numbers() {
    let interpreter = Interpreter { text: &"22+33" };
    let result = interpreter.evaluate();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 55);
}

#[test]
fn it_skips_white_spaces() {
    let interpreter = Interpreter { text: &"22 + 33 " };
    let result = interpreter.evaluate();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 55);
}

#[test]
fn it_returns_not_a_number() {
    let interpreter = Interpreter { text: &"a2 + 33 " };
    let result = interpreter.evaluate();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), OperationError::NotANumber);
}

#[test]
fn it_treats_spaces_as_the_end_of_each_clouse() {
    let interpreter = Interpreter { text: &"2 2 + 3" };
    let result = interpreter.evaluate();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), OperationError::NotANumber);
}
