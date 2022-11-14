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
}

pub fn diagonal_distance(from: Vec2, to: Vec2) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    return dx.abs().max(dy.abs());
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
