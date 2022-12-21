pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start.mul_add(1.0 - t, end * t)
}

pub fn lerp_u8(start: u8, end: u8, t: f32) -> u8 {
    let s = start as f32;
    let e = end as f32;
    (s.mul_add(1.0 - t, e * t) % 255.0) as u8
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
