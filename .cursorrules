<!-- AUTO-GENERATED FROM CONSTITUTION.md — DO NOT EDIT DIRECTLY -->

# Rust Best Practices

- Use `clippy` and fix all warnings before committing
- Prefer returning `Result` over calling `unwrap()` in library code; `unwrap()` is acceptable in tests
- Use `thiserror` for library errors and `anyhow` for application errors
- Prefer iterators and combinators over manual loops where readability isn't sacrificed
- Keep functions small and focused on a single responsibility
- Use descriptive variable names; avoid single-letter names outside of closures and short loops
- Derive common traits (`Debug`, `Clone`, `PartialEq`) on public types
- Prefer borrowing over cloning unless ownership is required
- Write doc comments (`///`) for all public items
- Group imports: std first, then external crates, then local modules
- Use `cargo fmt` to enforce consistent formatting
- Prefer strong types and enums over stringly-typed values
- Handle all `Result` and `Option` values explicitly; avoid silent fallbacks
