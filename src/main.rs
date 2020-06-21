use std::fs::File;
use std::io::Write;

mod three;
use three::color3::ColorRGB;
use three::vector3::Vector;

mod ray_engine;
use ray_engine::camera::Camera;
use ray_engine::ray::Ray;

mod shapes;

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
    file_h
        .write(&cam.img_height.to_string().as_bytes())
        .unwrap();
    file_h.write(b"\n255\n").unwrap();

    let adj_img_h: f64 = cam.img_height as f64 - 1.0;
    let adj_img_w: f64 = cam.img_width as f64 - 1.0;

    let llc = cam.lower_left_corner.copy();
    for row in (0..cam.img_height).rev() {
        for pixel_in_row in 0..cam.img_width {
            let u = (pixel_in_row as f64) / adj_img_w;
            let v = (row as f64) / adj_img_h;
            let direction = cam
                .lower_left_corner
                .sum(&cam.horizontal.scale(u))
                .sum(&cam.vertical.scale(v))
                .sub(&cam.origin);
            let ray = Ray::create(cam.origin.copy(), direction);
            let rgb = ray.to_rgb();
            match ray.to_rgb().write_ppm_row(&mut file_h) {
                Err(_) => println!("Unable to write row\n"),
                _ => {
                    // noop
                }
            }
        }
    }
}
