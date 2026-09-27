//! Stack visualization using ASCII art

use colored::*;

pub struct StackVisualizer {
    max_width: usize,
}

impl StackVisualizer {
    pub fn new() -> Self {
        Self {
            max_width: 20,
        }
    }

    pub fn render_stack(&self, stack: &[f64]) -> String {
        if stack.is_empty() {
            return self.render_empty_stack();
        }

        let mut result = Vec::new();
        
        // Header
        result.push("┌─ Stack ─┐".bright_blue().to_string());
        
        // Stack items (top to bottom)
        for (i, &value) in stack.iter().rev().enumerate() {
            let formatted_value = self.format_value(value);
            let is_top = i == 0;
            
            if is_top {
                result.push(format!("│ {} │ ← top", formatted_value.green().bold()));
            } else {
                result.push(format!("│ {} │", formatted_value.white()));
            }
            
            // Add separator between items (except for last)
            if i < stack.len() - 1 {
                result.push("├─────────┤".bright_black().to_string());
            }
        }
        
        // Footer
        result.push("└─────────┘".bright_blue().to_string());
        result.push(format!("  {} items", stack.len()).bright_black().to_string());
        
        result.join("\n")
    }

    fn render_empty_stack(&self) -> String {
        vec![
            "┌─ Stack ─┐".bright_blue().to_string(),
            "│  empty  │".yellow().to_string(),
            "└─────────┘".bright_blue().to_string(),
            "  0 items".bright_black().to_string(),
        ].join("\n")
    }

    fn format_value(&self, value: f64) -> String {
        let formatted = if value.fract() == 0.0 && value.abs() < 1e15 {
            format!("{}", value as i64)
        } else {
            let formatted = format!("{:.3}", value);
            if formatted.ends_with("000") {
                formatted.trim_end_matches('0').trim_end_matches('.').to_string()
            } else {
                formatted
            }
        };
        
        // Pad to consistent width
        format!("{:>7}", formatted)
    }

    pub fn render_stack_horizontal(&self, stack: &[f64]) -> String {
        if stack.is_empty() {
            return "Stack: [ empty ]".yellow().to_string();
        }

        let items: Vec<String> = stack.iter()
            .map(|&val| self.format_value(val).trim().to_string())
            .collect();
        
        format!("Stack: [ {} ]", items.join(" │ ").white())
    }

    pub fn render_stack_operations(&self, stack: &[f64]) -> String {
        if stack.is_empty() {
            return String::new();
        }

        let mut operations = Vec::new();
        
        match stack.len() {
            1 => {
                operations.push("📋 Available: sqrt, sin, cos, tan, abs, floor, ceil, round".bright_green().to_string());
            }
            2 => {
                operations.push("🔢 Available: +, -, *, /, ^, %".bright_green().to_string());
                operations.push("📋 Also: sqrt, sin, cos, tan, abs, floor, ceil, round (on top)".bright_blue().to_string());
            }
            _ => {
                operations.push("🔢 Available: +, -, *, /, ^, % (uses top 2)".bright_green().to_string());
                operations.push("📋 Also: sqrt, sin, cos, tan, abs, floor, ceil, round (on top)".bright_blue().to_string());
            }
        }
        
        operations.join("\n")
    }
}

impl Default for StackVisualizer {
    fn default() -> Self {
        Self::new()
    }
}