pub mod color3;


pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new() -> Vector {
        Vector{ x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn create(x: f64, y: f64, z: f64) -> Vector {
        Vector{ x, y, z }
    }

    pub fn sum(&self, other: &Vector) -> Vector {
        Vector{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn product(&self, other: &Vector) -> Vector {
        Vector{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    pub fn scale(&self, factor: f64) -> Vector {
        Vector{
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }

    pub fn len_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let v1 = Vector::create(4.0, 6.0, 1.5);
        let v2 = Vector::create(2.0, 2.0, 1.5);
        let p = v1.product(&v2);
        assert_eq!(p.x, 8.0);
        assert_eq!(p.y, 12.0);
        assert_eq!(p.z, 2.25);
    }

    #[test]
    fn test_scale() {
        let v = Vector::create(4.0, 2.0, 1.0);
        let v = v.scale(2.0);
        assert_eq!(v.x, 8.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.z, 2.0);
    }
}
