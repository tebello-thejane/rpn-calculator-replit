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
    Power,
    Modulo,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Abs,
    Floor,
    Ceil,
    Round,
}

impl Operator {
    pub fn from_str(s: &str) -> Result<Self, RpnError> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "^" | "pow" => Ok(Operator::Power),
            "%" | "mod" => Ok(Operator::Modulo),
            "sqrt" => Ok(Operator::Sqrt),
            "sin" => Ok(Operator::Sin),
            "cos" => Ok(Operator::Cos),
            "tan" => Ok(Operator::Tan),
            "log" => Ok(Operator::Log),
            "ln" => Ok(Operator::Ln),
            "abs" => Ok(Operator::Abs),
            "floor" => Ok(Operator::Floor),
            "ceil" => Ok(Operator::Ceil),
            "round" => Ok(Operator::Round),
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
            Operator::Power => Ok(left.powf(right)),
            Operator::Modulo => {
                if right == 0.0 {
                    Err(RpnError::DivisionByZero)
                } else {
                    Ok(left % right)
                }
            }
            _ => Err(RpnError::InvalidToken(format!("'{}' is not a binary operator", self.symbol()))),
        }
    }

    pub fn apply_unary(&self, operand: f64) -> Result<f64, RpnError> {
        match self {
            Operator::Sqrt => {
                if operand < 0.0 {
                    Err(RpnError::ParseError("Square root of negative number".to_string()))
                } else {
                    Ok(operand.sqrt())
                }
            }
            Operator::Sin => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Sine of infinite or NaN value".to_string()))
                } else {
                    Ok(operand.sin())
                }
            }
            Operator::Cos => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Cosine of infinite or NaN value".to_string()))
                } else {
                    Ok(operand.cos())
                }
            }
            Operator::Tan => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Tangent of infinite or NaN value".to_string()))
                } else {
                    let result = operand.tan();
                    if result.is_infinite() {
                        Err(RpnError::ParseError("Tangent result is infinite (near π/2 + nπ)".to_string()))
                    } else {
                        Ok(result)
                    }
                }
            }
            Operator::Log => {
                if operand <= 0.0 {
                    Err(RpnError::ParseError("Logarithm of non-positive number".to_string()))
                } else if operand.is_infinite() {
                    Ok(f64::INFINITY)
                } else {
                    Ok(operand.log10())
                }
            }
            Operator::Ln => {
                if operand <= 0.0 {
                    Err(RpnError::ParseError("Natural logarithm of non-positive number".to_string()))
                } else if operand.is_infinite() {
                    Ok(f64::INFINITY)
                } else {
                    Ok(operand.ln())
                }
            }
            Operator::Abs => {
                if operand.is_nan() {
                    Err(RpnError::ParseError("Absolute value of NaN".to_string()))
                } else {
                    Ok(operand.abs())
                }
            }
            Operator::Floor => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Floor of infinite or NaN value".to_string()))
                } else {
                    Ok(operand.floor())
                }
            }
            Operator::Ceil => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Ceiling of infinite or NaN value".to_string()))
                } else {
                    Ok(operand.ceil())
                }
            }
            Operator::Round => {
                if operand.is_infinite() || operand.is_nan() {
                    Err(RpnError::ParseError("Round of infinite or NaN value".to_string()))
                } else {
                    Ok(operand.round())
                }
            }
            _ => Err(RpnError::InvalidToken(format!("'{}' is not a unary operator", self.symbol()))),
        }
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, 
            Operator::Sqrt | Operator::Sin | Operator::Cos | Operator::Tan | 
            Operator::Log | Operator::Ln | Operator::Abs | Operator::Floor | 
            Operator::Ceil | Operator::Round
        )
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
            Operator::Modulo => "%",
            Operator::Sqrt => "sqrt",
            Operator::Sin => "sin",
            Operator::Cos => "cos",
            Operator::Tan => "tan",
            Operator::Log => "log",
            Operator::Ln => "ln",
            Operator::Abs => "abs",
            Operator::Floor => "floor",
            Operator::Ceil => "ceil",
            Operator::Round => "round",
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
    fn test_parse_advanced_operators() {
        let tokens = parse_expression("25 sqrt").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Number(25.0));
        assert_eq!(tokens[1], Token::Operator(Operator::Sqrt));
    }

    #[test]
    fn test_parse_power_operator() {
        let tokens = parse_expression("2 3 pow").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2], Token::Operator(Operator::Power));
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
