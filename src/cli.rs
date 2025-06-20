use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::calculator::Calculator;
use crate::error::RpnError;

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
    let mut calculator = Calculator::new();
    let result = calculator.evaluate_and_format(expression);
    println!("{}", result);
    Ok(())
}

pub fn run_interactive_mode() -> Result<(), RpnError> {
    let mut calculator = Calculator::new();
    let mut rl = DefaultEditor::new().map_err(|e| RpnError::IoError(e.to_string()))?;
    
    println!("RPN Calculator - Interactive Mode");
    println!("Enter RPN expressions (e.g., '3 4 +' for 3 + 4)");
    println!("Commands:");
    println!("  help    - Show this help message");
    println!("  history - Show calculation history");
    println!("  clear   - Clear calculation history");
    println!("  quit    - Exit the calculator");
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
                        println!("Goodbye!");
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
                        println!("History cleared.");
                    }
                    _ => {
                        let result = calculator.evaluate_and_format(line);
                        println!("{}", result);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed. Use 'quit' to exit.");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D pressed. Goodbye!");
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
    println!("RPN Calculator Help");
    println!("==================");
    println!();
    println!("RPN (Reverse Polish Notation) places operators after operands.");
    println!();
    println!("Supported operators:");
    println!("  +  Addition");
    println!("  -  Subtraction");
    println!("  *  Multiplication");
    println!("  /  Division");
    println!();
    println!("Examples:");
    println!("  3 4 +      →  7     (equivalent to 3 + 4)");
    println!("  10 5 -     →  5     (equivalent to 10 - 5)");
    println!("  6 7 *      →  42    (equivalent to 6 * 7)");
    println!("  15 3 /     →  5     (equivalent to 15 / 3)");
    println!("  3 4 + 2 *  →  14    (equivalent to (3 + 4) * 2)");
    println!("  2 3 4 + *  →  14    (equivalent to 2 * (3 + 4))");
    println!();
    println!("Commands:");
    println!("  help    - Show this help message");
    println!("  history - Show calculation history");
    println!("  clear   - Clear calculation history");
    println!("  quit    - Exit the calculator");
    println!();
}

fn show_history(calculator: &Calculator) {
    let history = calculator.get_history();
    if history.is_empty() {
        println!("No calculations in history.");
    } else {
        println!("Calculation History:");
        println!("===================");
        for (i, entry) in history.iter().enumerate() {
            println!("{:3}: {}", i + 1, entry);
        }
    }
}
