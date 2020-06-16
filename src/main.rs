use std::fs::File;
use std::io::{Write};

mod three;
use three::vector3::Vector;
use three::color3::ColorRGB;

mod ray_engine;
use ray_engine::camera::Camera;
use ray_engine::ray::Ray;

fn main() {
    let asp_ratio = 16.0 / 9.0;
    let img_w = 384;
    let view_h = 2.0;
    let focal_l = 1.0;
    let mut cam = Camera::create(asp_ratio, img_w, view_h, focal_l);

    // Open the file
    let mut file_h = File::create("test.ppm").unwrap();
    file_h.write(b"P3\n").unwrap();
    file_h.write(&cam.img_width.to_string().as_bytes()).unwrap();
    file_h.write(b" ").unwrap();
    file_h.write(&cam.img_height.to_string().as_bytes()).unwrap();
    file_h.write(b"\n255\n").unwrap();

    let adj_img_h: f64 = cam.img_height as f64 - 1.0;
    let adj_img_w: f64 = cam.img_width as f64 - 1.0;
    for x in 0..cam.img_height {
        for y in 0..cam.img_width {
            let r: f64 = (y as f64) / adj_img_w;
            let g: f64 = (adj_img_h - x as f64) / adj_img_h;
            let b: f64 = 0.90;

            // let u = (y as f64) / adj_img_w;
            // let v = (x as f64) / adj_img_h;
            // let direction = cam.lower_left_corner
            //     .sum(&cam.horizontal.scale(u))
            //     .sum(&cam.vertical.scale(v))
            //     .sub(&cam.origin);
            // let ray = Ray::create(cam.origin.copy(), direction);

            // let color = ColorRGB::from_f64(r, g, b);
            // let color = ray.to_rgb();
            let color = ColorRGB::from_f64(r, g, b);
            match color.write_ppm_row(&mut file_h) {
                Err(_) => println!("Unable to write row\n"),
                _ => {
                    // noop
                }
            }
        }
    }
}
