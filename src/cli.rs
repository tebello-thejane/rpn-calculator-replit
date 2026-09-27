use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::calculator::Calculator;
use crate::error::RpnError;
use crate::features::FeatureManager;
use colored::*;

#[derive(Parser)]
#[command(name = "rpn-calculator")]
#[command(about = "A feature-rich RPN (Reverse Polish Notation) calculator")]
#[command(version = "0.1.0")]
pub struct Args {
    /// RPN expression to evaluate (if not provided, enters interactive mode)
    #[arg(help = "RPN expression to evaluate (e.g., '3 4 + 2 *')")]
    pub expression: Option<String>,
    
    /// Show calculation history
    #[arg(long, help = "Show calculation history")]
    pub history: bool,
}

pub fn run_batch_mode(expression: &str) -> Result<(), RpnError> {
    let mut calculator = Calculator::new(); // Use fresh calculator for batch mode
    match calculator.evaluate_and_format(expression) {
        Ok(result) => {
            println!("{}", result.green().bold());
            Ok(())
        }
        Err(e) => {
            println!("{}", format!("Error: {}", e).red());
            Err(e)
        }
    }
}

pub fn run_interactive_mode() -> Result<(), RpnError> {
    let mut calculator = Calculator::new_with_history();
    let feature_manager = FeatureManager::new();
    let mut rl = DefaultEditor::new().map_err(|e| RpnError::IoError(e.to_string()))?;
    
    println!("{}", "🧮 RPN Calculator - Interactive Mode".cyan().bold());
    println!("{}", "Enter RPN expressions (e.g., '3 4 +' for 3 + 4)".bright_white());
    
    // Show if history was loaded
    let history_count = calculator.get_history().len();
    if history_count > 0 {
        println!("{}", format!("📚 Loaded {} previous calculations from history", history_count).yellow());
    }
    
    println!("\n{}", "Commands:".bright_blue().bold());
    println!("  {}    - Show this help message", "help".green());
    println!("  {}  - Show calculation history", "history".green());
    println!("  {}   - Show current stack and context", "stack".green());
    println!("  {}   - Clear calculation history", "clear".green());
    println!("  {}   - Show LaTeX representation", "latex".green());
    println!("  {}    - Exit the calculator", "quit".green());
    println!();

    loop {
        let readline = rl.readline("rpn> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }
                
                rl.add_history_entry(line).map_err(|e| RpnError::IoError(e.to_string()))?;
                
                match line {
                    "quit" | "exit" | "q" => {
                        println!("{}", "👋 Goodbye!".bright_magenta().bold());
                        break;
                    }
                    "help" | "h" => {
                        show_help();
                    }
                    "history" => {
                        show_history(&calculator);
                    }
                    "clear" => {
                        calculator.clear_history();
                        println!("{}", "🗑️  History cleared and saved.".yellow());
                    }
                    "stack" => {
                        show_stack_and_context(&calculator, &feature_manager);
                    }
                    "latex" => {
                        if let Some(last_expr) = calculator.get_history().last() {
                            let expr_part = last_expr.split(" = ").next().unwrap_or("");
                            let latex = feature_manager.render_latex(expr_part);
                            println!("{}", latex);
                        } else {
                            println!("{}", "No expressions to convert to LaTeX".yellow());
                        }
                    }
                    _ => {
                        match calculator.evaluate_and_format(line) {
                            Ok(result) => {
                                // Parse the result back to f64 for rendering equation box
                                if let Ok(result_num) = result.parse::<f64>() {
                                    #[cfg(feature = "latex-rendering")]
                                    {
                                        let equation_box = feature_manager.latex_renderer.render_equation_box(line, result_num);
                                        println!("{}", equation_box);
                                    }
                                    #[cfg(not(feature = "latex-rendering"))]
                                    {
                                        println!("{}", format!("= {}", result).green().bold());
                                    }
                                } else {
                                    println!("{}", format!("= {}", result).green().bold());
                                }
                                
                                // Show enhanced stack status
                                show_stack_status(&calculator, &feature_manager);
                            }
                            Err(e) => {
                                println!("{}", format!("Error: {}", e).red());
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "⚠️  CTRL-C pressed. Use 'quit' to exit.".yellow());
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "👋 CTRL-D pressed. Goodbye!".bright_magenta().bold());
                break;
            }
            Err(err) => {
                return Err(RpnError::IoError(format!("Error reading input: {}", err)));
            }
        }
    }
    
    Ok(())
}

fn show_help() {
    println!("{}", "📖 RPN Calculator Help".cyan().bold());
    println!("{}", "======================".cyan());
    println!();
    println!("{}", "RPN (Reverse Polish Notation) places operators after operands.".bright_white());
    println!();
    println!("{}", "Binary operators (require two operands):".blue().bold());
    println!("  {}    Addition", "+".green());
    println!("  {}    Subtraction", "-".green());
    println!("  {}    Multiplication", "*".green());
    println!("  {}    Division", "/".green());
    println!("  {} or {}  Power/Exponentiation", "^".green(), "pow".green());
    println!("  {} or {}  Modulo", "%".green(), "mod".green());
    println!();
    println!("{}", "Unary operators (require one operand):".magenta().bold());
    println!("  {}   Square root", "sqrt".yellow());
    println!("  {}    Sine (radians)", "sin".yellow());
    println!("  {}    Cosine (radians)", "cos".yellow());
    println!("  {}    Tangent (radians)", "tan".yellow());
    println!("  {}    Base-10 logarithm", "log".yellow());
    println!("  {}     Natural logarithm", "ln".yellow());
    println!("  {}    Absolute value", "abs".yellow());
    println!("  {}  Floor (round down)", "floor".yellow());
    println!("  {}   Ceiling (round up)", "ceil".yellow());
    println!("  {}  Round to nearest integer", "round".yellow());
    println!();
    println!("{}", "Examples:".bright_blue().bold());
    println!("  {} {}       {}  {}     {}", "3 4".white(), "+".green(), "→".bright_blue(), "7".green().bold(), "(equivalent to 3 + 4)".dimmed());
    println!("  {} {}       {}  {}     {}", "2 3".white(), "^".green(), "→".bright_blue(), "8".green().bold(), "(equivalent to 2³)".dimmed());
    println!("  {} {}     {}  {}     {}", "25".white(), "sqrt".yellow(), "→".bright_blue(), "5".green().bold(), "(square root of 25)".dimmed());
    println!("  {} {}       {}  {}     {}", "0".white(), "sin".yellow(), "→".bright_blue(), "0".green().bold(), "(sine of 0 radians)".dimmed());
    println!("  {} {}      {}  {}     {}", "-5".white(), "abs".yellow(), "→".bright_blue(), "5".green().bold(), "(absolute value of -5)".dimmed());
    println!("  {} {}   {}  {}     {}", "3.7".white(), "floor".yellow(), "→".bright_blue(), "3".green().bold(), "(floor of 3.7)".dimmed());
    println!("  {} {} {}   {}  {}    {}", "3 4".white(), "+".green(), "2 *".green(), "→".bright_blue(), "14".green().bold(), "(equivalent to (3 + 4) * 2)".dimmed());
    println!("  {} {} {}   {}  {}   {}", "2 3 4".white(), "+".green(), "^".green(), "→".bright_blue(), "128".green().bold(), "(equivalent to 2^(3+4))".dimmed());
    println!();
    println!("{}", "Commands:".bright_blue().bold());
    println!("  {}    - Show this help message", "help".green());
    println!("  {} - Show calculation history", "history".green());
    println!("  {}   - Show current stack and context", "stack".green());
    println!("  {}   - Clear calculation history", "clear".green());
    println!("  {}   - Show LaTeX representation", "latex".green());
    println!("  {}    - Exit the calculator", "quit".green());
    println!();
}

fn show_history(calculator: &Calculator) {
    let history = calculator.get_history();
    if history.is_empty() {
        println!("{}", "📭 No calculations in history.".yellow());
    } else {
        println!("{}", "📊 Calculation History:".cyan().bold());
        println!("{}", "=====================".cyan());
        for (i, entry) in history.iter().enumerate() {
            let parts: Vec<&str> = entry.split(" = ").collect();
            if parts.len() == 2 {
                println!("{}: {} {} {}", 
                    format!("{:3}", i + 1).bright_black(),
                    parts[0].white(),
                    "=".bright_blue(),
                    parts[1].green().bold()
                );
            } else {
                println!("{}: {}", format!("{:3}", i + 1).bright_black(), entry.white());
            }
        }
    }
}

fn show_stack_status(calculator: &Calculator, feature_manager: &FeatureManager) {
    let stack_display = feature_manager.display_stack(calculator);
    if calculator.get_stack_depth() > 0 {
        println!("{}", stack_display);
    }
}

fn show_stack_and_context(calculator: &Calculator, feature_manager: &FeatureManager) {
    println!("{}", "📚 Calculator Context:".cyan().bold());
    println!("{}", "====================".cyan());
    
    // Enhanced stack display
    let stack_display = feature_manager.display_stack(calculator);
    println!("{}", stack_display);
    
    // History context
    let history_count = calculator.get_history().len();
    println!("{}: {}", "History".blue(), format!("{} calculations", history_count).white());
    
    // Mode indicators
    println!("{}: {}", "Mode".blue(), "Radians".white());
    println!("{}: {}", "Base".blue(), "Decimal".white());
    
    // LaTeX symbols (if enabled)
    #[cfg(feature = "latex-rendering")]
    {
        println!();
        println!("{}", feature_manager.latex_renderer.get_mathematical_symbols());
    }
}
