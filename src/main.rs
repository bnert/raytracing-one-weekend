use std::fs::File;
use std::io::{Write};

mod vector3;
use vector3::Vector;
use vector3::color3::ColorRGB;

static IMG_W: i32 = 256;
static IMG_H: i32 = 256;

fn main() {
    // Open the file
    let mut file_h = File::create("test.ppm").unwrap();
    file_h.write(b"P3\n").unwrap();
    file_h.write(&IMG_W.to_string().as_bytes()).unwrap();
    file_h.write(b" ").unwrap();
    file_h.write(&IMG_H.to_string().as_bytes()).unwrap();
    file_h.write(b"\n255\n").unwrap();

    let adj_img_h: i32 = IMG_H - 1;
    let adj_img_w: i32 = IMG_W - 1;
    for x in 0..IMG_H {
        for y in 0..IMG_W {
            let r: f64 = (y as f64) / adj_img_w as f64;
            let g: f64 = (adj_img_h as f64 - x as f64) / adj_img_h as f64;
            let b: f64 = 0.25;

            let scoped_r: i32 = (255.999 * r) as i32;
            let scoped_g: i32 = (255.999 * g) as i32;
            let scoped_b: i32 = (255.999 * b) as i32;

            file_h.write(&scoped_r.to_string().as_bytes()).unwrap();
            file_h.write(b" ").unwrap();
            file_h.write(&scoped_g.to_string().as_bytes()).unwrap();
            file_h.write(b" ").unwrap();
            file_h.write(&scoped_b.to_string().as_bytes()).unwrap();
            file_h.write(b"\n").unwrap();
        }
    }
}
