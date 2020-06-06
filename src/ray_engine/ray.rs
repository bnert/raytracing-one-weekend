use crate::three::vector3::Vector;
use crate::three::color3::ColorRGB;

/**
 * A Ray's main operation is `interpolation` which
 * in this case is a linear interpolation of a direction
 * with respect to its origin
 * 
 * Equation is: P(t) = A + tb,
 * where P == point @ time
 * A = ray origin
 * b = ray direction
 * t = real number param
 */
pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vector::new(),
            direction: Vector::new()
        }
    }

    pub fn create(origin: Vector, direction: Vector) -> Ray {
        Ray { origin: origin, direction: direction }
    }

    pub fn interpolate(&self, t: f64) -> Vector {
        self.origin.sum(&self.direction.scale(t))
    }

    pub fn to_rgb(&self) -> ColorRGB {
        let unit_vec = self.direction.unit_vec();
        let t: f64 = 0.5 * (unit_vec.y + 1.0);
        let rgb_ref = [0.5, 0.7, 1.0];
        let mut rgb = [0.0, 0.0, 0.0];
        for i in 0..3 {
            // When t == 1, all white will show on
            // the screen, otherwise a gradient to blue
            // will show.
            rgb[i] = (t * 1.0) + ((1.0 - t) * rgb_ref[i]);
        }
        ColorRGB::from_f64(rgb[0], rgb[1], rgb[2])
    }
}
