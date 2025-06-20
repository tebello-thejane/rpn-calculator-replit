use crate::error::RpnError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn from_str(s: &str) -> Result<Self, RpnError> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(RpnError::InvalidToken(s.to_string())),
        }
    }

    pub fn apply(&self, left: f64, right: f64) -> Result<f64, RpnError> {
        match self {
            Operator::Add => Ok(left + right),
            Operator::Subtract => Ok(left - right),
            Operator::Multiply => Ok(left * right),
            Operator::Divide => {
                if right == 0.0 {
                    Err(RpnError::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

pub fn parse_expression(input: &str) -> Result<Vec<Token>, RpnError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(RpnError::EmptyExpression);
    }

    let mut tokens = Vec::new();
    
    for token_str in input.split_whitespace() {
        if let Ok(number) = token_str.parse::<f64>() {
            tokens.push(Token::Number(number));
        } else if let Ok(operator) = Operator::from_str(token_str) {
            tokens.push(Token::Operator(operator));
        } else {
            return Err(RpnError::InvalidToken(token_str.to_string()));
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let tokens = parse_expression("3 4 +").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(3.0));
        assert_eq!(tokens[1], Token::Number(4.0));
        assert_eq!(tokens[2], Token::Operator(Operator::Add));
    }

    #[test]
    fn test_parse_invalid_token() {
        let result = parse_expression("3 4 invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_expression() {
        let result = parse_expression("");
        assert!(result.is_err());
    }
}
