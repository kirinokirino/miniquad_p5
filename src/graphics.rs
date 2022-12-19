use crate::{
    common::lerp,
    geometry::{Rect, Triangle},
    math::{diagonal_distance, point_is_in_triangle, Vec2},
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

pub fn empty_triangle(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec<Vec2> {
    let mut triangle = line(p0, p1);
    triangle.extend(line(p1, p2));
    triangle.extend(line(p0, p2));
    triangle
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

pub fn triangle(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec<Vec2> {
    let rect_points = Rect::bounding(&[p0, p1, p2])
        .points()
        .into_iter()
        .filter(|point| point_is_in_triangle(*point, &Triangle::new(p0, p1, p2)))
        .collect();
    rect_points
}
