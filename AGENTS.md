# Super-Compare Agent Guide

## Build & Run

- **Build (debug):** `cargo build`
- **Build (release):** `cargo build --release`
- **Run tests:** `cargo test`
- **Binary locations:**
  - Debug: `target/debug/super-compare.exe`
  - Release: `target/release/super-compare.exe`
  - Tests expect the binary at `target/debug/super-compare.exe`

## Project Structure

- `src/main.rs` – only source file (CLI directory comparison tool)
- `tests/test_compare.rs` – integration tests using tempdirs and the debug binary
- `Cargo.toml` – use edition 2024

## Usage

```bash
super-compare <dir1> [dir2]
```

Output format: `+` for files added (only in dir2), `-` for files removed (only in dir1)

## Testing

- Write tests for every feature addition
- Tests live in `tests/` and use `tempfile` for temp dirs
- Tests call the debug binary directly from `target/debug/super-compare.exe`