use std::ops::{Add, Mul};

/// Get either width or height, depending on which is longer.
pub fn diagonal_distance(from: Vec2, to: Vec2) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    dx.abs().max(dy.abs())
}

/*
    Determine if point is within triangle formed by points p1, p2, p3.
    If so, the point will be on the same side of each of the half planes
    defined by vectors p1p2, p2p3, and p3p1.
*/
pub fn point_is_in_triangle(point: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> bool {
    let side1 = side_of_the_plane(point, p0, p1);
    let side2 = side_of_the_plane(point, p1, p2);
    let side3 = side_of_the_plane(point, p2, p0);
    side1 && side2 && side3 || !side1 && !side2 && !side3
}

fn side_of_the_plane(point: Vec2, plane1: Vec2, plane2: Vec2) -> bool {
    ((point.x - plane2.x) * (plane1.y - plane2.y) - (plane1.x - plane2.x) * (point.y - plane2.y))
        .is_sign_positive()
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: angle.sin(),
            y: angle.cos(),
        }
    }

    pub fn inside(self, width: i32, height: i32) -> bool {
        self.y > 0.0 && self.x > 0.0 && (self.x as i32) < width && (self.y as i32) < height
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<&Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
