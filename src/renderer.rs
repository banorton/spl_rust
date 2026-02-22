use macroquad::prelude::*;

pub fn compute_transform(lines: &[(Vec2, Vec2)]) -> (Vec2, f32) {
    if lines.is_empty() {
        return (Vec2::ZERO, 1.0);
    }
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for &(a, b) in lines {
        for p in [a, b] {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
    }
    let padding = 40.0;
    let w = max_x - min_x;
    let h = max_y - min_y;
    let scale_x = (screen_width() - 2.0 * padding) / w.max(1.0);
    let scale_y = (screen_height() - 2.0 * padding) / h.max(1.0);
    let scale = scale_x.min(scale_y);
    let center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    let offset = Vec2::new(screen_width() / 2.0, screen_height() / 2.0) - center * scale;
    (offset, scale)
}

pub fn draw_lines(lines: &[(Vec2, Vec2)], offset: Vec2, scale: f32) {
    clear_background(BLACK);
    for &(a, b) in lines {
        draw_line(
            a.x * scale + offset.x,
            a.y * scale + offset.y,
            b.x * scale + offset.x,
            b.y * scale + offset.y,
            2.0,
            GREEN,
        );
    }
}
