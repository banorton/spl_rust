# L-System Tree Generator — MVP Plan

## Goal

Minimal working L-system that expands a grammar and renders a 2D tree to a window. No UI chrome, no config files — just a hardcoded interesting tree grammar rendered on screen.

## Architecture

Single Cargo binary crate at project root (`src/main.rs`) with three modules:

```
src/
  main.rs          — sets up grammar, runs expansion, launches renderer
  lsystem.rs       — grammar definition + string expansion
  turtle.rs        — interprets expanded string into line segments
  renderer.rs      — draws line segments to a window (macroquad)
```

## Modules

### 1. `lsystem.rs` — Grammar & Expansion

- `struct Rule { from: char, to: String }`
- `struct LSystem { axiom: String, rules: Vec<Rule> }`
- `fn expand(&self, iterations: usize) -> String` — apply rules N times
- Simple `HashMap<char, String>` lookup for O(1) rule matching

### 2. `turtle.rs` — Turtle Interpreter

Interprets the expanded string using turtle graphics commands:

| Char | Action |
|------|--------|
| `F`  | Move forward, draw line |
| `+`  | Turn right by angle |
| `-`  | Turn left by angle |
| `[`  | Push state (position + angle) onto stack |
| `]`  | Pop state from stack |

- `struct TurtleState { x: f32, y: f32, angle: f32 }`
- `struct TurtleConfig { step_length: f32, angle_delta: f32 }`
- `fn interpret(input: &str, config: &TurtleConfig) -> Vec<(Vec2, Vec2)>` — returns line segments

### 3. `renderer.rs` — Drawing

- Uses `macroquad` for a simple 2D window
- `async fn render_lines(lines: &[(Vec2, Vec2)])` — draws all segments each frame
- Green color, black background, that's it

### 4. `main.rs`

- Hardcode a classic tree grammar, e.g.:
  - Axiom: `F`
  - Rule: `F → FF+[+F-F-F]-[-F+F+F]`
  - Angle: 22.5°, Iterations: 4
- Expand → interpret → render

## Dependencies

- `macroquad` — simple 2D rendering, async main, zero boilerplate

## Out of Scope (for now)

- CLI args / parameter tweaking
- Multiple grammars / presets
- ASCII rendering
- Stochastic rules
- Color/thickness variation
- Animation of growth

## Steps

1. `cargo init` at project root, add `macroquad` dep
2. Implement `lsystem.rs` with expansion + unit tests
3. Implement `turtle.rs` with interpreter + unit tests
4. Implement `renderer.rs` with macroquad drawing
5. Wire everything in `main.rs`
6. Run `cargo fmt && cargo clippy`, fix any issues
7. Commit
