pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn create(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }
}
