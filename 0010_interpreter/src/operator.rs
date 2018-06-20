use super::OperationError;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Unknown(char),
}

impl From<char> for Operator {
    fn from(ch: char) -> Self {
        match ch {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            c => Operator::Unknown(c),
        }
    }
}

impl Operator {
    pub fn eval(&self, left: i32, right: i32) -> Result<i32, OperationError> {
        match self {
            Operator::Plus => Ok(left as i32 + right as i32),
            Operator::Minus => Ok(left as i32 - right as i32),
            Operator::Unknown(op) => Err(OperationError::OperationNotFound(*op)),
        }
    }
}
