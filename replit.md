# RPN Calculator Project

## Overview
A feature-rich command-line RPN calculator built in Rust with interactive and batch processing modes. The calculator provides comprehensive mathematical operations, persistent history, colorful interface, and real-time stack visualization.

## Recent Changes
- **2025-06-26**: Added stack display and context visualization
  - Real-time stack contents shown after each calculation
  - Context command displays stack depth, history count, mode indicators
  - Intelligent operation suggestions based on current stack state
  - Enhanced visual feedback for better user experience

- **2025-06-26**: Implemented colorful interface with emojis
  - Color-coded output: green for results, red for errors
  - Styled help system with operator categorization
  - Visual indicators for different interface elements
  - Enhanced history display with structured formatting

- **2025-06-26**: Added persistent calculation history
  - Cross-platform storage using system data directory
  - Automatic saving after each calculation
  - History preserved between sessions for both modes
  - Clear command properly removes and saves empty state

- **2025-06-26**: Enhanced mathematical operations
  - Added 10 new mathematical functions (power, modulo, trigonometric, logarithmic, rounding)
  - Unary and binary operation support with proper validation
  - Comprehensive error handling for domain-specific operations

## Project Architecture

### Core Components
- **Calculator Engine** (`src/calculator.rs`): Core RPN evaluation logic with stack management
- **Parser** (`src/parser.rs`): Token parsing and operator definitions
- **CLI Interface** (`src/cli.rs`): Interactive and batch mode handling with visual enhancements
- **Error Handling** (`src/error.rs`): Comprehensive error types and user-friendly messages

### Key Features
1. **Mathematical Operations**: 16 operations including basic arithmetic, advanced functions
2. **Stack Visualization**: Real-time display of stack contents and context information
3. **Persistent History**: Cross-platform storage with JSON serialization
4. **Colorful Interface**: Styled output with emojis and color coding
5. **Dual Modes**: Interactive REPL and batch command-line processing

### Dependencies
- `clap`: Command-line argument parsing
- `rustyline`: Interactive readline with history
- `serde`/`serde_json`: History serialization
- `dirs`: Cross-platform data directory
- `colored`: Terminal color output

## User Preferences
- Prefers colorful, visually appealing interfaces
- Values practical, useful functionality over decorative features
- Appreciates real-time feedback and context information
- Wants efficient calculator workflow with visual stack management

## Technical Notes
- All 12 tests passing consistently
- Batch mode uses fresh calculator instance for isolated calculations
- Interactive mode maintains persistent stack for continuous operations
- History automatically saves after each calculation
- Cross-platform compatibility for data storage