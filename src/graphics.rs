use crate::{
    common::lerp,
    math::{diagonal_distance, Vec2},
};

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
    let diagonal_distance = diagonal_distance(from, to);
    let mut points: Vec<Vec2> = Vec::with_capacity(diagonal_distance as usize);
    for i in 0..diagonal_distance as usize {
        let progress = if i == 0 {
            0.0
        } else {
            i as f32 / diagonal_distance
        };
        let lerp_x = lerp(from.x, to.x, progress);
        let lerp_y = lerp(from.y, to.y, progress);
        points.push(Vec2::new(lerp_x, lerp_y).round());
    }
    points
}

pub fn dotted_line(from: Vec2, to: Vec2, step: f32) -> Vec<Vec2> {
    let diagonal_distance = diagonal_distance(from, to);
    let mut points: Vec<Vec2> = Vec::with_capacity(diagonal_distance as usize);
    for i in 0..(diagonal_distance / step) as usize {
        let progress = if i == 0 {
            0.0
        } else {
            i as f32 / (diagonal_distance / step)
        };
        let lerp_x = lerp(from.x, to.x, progress);
        let lerp_y = lerp(from.y, to.y, progress);
        points.push(Vec2::new(lerp_x, lerp_y).round());
    }
    points
}
