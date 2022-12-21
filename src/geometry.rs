use crate::common::lerp;
use crate::graphics::line;
use crate::math::{diagonal_distance, point_is_in_triangle, Vec2};

/// Module for analytical forms of shapes

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

    /// top_left, top_right, bottom_right, bottom_left
    pub fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        let bottom_right = self.bottom_right();
        let top_right = Vec2::new(bottom_right.x, self.origin.y);
        let bottom_left = Vec2::new(self.origin.x, bottom_right.y);
        (self.origin, top_right, bottom_right, bottom_left)
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.origin + Vec2::new(self.size.width as f32, self.size.height as f32)
    }

    pub fn solid_color(&self) -> Vec<Vec2> {
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

    pub fn empty(&self) -> Vec<Vec2> {
        let (a, b, c, d) = self.corners();
        let mut rectangle = line(a, b);
        rectangle.extend(line(b, c));
        rectangle.extend(line(c, d));
        rectangle.extend(line(d, a));
        rectangle
    }
}

#[derive(Debug)]
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
        }
        if x < self.left {
            self.left = x;
        }
        if y < self.top {
            self.top = y;
        }
        if y > self.bottom {
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

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
}

impl Line {
    pub fn new(from: Vec2, to: Vec2) -> Self {
        Self { a: from, b: to }
    }

    /// 0 -> point on start, 1 -> point on end, (0, 1) -> point between start and end
    /// doesn't work with negative
    pub fn todo_name(&self, point: Vec2) -> f32 {
        let point = self.project(point);
        let from_start = (self.a - point).length_squared();
        let full_path = (self.a - self.b).length_squared();
        (from_start / full_path).sqrt()
    }

    pub fn project(&self, point: Vec2) -> Vec2 {
        let offset = self.a;
        let point = offset - point;
        let plane = offset - self.b;
        offset + (plane * point.dot(plane) * plane.length_squared().recip())
    }

    pub fn solid(&self) -> Vec<Vec2> {
        let Line { a, b } = *self;
        let diagonal_distance = diagonal_distance(a, b);
        let mut points: Vec<Vec2> = Vec::with_capacity(diagonal_distance as usize);
        for i in 0..diagonal_distance as usize {
            let progress = if i == 0 {
                0.0
            } else {
                i as f32 / diagonal_distance
            };
            let lerp_x = lerp(a.x, b.x, progress);
            let lerp_y = lerp(a.y, b.y, progress);
            points.push(Vec2::new(lerp_x, lerp_y).round());
        }
        points
    }

    pub fn dotted(&self, step: f32) -> Vec<Vec2> {
        let Line { a, b } = *self;
        let diagonal_distance = diagonal_distance(a, b);
        let mut points: Vec<Vec2> = Vec::with_capacity(diagonal_distance as usize);
        for i in 0..(diagonal_distance / step) as usize {
            let progress = if i == 0 {
                0.0
            } else {
                i as f32 / (diagonal_distance / step)
            };
            let lerp_x = lerp(a.x, b.x, progress);
            let lerp_y = lerp(a.y, b.y, progress);
            points.push(Vec2::new(lerp_x, lerp_y).round());
        }
        points
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    pub fn new(p0: Vec2, p1: Vec2, p2: Vec2) -> Self {
        // TODO: ensure the order of points (clockwise)
        Self {
            a: p0,
            b: p1,
            c: p2,
        }
    }

    pub fn solid_color(&self) -> Vec<Vec2> {
        let Triangle { a, b, c } = *self;
        let rect_points = Rect::bounding(&[a, b, c])
            .solid_color()
            .into_iter()
            .filter(|point| point_is_in_triangle(*point, self))
            .collect();
        rect_points
    }

    pub fn empty(&self) -> Vec<Vec2> {
        let Triangle { a, b, c } = *self;
        let mut triangle = line(a, b);
        triangle.extend(line(b, c));
        triangle.extend(line(a, c));
        triangle
    }
}
