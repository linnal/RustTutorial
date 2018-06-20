use std::iter::Peekable;
use std::str::Chars;
mod operator;
use operator::Operator;

#[derive(Debug, PartialEq)]
pub enum OperationError {
    OperationNotFound(char),
    NotANumber
}

pub struct Interpreter<'a> {
    pub text: &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn evaluate(&self) -> Result<i32, OperationError> {
        let mut text_chars: Peekable<Chars> = self.text.chars().peekable();

        let left = self.extract_int(&mut text_chars)?;
        let op = self.extract_op(&mut text_chars);
        let right = self.extract_int(&mut text_chars)?;

        op.eval(left, right)
    }

    fn extract_int(&self, chars: &mut Peekable<Chars>) -> Result<i32, OperationError> {
        let mut number = String::new();
        while chars.peek() != None {
            if chars.peek().unwrap() == &' ' {
                chars.next();
            } else if chars.peek().unwrap().is_digit(10) {
                number.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        number.parse::<i32>().map_err(|_| OperationError::NotANumber)
    }

    fn extract_op(&self, chars: &mut Peekable<Chars>) -> Operator {
        let op = chars.next();
        op.unwrap().into()
    }
}
