pub fn in_range(v: f32, range: (f32, f32)) -> f32 {
    if v < range.0 {
        range.0
    } else if v > range.1 {
        range.1
    } else {
        v
    }
}