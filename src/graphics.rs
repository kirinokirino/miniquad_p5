use crate::{geometry::Line, math::Vec2};

/// Functions here produce pixels, analytical shapes are in `geometry`

pub fn circle(origin: Vec2, radius: f32) -> Vec<Vec2> {
    let surface = (radius * std::f32::consts::TAU).ceil();
    let mut points: Vec<Vec2> = Vec::with_capacity(surface as usize);
    for i in 0..surface as usize {
        points.push(origin + Vec2::from_angle(std::f32::consts::TAU / surface * i as f32) * radius);
    }
    points
}

pub fn line(from: Vec2, to: Vec2) -> Vec<Vec2> {
    Line::new(from, to).solid()
}
