use clap::Parser;
use rpn_calculator::cli::{Args, run_interactive_mode, run_batch_mode};

fn main() {
    let args = Args::parse();
    
    match args.expression {
        Some(expr) => {
            // Batch mode - process the expression from command line
            if let Err(e) = run_batch_mode(&expr) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            // Interactive mode
            if let Err(e) = run_interactive_mode() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
