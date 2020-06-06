use crate::three::vector3::Vector;

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_width: u32,
    pub img_height: u32,

    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    pub origin: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub lower_left_corner: Vector
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 0.0,
            img_width: 0,
            img_height: 0,

            viewport_height: 0.0,
            viewport_width: 0.0,
            focal_length: 0.0,

            origin: Vector::new(),
            horizontal: Vector::new(), 
            vertical: Vector::new(),
            lower_left_corner: Vector::new()
        }
    }

    pub fn create(asp: f32, img_w: u32, view_h: f32, foc_len: f32) -> Camera {
        let asp_r = asp;
        let iw = img_w;
        let ih = (img_w as f32 / asp) as u32;
        let vh = view_h;
        let vw = vh * asp;
        let fl = foc_len;
        let orig = Vector::create(0.0, 0.0, 0.0);
        let horz = Vector::create(vw as f64, 0.0, 0.0);
        let vert = Vector::create(0.0, vh as f64, 0.0);
        // This computes where the lower left corner of
        // the viewable plane (viewport) is located.
        // Eq: origin - (vert / 2) - (horz / 2) - (0, 0, focal_len)
        let llc = orig.sub(&horz.scale(0.5))
            .sub(&vert.scale(0.5))
            .sub(&Vector::create(0.0, 0.0, fl.into()));

        Camera {
            aspect_ratio: asp_r,
            img_width: iw,
            img_height: ih,

            viewport_height: vh,
            viewport_width: vh * asp,
            focal_length: fl,

            origin: orig,
            horizontal: horz,
            vertical: vert,
            lower_left_corner: llc
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        let asp_ratio = 16.0 / 9.0;
        let img_w = 384;
        let view_h = 2.0;
        let focal_l = 1.0;
        let c = Camera::create(asp_ratio, img_w, view_h, focal_l);
        let view_w = view_h * asp_ratio;
        let horz_res = ((32.0 / 9.0) as f32) as f64;
        assert_eq!(c.horizontal.x, horz_res); // gross but good for now
        let l = c.lower_left_corner;
        assert_eq!(l.x, -(horz_res / 2.0));
        assert_eq!(l.y, -1.0);
        assert_eq!(l.z, -1.0);
    }
}
