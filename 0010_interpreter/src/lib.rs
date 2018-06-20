use std::iter::Peekable;
use std::str::Chars;

pub struct Interpreter<'a> {
    text: & 'a str,
}

impl <'a> Interpreter<'a> {

    pub fn evaluate(&self) -> Result<i32, String> {
        let mut text_chars: Peekable<Chars> = self.text.chars().peekable();

        let left = self.extract_int(&mut text_chars);
        let op = self.extract_op(&mut text_chars);
        let right = self.extract_int(&mut text_chars);
        match op {
            '+' => Ok(left as i32 + right as i32),
            '-' => Ok(left as i32 - right as i32),
            _ => Err("Operation not found".to_string())
        }
    }

    fn extract_int(&self, chars: &mut Peekable<Chars>) -> i32 {
        let mut number = "".to_string();
        while chars.peek() != None {
            if chars.peek().unwrap() == &' ' {
                chars.next();
            }
            else if chars.peek().unwrap().is_digit(10) {
                number.push(chars.next().unwrap());
            } else {
                break
            }
        }
        number.parse::<i32>().unwrap()
    }

    fn extract_op(&self, chars: &mut Peekable<Chars>) -> char {
        let op = chars.next();
        op.unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result.unwrap_err(), "Operation not found".to_string());
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
}
