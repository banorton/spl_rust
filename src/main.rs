mod lsystem;
mod renderer;
mod turtle;

use macroquad::prelude::*;

#[macroquad::main("L-System Tree")]
async fn main() {
    let mut sys = lsystem::LSystem::new("F");
    sys.add_weighted_rule('F', 1.0, "FF+[+F-F-F]-[-F+F+F]");
    sys.add_weighted_rule('F', 0.5, "FF-[-F+F]+[+F-F]");
    sys.add_weighted_rule('F', 0.3, "F[+F]F[-F]F");

    let expanded = sys.expand(4);
    let config = turtle::TurtleConfig {
        step_length: 4.0,
        angle_delta: 22.5_f32.to_radians(),
    };
    let lines = turtle::interpret(&expanded, &config);

    loop {
        let (offset, scale) = renderer::compute_transform(&lines);
        renderer::draw_lines(&lines, offset, scale);
        next_frame().await;
    }
}
