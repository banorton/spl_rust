use macroquad::prelude::Vec2;

pub struct TurtleConfig {
    pub step_length: f32,
    pub angle_delta: f32, // radians
}

struct TurtleState {
    pos: Vec2,
    angle: f32, // radians, 0 = up
}

pub fn interpret(input: &str, config: &TurtleConfig) -> Vec<(Vec2, Vec2)> {
    let mut state = TurtleState {
        pos: Vec2::ZERO,
        angle: 0.0,
    };
    let mut stack: Vec<(Vec2, f32)> = Vec::new();
    let mut lines = Vec::new();

    for ch in input.chars() {
        match ch {
            'F' => {
                let dx = config.step_length * state.angle.sin();
                let dy = -config.step_length * state.angle.cos();
                let next = state.pos + Vec2::new(dx, dy);
                lines.push((state.pos, next));
                state.pos = next;
            }
            '+' => state.angle += config.angle_delta,
            '-' => state.angle -= config.angle_delta,
            '[' => stack.push((state.pos, state.angle)),
            ']' => {
                if let Some((pos, angle)) = stack.pop() {
                    state.pos = pos;
                    state.angle = angle;
                }
            }
            _ => {} // ignore unknown chars (e.g. 'X')
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn approx_eq(a: Vec2, b: Vec2) -> bool {
        (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4
    }

    #[test]
    fn test_single_forward() {
        let config = TurtleConfig {
            step_length: 10.0,
            angle_delta: PI / 4.0,
        };
        let lines = interpret("F", &config);
        assert_eq!(lines.len(), 1);
        assert!(approx_eq(lines[0].0, Vec2::ZERO));
        assert!(approx_eq(lines[0].1, Vec2::new(0.0, -10.0))); // up
    }

    #[test]
    fn test_turn_and_forward() {
        let config = TurtleConfig {
            step_length: 10.0,
            angle_delta: PI / 2.0,
        };
        let lines = interpret("F+F", &config);
        assert_eq!(lines.len(), 2);
        assert!(approx_eq(lines[1].1, Vec2::new(10.0, -10.0))); // turned right 90°
    }

    #[test]
    fn test_branch_restores_state() {
        let config = TurtleConfig {
            step_length: 10.0,
            angle_delta: PI / 4.0,
        };
        let lines = interpret("F[+F]F", &config);
        assert_eq!(lines.len(), 3);
        // third segment should start from end of first (branch restored state)
        assert!(approx_eq(lines[2].0, lines[0].1));
    }

    #[test]
    fn test_unknown_chars_ignored() {
        let config = TurtleConfig {
            step_length: 10.0,
            angle_delta: PI / 4.0,
        };
        let lines = interpret("XFX", &config);
        assert_eq!(lines.len(), 1);
    }
}
