use crate::three::vector3::Vector;

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Vector::new(),
            radius: 1.0,
        }
    }

    pub fn create(x: f64, y: f64, z: f64, r: f64) -> Sphere {
        Sphere {
            center: Vector::create(x, y, z),
            radius: r,
        }
    }
}
