use crate::shapes::sphere::Sphere;
use crate::three::color3::ColorRGB;
use crate::three::vector3::Vector;

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
#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> f64 {
    /**
     * From the given unit vectors, construct a quadratic
     * equation to determine if there exists a surface in the
     * path of the given ray.
     *
     * eq: (-b (+/-) sqrt(b^2 - 4ac)) / 2a
     *
     * With b = 2h, where h is the length of dot product
     * of the camera/sphere center difference and the ray direction
     */
    let camera_sphere_delta = ray.origin.sub(&sphere.center);
    let a = ray.direction.len_sq();
    let b = camera_sphere_delta.dot(&ray.direction);
    let c = camera_sphere_delta.len_sq() - (sphere.radius.powi(2));
    // In the form of b^2 - 4 * a * c
    // for a given quadtratic. If greater than 0, then
    // there is at least one intersection with a surface
    let discriminant = b.powi(2) - (a * c);
    // The ray does not intersect w/ a surface
    if discriminant < 0.0 {
        return -1.0;
    }
    // There is at least one 
    return (-b - discriminant.sqrt()) / a;
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vector::new(),
            direction: Vector::new(),
        }
    }

    pub fn create(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn interpolate(&self, t: f64) -> Vector {
        self.origin.sum(&self.direction.scale(t))
    }

    pub fn to_rgb(&self) -> ColorRGB {
        let t = hit_sphere(&Sphere::create(0.0, 0.0, -1.0, 0.5), self);
        if (t > 0.0) {
            let n = self.interpolate(t).sub(&Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            });
            let n = n.unit_vec();
            let n = n.sum(&Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            });
            let n = n.scale(0.5);
            return ColorRGB::from_vector3(&n);
        }

        let unit_vec = self.direction.unit_vec();
        let t: f64 = 0.5 * (unit_vec.y + 1.0);
        let white_vec = Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let blue_vec = Vector {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
        let adj_white_vec = white_vec.scale(1.0 - t);
        let adj_blue_vec = blue_vec.scale(t);
        return ColorRGB::from_vector3(&adj_white_vec.sum(&adj_blue_vec));
    }
}
