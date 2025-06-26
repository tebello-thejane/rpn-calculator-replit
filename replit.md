# RPN Calculator

## Overview
This is a command-line Reverse Polish Notation (RPN) calculator built in Rust. The application provides both single-expression evaluation and an interactive REPL mode for continuous calculations. RPN is a mathematical notation where operators follow their operands (e.g., "3 4 +" instead of "3 + 4").

## System Architecture
The application follows a modular Rust architecture with clear separation of concerns:

**Language & Runtime**: Rust 2021 edition with cargo build system
**Interface**: Command-line application with both one-shot and interactive modes
**Data Storage**: Local file-based configuration and history (using user directories)
**Dependencies**: Minimal external dependencies focused on CLI functionality

## Key Components

### Core Calculator (`src/calculator.rs`)
- **Purpose**: Contains the main RPN calculation logic
- **Architecture**: Stack-based evaluation engine that processes tokens sequentially
- **Design Decision**: Uses a vector as a stack for simplicity and performance
- **Operations**: Supports basic arithmetic operations (+, -, *, /)

### Parser (`src/parser.rs`) 
- **Purpose**: Tokenizes and validates input expressions
- **Architecture**: Simple string-based parser that splits input into tokens
- **Design Decision**: Separates parsing from evaluation for better error handling
- **Validation**: Checks for valid numbers and operators

### Error Handling (`src/error.rs`)
- **Purpose**: Centralized error management for the application
- **Architecture**: Custom error types using Rust's Result pattern
- **Design Decision**: Provides specific error messages for different failure modes
- **Benefits**: Better user experience with clear error descriptions

### CLI Interface (`src/cli.rs`)
- **Purpose**: Handles command-line argument parsing and user interaction
- **Architecture**: Uses clap for argument parsing and rustyline for REPL
- **Design Decision**: Supports both batch and interactive modes
- **Features**: Command history, colored output, and persistent settings

### Main Entry Point (`src/main.rs`)
- **Purpose**: Application bootstrap and mode selection
- **Architecture**: Delegates to appropriate modules based on CLI arguments
- **Design Decision**: Keeps main function minimal for maintainability

### Library Interface (`src/lib.rs`)
- **Purpose**: Exposes public API for potential library usage
- **Architecture**: Re-exports core functionality from modules
- **Design Decision**: Allows the calculator to be used as both binary and library

## Data Flow

1. **Input Processing**: User input (CLI args or REPL) → CLI module
2. **Parsing**: Raw input → Parser → Validated tokens
3. **Calculation**: Tokens → Calculator → Result or Error
4. **Output**: Result → CLI → Formatted output to user
5. **State Management**: History and settings managed by CLI module

## External Dependencies

### Production Dependencies
- **clap (4.0)**: Command-line argument parsing with derive macros
- **rustyline (12.0)**: Interactive line editing and history for REPL mode
- **serde (1.0)**: Serialization framework for configuration persistence
- **serde_json (1.0)**: JSON serialization for settings storage
- **dirs (5.0)**: Cross-platform user directory detection
- **colored (2.0)**: Terminal color output for better user experience

### Design Rationale
- **clap**: Chosen for its ergonomic derive API and comprehensive CLI features
- **rustyline**: Provides professional REPL experience with history and editing
- **serde**: Industry standard for Rust serialization, enables easy config management
- **dirs**: Cross-platform way to find user directories for storing settings
- **colored**: Enhances usability with visual feedback

## Deployment Strategy

### Development
- **Build System**: Cargo with standard Rust project structure
- **Environment**: Configured for Replit with Nix package management
- **Dependencies**: All dependencies managed through Cargo.toml

### Runtime Requirements
- **Platform**: Cross-platform (Linux, macOS, Windows)
- **Runtime**: No external runtime dependencies beyond system libraries
- **Storage**: Uses user home directory for configuration and history

### Distribution
- **Binary**: Single executable built with `cargo build --release`
- **Installation**: Can be installed via `cargo install` or direct binary distribution
- **Configuration**: Auto-creates config files in user directories on first run

## Changelog
- June 26, 2025. Initial setup

## User Preferences
Preferred communication style: Simple, everyday language.