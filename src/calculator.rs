use crate::error::RpnError;
use crate::parser::{Token, parse_expression};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calculator {
    stack: Vec<f64>,
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            stack: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn new_with_history() -> Self {
        match Self::load_history() {
            Ok(mut calc) => {
                calc.stack.clear(); // Always start with empty stack
                calc
            }
            Err(_) => Self::new(),
        }
    }

    fn get_history_file_path() -> Result<PathBuf, RpnError> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| RpnError::IoError("Cannot determine data directory".to_string()))?;
        path.push("rpn-calculator");
        
        // Create directory if it doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path)
                .map_err(|e| RpnError::IoError(format!("Cannot create data directory: {}", e)))?;
        }
        
        path.push("history.json");
        Ok(path)
    }

    fn load_history() -> Result<Calculator, RpnError> {
        let path = Self::get_history_file_path()?;
        
        if !path.exists() {
            return Ok(Calculator::new());
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| RpnError::IoError(format!("Cannot read history file: {}", e)))?;
        
        let calc: Calculator = serde_json::from_str(&content)
            .map_err(|e| RpnError::ParseError(format!("Cannot parse history file: {}", e)))?;
        
        Ok(calc)
    }

    pub fn save_history(&self) -> Result<(), RpnError> {
        let path = Self::get_history_file_path()?;
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| RpnError::ParseError(format!("Cannot serialize history: {}", e)))?;
        
        fs::write(&path, content)
            .map_err(|e| RpnError::IoError(format!("Cannot write history file: {}", e)))?;
        
        Ok(())
    }

    pub fn evaluate(&mut self, expression: &str) -> Result<f64, RpnError> {
        let tokens = parse_expression(expression)?;
        
        // Clear the stack for each new expression
        self.stack.clear();
        
        for token in tokens {
            match token {
                Token::Number(num) => {
                    self.stack.push(num);
                }
                Token::Operator(op) => {
                    if op.is_unary() {
                        if self.stack.is_empty() {
                            return Err(RpnError::InsufficientOperands(op.symbol().to_string()));
                        }
                        let operand = self.stack.pop().unwrap();
                        let result = op.apply_unary(operand)?;
                        self.stack.push(result);
                    } else {
                        if self.stack.len() < 2 {
                            return Err(RpnError::InsufficientOperands(op.symbol().to_string()));
                        }
                        
                        // Pop two operands (note: right operand is popped first)
                        let right = self.stack.pop().unwrap();
                        let left = self.stack.pop().unwrap();
                        
                        let result = op.apply(left, right)?;
                        self.stack.push(result);
                    }
                }
            }
        }

        if self.stack.len() != 1 {
            return Err(RpnError::ParseError(
                "Invalid expression: too many operands or insufficient operators".to_string()
            ));
        }

        let result = self.stack[0];
        
        // Add to history
        self.history.push(format!("{} = {}", expression, result));
        
        // Auto-save history after each calculation
        let _ = self.save_history(); // Ignore errors for non-critical operation
        
        Ok(result)
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        let _ = self.save_history(); // Save cleared state
    }

    pub fn get_stack(&self) -> &[f64] {
        &self.stack
    }

    /// Evaluate expression and return formatted result
    pub fn evaluate_and_format(&mut self, expression: &str) -> String {
        match self.evaluate(expression) {
            Ok(result) => {
                if result.fract() == 0.0 && result.abs() < 1e15 {
                    // Display as integer if it's a whole number
                    format!("{}", result as i64)
                } else {
                    // Display as float, removing trailing zeros
                    let formatted = format!("{}", result);
                    if formatted.contains('.') && formatted.ends_with('0') {
                        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
                    } else {
                        formatted
                    }
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut calc = Calculator::new();
        
        assert_eq!(calc.evaluate("3 4 +").unwrap(), 7.0);
        assert_eq!(calc.evaluate("10 5 -").unwrap(), 5.0);
        assert_eq!(calc.evaluate("6 7 *").unwrap(), 42.0);
        assert_eq!(calc.evaluate("15 3 /").unwrap(), 5.0);
    }

    #[test]
    fn test_advanced_operations() {
        let mut calc = Calculator::new();
        
        assert_eq!(calc.evaluate("2 3 ^").unwrap(), 8.0);
        assert_eq!(calc.evaluate("25 sqrt").unwrap(), 5.0);
        assert_eq!(calc.evaluate("-5 abs").unwrap(), 5.0);
        assert_eq!(calc.evaluate("3.7 floor").unwrap(), 3.0);
        assert_eq!(calc.evaluate("3.2 ceil").unwrap(), 4.0);
        assert_eq!(calc.evaluate("17 5 %").unwrap(), 2.0);
    }

    #[test]
    fn test_trigonometric_functions() {
        let mut calc = Calculator::new();
        
        let result = calc.evaluate("0 sin").unwrap();
        assert!((result - 0.0).abs() < 1e-10);
        
        let result = calc.evaluate("0 cos").unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_expression() {
        let mut calc = Calculator::new();
        // (3 + 4) * 2 = 14 in RPN: 3 4 + 2 *
        assert_eq!(calc.evaluate("3 4 + 2 *").unwrap(), 14.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new();
        let result = calc.evaluate("5 0 /");
        assert!(result.is_err());
    }

    #[test]
    fn test_insufficient_operands() {
        let mut calc = Calculator::new();
        let result = calc.evaluate("5 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_history() {
        let mut calc = Calculator::new();
        calc.evaluate("3 4 +").unwrap();
        calc.evaluate("10 5 -").unwrap();
        
        let history = calc.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "3 4 + = 7");
        assert_eq!(history[1], "10 5 - = 5");
    }
}
