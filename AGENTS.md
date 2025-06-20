# Agent Guidelines for pong_rs

## Build/Test Commands
- `cargo run` - Build and run the game
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo check` - Quick syntax/type check without building
- `cargo clippy` - Run linter for code quality

## Code Style
- **Language**: Rust 2021 edition with macroquad 0.4 game framework
- **Imports**: Use `use macroquad::prelude::*;` for macroquad functionality
- **Formatting**: Follow standard Rust formatting (use `cargo fmt`)
- **Naming**: snake_case for functions/variables, PascalCase for types/structs
- **Functions**: Use async fn for main game loop functions
- **Error Handling**: Use Result<T, E> and ? operator for error propagation
- **Comments**: Use // for line comments, /// for documentation
- **Game Loop**: Use `next_frame().await` at end of main loop
- **Colors**: Use macroquad color constants (BLACK, WHITE, GRAY, etc.)
- **Input**: Use `is_key_pressed()` for single key presses, `is_key_down()` for held keys

## Project Structure
- `src/main.rs` - Main game entry point
- Game uses macroquad for graphics and input handling
- No custom rules files found - follow standard Rust conventions