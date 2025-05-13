# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test [test_name]` (for single test)
- Format code: `cargo fmt`
- Linting: `cargo clippy`

## Code Style Guidelines
- **Imports:** Use `use eframe::egui;` style, group related imports
- **Formatting:** Follow Rust standard formatting (enforced by rustfmt)
- **Types:** 
  - Use generics with trait bounds (`impl<T: std::fmt::Display>`)
  - Use Rust idioms like `Option<usize>` for nullable values
- **Naming:** Use snake_case for variables/functions, CamelCase for types
- **Error Handling:** Use Rust Result type and the `?` operator
- **Structure:** 
  - Implement widgets as separate structs with a `show` method
  - Use `saturating_sub()` for numeric operations that shouldn't underflow
- **Comments:** Limited - code should be self-documenting
- **Documentation:** Add examples to docs.md for new features/widgets