use std::fmt;

#[derive(Debug, Clone)]
pub enum RpnError {
    InvalidToken(String),
    InsufficientOperands(String),
    DivisionByZero,
    EmptyExpression,
    ParseError(String),
    IoError(String),
}

impl fmt::Display for RpnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpnError::InvalidToken(token) => write!(f, "Invalid token: '{}'. Please use numbers or operators (+, -, *, /)", token),
            RpnError::InsufficientOperands(op) => write!(f, "Insufficient operands for operator '{}'. RPN requires operands before operators (e.g., '3 4 +' not '+ 3 4')", op),
            RpnError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            RpnError::EmptyExpression => write!(f, "Empty expression provided. Please enter a valid RPN expression"),
            RpnError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RpnError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for RpnError {}

impl From<std::num::ParseFloatError> for RpnError {
    fn from(error: std::num::ParseFloatError) -> Self {
        RpnError::ParseError(format!("Invalid number format: {}", error))
    }
}

impl From<std::io::Error> for RpnError {
    fn from(error: std::io::Error) -> Self {
        RpnError::IoError(error.to_string())
    }
}
