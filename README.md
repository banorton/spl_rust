# spl_rust

Course code for survey of programming languages.

- **docs/** - Documentation
- **code/** - Projects and code examples
- **src/** - L-system tree generator

## L-System Tree Generator

Procedurally generates natural-looking trees using [L-systems](https://en.wikipedia.org/wiki/L-system) and turtle graphics, rendered in a 2D window with macroquad.

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)

### Run the demo

```bash
cargo run
```

A window will open displaying a generated tree using the grammar `F → FF+[+F-F-F]-[-F+F+F]` with angle 22.5° and 4 iterations. Close the window to exit.

### Run tests

```bash
cargo test
```
