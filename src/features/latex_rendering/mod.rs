//! Advanced terminal LaTeX rendering with proper mathematical notation

use colored::*;
use crate::parser::{parse_expression, Token, Operator};

pub struct LatexRenderer {
    use_box_drawing: bool,
    use_unicode_math: bool,
}

impl LatexRenderer {
    pub fn new() -> Self {
        Self {
            use_box_drawing: true,
            use_unicode_math: true,
        }
    }

    pub fn convert_rpn_to_latex(&self, expression: &str) -> String {
        match self.rpn_to_rendered_math(expression) {
            Ok(rendered) => rendered,
            Err(_) => format!("Expression: {}", expression.bright_white()),
        }
    }

    fn rpn_to_rendered_math(&self, expression: &str) -> Result<String, Box<dyn std::error::Error>> {
        let tokens = parse_expression(expression)?;
        let mut stack: Vec<MathNode> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(num) => {
                    stack.push(MathNode::Number(num));
                }
                Token::Operator(op) => {
                    if op.is_unary() {
                        if stack.is_empty() {
                            return Err("Insufficient operands".into());
                        }
                        let operand = stack.pop().unwrap();
                        let result = MathNode::UnaryOp(op, Box::new(operand));
                        stack.push(result);
                    } else {
                        if stack.len() < 2 {
                            return Err("Insufficient operands".into());
                        }
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        let result = MathNode::BinaryOp(op, Box::new(left), Box::new(right));
                        stack.push(result);
                    }
                }
            }
        }

        if stack.len() != 1 {
            return Err("Invalid expression".into());
        }

        let math_tree = stack.pop().unwrap();
        Ok(self.render_math_node(&math_tree))
    }

    fn render_math_node(&self, node: &MathNode) -> String {
        match node {
            MathNode::Number(num) => self.format_number(*num),
            MathNode::UnaryOp(op, operand) => self.render_unary_operation(op, operand),
            MathNode::BinaryOp(op, left, right) => self.render_binary_operation(op, left, right),
        }
    }

    fn format_number(&self, num: f64) -> String {
        if num.fract() == 0.0 && num.abs() < 1e15 {
            format!("{}", num as i64).bright_cyan().to_string()
        } else if num < 0.0 {
            format!("{:.3}", num).trim_end_matches('0').trim_end_matches('.').bright_cyan().to_string()
        } else {
            format!("{:.3}", num).trim_end_matches('0').trim_end_matches('.').bright_cyan().to_string()
        }
    }

    fn render_binary_operation(&self, op: &Operator, left: &MathNode, right: &MathNode) -> String {
        match op {
            Operator::Add => {
                format!("{} {} {}", 
                    self.render_math_node(left),
                    "+".bright_green().bold(),
                    self.render_math_node(right))
            }
            Operator::Subtract => {
                format!("{} {} {}", 
                    self.render_math_node(left),
                    "−".bright_red().bold(),  // Unicode minus
                    self.render_math_node(right))
            }
            Operator::Multiply => {
                format!("{} {} {}", 
                    self.render_math_node(left),
                    "×".bright_yellow().bold(),
                    self.render_math_node(right))
            }
            Operator::Divide => {
                self.render_fraction(left, right)
            }
            Operator::Power => {
                self.render_exponent(left, right)
            }
            Operator::Modulo => {
                format!("{} {} {}", 
                    self.render_math_node(left),
                    "mod".bright_magenta().bold(),
                    self.render_math_node(right))
            }
            _ => format!("{}({}, {})", op.symbol(), self.render_math_node(left), self.render_math_node(right)),
        }
    }

    fn render_unary_operation(&self, op: &Operator, operand: &MathNode) -> String {
        match op {
            Operator::Sqrt => {
                self.render_square_root(operand)
            }
            Operator::Sin => {
                format!("{}({})", "sin".bright_blue().bold(), self.render_math_node(operand))
            }
            Operator::Cos => {
                format!("{}({})", "cos".bright_blue().bold(), self.render_math_node(operand))
            }
            Operator::Tan => {
                format!("{}({})", "tan".bright_blue().bold(), self.render_math_node(operand))
            }
            Operator::Log => {
                format!("{}({})", "log".bright_purple().bold(), self.render_math_node(operand))
            }
            Operator::Ln => {
                format!("{}({})", "ln".bright_purple().bold(), self.render_math_node(operand))
            }
            Operator::Abs => {
                format!("│{}│", self.render_math_node(operand).bright_white())
            }
            Operator::Floor => {
                format!("⌊{}⌋", self.render_math_node(operand).bright_white())
            }
            Operator::Ceil => {
                format!("⌈{}⌉", self.render_math_node(operand).bright_white())
            }
            Operator::Round => {
                format!("{}({})", "round".bright_white().bold(), self.render_math_node(operand))
            }
            _ => format!("{}({})", op.symbol(), self.render_math_node(operand)),
        }
    }

    fn render_fraction(&self, numerator: &MathNode, denominator: &MathNode) -> String {
        let num_str = self.render_math_node(numerator);
        let den_str = self.render_math_node(denominator);
        
        // Calculate the width needed for the fraction
        let num_width = self.display_width(&num_str);
        let den_width = self.display_width(&den_str);
        let max_width = std::cmp::max(num_width, den_width);
        
        if !self.use_box_drawing || max_width > 20 {
            // Fallback to inline notation for very wide expressions
            return format!("({}) {} ({})", num_str, "÷".bright_yellow().bold(), den_str);
        }

        // Create fraction with proper alignment
        let num_padded = self.center_text(&num_str, max_width);
        let den_padded = self.center_text(&den_str, max_width);
        let line = "─".repeat(max_width);

        format!("{}\n{}\n{}", num_padded, line.bright_white(), den_padded)
    }

    fn render_square_root(&self, operand: &MathNode) -> String {
        let operand_str = self.render_math_node(operand);
        
        if !self.use_box_drawing {
            return format!("√{}", operand_str);
        }

        let operand_width = self.display_width(&operand_str);
        
        if operand_width <= 1 {
            // Simple single character
            format!("√{}", operand_str)
        } else if operand_width <= 8 {
            // Use Unicode radical
            let top_line = "‾".repeat(operand_width);
            format!("√{}\n {}", top_line.bright_white(), operand_str)
        } else {
            // Fallback for very wide expressions
            format!("√({})", operand_str)
        }
    }

    fn render_exponent(&self, base: &MathNode, exp: &MathNode) -> String {
        let base_str = self.render_math_node(base);
        let exp_str = self.render_math_node(exp);
        
        // Try to use Unicode superscripts for simple exponents
        if let Some(unicode_exp) = self.to_unicode_superscript(&exp_str) {
            format!("{}{}", base_str, unicode_exp.bright_yellow())
        } else {
            format!("{}^({})", base_str, exp_str.bright_yellow())
        }
    }

    fn to_unicode_superscript(&self, text: &str) -> Option<String> {
        // Remove ANSI color codes for checking
        let clean_text = self.strip_ansi(text);
        
        if clean_text.len() > 3 || !clean_text.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return None;
        }

        let result: String = clean_text.chars().map(|c| match c {
            '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴',
            '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
            '-' => '⁻',
            _ => c,
        }).collect();

        Some(result)
    }

    fn center_text(&self, text: &str, width: usize) -> String {
        let text_width = self.display_width(text);
        if text_width >= width {
            return text.to_string();
        }
        
        let padding = width - text_width;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        
        format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
    }

    fn display_width(&self, text: &str) -> usize {
        // Approximate display width, accounting for ANSI escape sequences
        self.strip_ansi(text).chars().count()
    }

    fn strip_ansi(&self, text: &str) -> String {
        // Simple ANSI stripping (not perfect but sufficient for our use)
        let mut result = String::new();
        let mut in_escape = false;
        
        for c in text.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape && c == 'm' {
                in_escape = false;
            } else if !in_escape {
                result.push(c);
            }
        }
        result
    }

    pub fn render_equation_box(&self, expression: &str, result: f64) -> String {
        let math_display = self.convert_rpn_to_latex(expression);
        let result_str = self.format_number(result);
        
        // Calculate box width
        let math_width = self.display_width(&math_display);
        let result_width = self.display_width(&result_str) + 2; // "+ = "
        let content_width = std::cmp::max(math_width, result_width);
        let box_width = content_width + 4; // padding
        
        let top_line = format!("┌{}┐", "─".repeat(box_width - 2));
        let bottom_line = format!("└{}┘", "─".repeat(box_width - 2));
        let separator = format!("├{}┤", "─".repeat(box_width - 2));
        
        let math_line = format!("│ {} │", self.center_text(&math_display, content_width));
        let result_line = format!("│ {} {} │", "=".bright_white().bold(), self.center_text(&result_str, content_width - 2));
        
        vec![
            top_line.bright_blue().to_string(),
            math_line,
            separator.bright_blue().to_string(),
            result_line,
            bottom_line.bright_blue().to_string(),
        ].join("\n")
    }

    pub fn get_mathematical_symbols(&self) -> String {
        vec![
            "🧮 Mathematical Symbols Available:".bright_blue().bold().to_string(),
            format!("   Operators: {} {} {} {} {} {}", 
                "+".bright_green(), "−".bright_red(), "×".bright_yellow(), 
                "÷".bright_yellow(), "^".bright_yellow(), "mod".bright_magenta()),
            format!("   Functions: {} {} {} √ {} {}", 
                "sin".bright_blue(), "cos".bright_blue(), "tan".bright_blue(),
                "log".bright_purple(), "ln".bright_purple()),
            format!("   Brackets: {} {} {} {} {} {}", 
                "│ │".bright_white(), "⌊ ⌋".bright_white(), "⌈ ⌉".bright_white(),
                "( )".bright_white(), "[ ]".bright_white(), "{ }".bright_white()),
            format!("   Superscripts: {}", "⁰¹²³⁴⁵⁶⁷⁸⁹⁻".bright_yellow()),
            format!("   Constants: {} {} {} {}", 
                "π".bright_cyan(), "e".bright_cyan(), "∞".bright_red(), "∅".bright_black()),
        ].join("\n")
    }
}

#[derive(Debug, Clone)]
enum MathNode {
    Number(f64),
    UnaryOp(Operator, Box<MathNode>),
    BinaryOp(Operator, Box<MathNode>, Box<MathNode>),
}

impl Default for LatexRenderer {
    fn default() -> Self {
        Self::new()
    }
}