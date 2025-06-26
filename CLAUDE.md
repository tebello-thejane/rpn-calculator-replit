# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Run in interactive mode
- `cargo run -- "3 4 +"` - Run in batch mode with expression
- `cargo test` - Run all tests

### Development
- `cargo check` - Quick compilation check without producing executable
- `cargo clippy` - Run linter for code quality checks
- `cargo fmt` - Format code according to Rust standards

## Architecture

This is a Rust-based RPN (Reverse Polish Notation) calculator with modular architecture:

### Core Components
- **Calculator (`calculator.rs`)** - Main calculation engine with persistent stack and history
  - Maintains stack state between calculations in interactive mode
  - Auto-saves calculation history to `~/.local/share/rpn-calculator/history.json`
  - Handles both batch mode (fresh calculator) and interactive mode (persistent state)
  
- **Parser (`parser.rs`)** - Token parsing and operator classification
  - Converts string expressions into `Token` enum (Number/Operator)
  - Distinguishes between unary and binary operators
  - Supports alternate operator names (e.g., "pow" for "^", "mod" for "%")

- **CLI (`cli.rs`)** - User interface with dual modes
  - Interactive mode with rustyline for command history and colored output
  - Batch mode for single expression evaluation
  - Built-in commands: help, history, stack, clear, quit

- **Error Handling (`error.rs`)** - Comprehensive error types with user-friendly messages

### Key Architectural Patterns
- **Stack Persistence**: Interactive mode maintains stack between calculations, enabling chained operations
- **History Management**: Automatic serialization/deserialization of calculation history using serde
- **Mode Switching**: Batch vs interactive modes with different calculator initialization strategies
- **Operator Polymorphism**: Unified operator handling for both unary and binary operations

### Data Flow
1. Expression parsing: String → Vec<Token>
2. Stack evaluation: Process tokens sequentially, maintaining RPN stack semantics
3. Result formatting: Handle integer vs float display formatting
4. History persistence: Auto-save after each calculation in interactive mode

### Testing
- Unit tests for each module focusing on calculation logic, parsing, and error conditions
- Tests cover edge cases like division by zero, insufficient operands, and complex expressions