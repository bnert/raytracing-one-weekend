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
    pub origin: Vector,
    pub direction: Vector
}


fn hit_sphere(sphere: &Vector, radius: f64, ray: &Ray) -> bool {
    let delta_sphere = ray.origin.sub(&sphere);
    let ray_direction = ray.direction.dot(&ray.direction);
    let b = 2.0 * delta_sphere.dot(&ray.direction);
    let c = delta_sphere.dot(&delta_sphere) - (radius * radius);
    // In the form of b^2 - 4 * a * c
    // for a given quadtratic
    let discriminant = b * b - 4.0 * ray_direction * c;
    discriminant > 0.0
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

        if hit_sphere(&Vector{x: 1.0, y: 1.0, z: -2.0}, 0.5, self) {
            return ColorRGB::from_f64(0.0, 0.0, 1.0);
        }

        if hit_sphere(&Vector{x: 0.0, y: 0.0, z: -3.0}, 0.5, self) {
            return ColorRGB::from_f64(0.0, 1.0, 0.0);
        }

        if hit_sphere(&Vector{x: 1.0, y: 1.0, z: -5.0}, 0.5, self) {
            return ColorRGB::from_f64(1.0, 0.0, 0.0);
        }
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
        return ColorRGB::from_f64(rgb[0], rgb[1], rgb[2]);
    }
}
