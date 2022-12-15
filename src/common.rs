use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn round(&self) -> Self {
        Vec2::new(self.x.round(), self.y.round())
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.sin(),
            y: angle.cos(),
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn diagonal_distance(from: Vec2, to: Vec2) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    return dx.abs().max(dy.abs());
}

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

pub fn rect(from: Vec2, to: Vec2) -> Vec<Vec2> {
    let (start_x, start_y, end_x, end_y) = (from.x.round() as i32, from.y.round() as i32, to.x.round() as i32, to.y.round() as i32);
    let mut points = Vec::with_capacity(((end_x - start_x) * (end_y - start_y)) as usize);
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            points.push(Vec2::new(x as f32, y as f32));
        }
    }
    points
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    return start * (1.0 - t) + end * t;
}

pub fn min(of: i32, or: i32) -> i32 {
    of.min(or)
}

pub fn max(of: i32, or: i32) -> i32 {
    of.max(or)
}

pub fn constrain<T: PartialOrd>(this: T, min: T, max: T) -> T {
    assert!(min < max);
    if this < min {
        return min;
    } else if this > max {
        return max;
    }
    this
}
