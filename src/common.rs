use std::default::Default;
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

pub fn diagonal_distance(from: Vec2, to: Vec2) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    dx.abs().max(dy.abs())
}

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

pub fn sort_y(mut p0: Vec2, mut p1: Vec2, mut p2: Vec2) -> (Vec2, Vec2, Vec2) {
    if p1.y < p0.y {
        std::mem::swap(&mut p1, &mut p0);
    }
    if p2.y < p0.y {
        std::mem::swap(&mut p2, &mut p0);
    }
    if p2.y < p1.y {
        std::mem::swap(&mut p2, &mut p1);
    }
    (p0, p1, p2)
}

pub fn triangle(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec<Vec2> {
    let mut points = Vec::new();
    let (p0, p1, p2) = sort_y(p0, p1, p2);
    let long_side = line(p0, p2);
    let mut other = line(p0, p1);
    other.extend(line(p1, p2));
    let mut other = other.iter().peekable();
    for point in &long_side {
        let current_y = point.y.round();
        while other
            .peek()
            .map_or(false, |point| (point.y.round() - current_y).abs() < 0.1)
        {
            other.next();
        }
        let other_point = other.next();
        match other_point {
            Some(other) => points.extend(dotted_line(*point, *other, std::f32::consts::PI)),
            None => break,
        }
    }
    points
}

pub fn triangle2(p0: Vec2, p1: Vec2, p2: Vec2) -> Vec<Vec2> {
    let rect_points = Rect::bounding(&[p0, p1, p2])
        .points()
        .into_iter()
        .filter(|point| point_is_in_triangle(*point, p0, p1, p2))
        .collect();
    rect_points
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

struct Bounds {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl Bounds {
    pub fn wrap(points: &[Vec2]) -> Self {
        points.iter().fold(Self::default(), |mut bounds, point| {
            bounds.wrap_point(*point);
            bounds
        })
    }

    pub fn wrap_point(&mut self, point: Vec2) {
        let Vec2 { x, y } = point;
        if x > self.right {
            self.right = x;
        } else if x < self.left {
            self.left = x;
        }
        if y < self.top {
            self.top = y;
        } else if y > self.bottom {
            self.bottom = y;
        }
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            top: f32::MAX,
            bottom: f32::MIN,
            right: f32::MIN,
            left: f32::MAX,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Size,
}

impl Rect {
    pub fn new(pos: Vec2, size: Size) -> Self {
        Self {
            origin: pos.round(),
            size,
        }
    }

    pub fn bounding(points: &[Vec2]) -> Self {
        let Bounds {
            top,
            bottom,
            left,
            right,
        } = Bounds::wrap(points);

        let origin = Vec2::new(left, top).round();
        let size = Size::new(
            (right.round() - left.round()) as u32,
            (bottom.round() - top.round()) as u32,
        );
        Self { origin, size }
    }

    pub fn points(&self) -> Vec<Vec2> {
        let (start_x, start_y, end_x, end_y) = (
            self.origin.x.round() as i32,
            self.origin.y.round() as i32,
            self.origin.x.round() as i32 + self.size.width as i32,
            self.origin.y.round() as i32 + self.size.height as i32,
        );
        let mut points = Vec::with_capacity(((end_x - start_x) * (end_y - start_y)) as usize);
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                points.push(Vec2::new(x as f32, y as f32));
            }
        }
        points
    }
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

    pub fn area(self) -> u32 {
        self.width * self.height
    }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start.mul_add(1.0 - t, end * t)
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
