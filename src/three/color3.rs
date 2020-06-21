use crate::three::vector3::Vector;
use bytes::{BufMut, BytesMut};
use std::io::{Result, Write};

pub struct ColorRGB {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl ColorRGB {
    pub fn new() -> ColorRGB {
        ColorRGB { r: 0, g: 0, b: 0 }
    }

    pub fn from_u32(r: u32, g: u32, b: u32) -> ColorRGB {
        ColorRGB { r: r, g: g, b: b }
    }

    pub fn from_f64(r: f64, g: f64, b: f64) -> ColorRGB {
        let (scp_r, scp_g, scp_b) = ColorRGB::scope_f64(&r, &g, &b);
        ColorRGB::from_u32(scp_r, scp_g, scp_b)
    }

    pub fn from_vector3(v: &Vector) -> ColorRGB {
        ColorRGB::from_f64(v.x, v.y, v.z)
    }

    pub fn write_ppm_row(&self, writer: &mut impl Write) -> Result<usize> {
        let mut buf = BytesMut::with_capacity(16);
        let color_string = self.as_string();
        buf.put(color_string.as_bytes());
        writer.write(&buf)
    }

    fn as_string(&self) -> String {
        let mut s = String::with_capacity(12);
        let ColorRGB { r, g, b } = self;
        s.push_str(&r.to_string());
        s.push(' ');
        s.push_str(&g.to_string());
        s.push(' ');
        s.push_str(&b.to_string());
        s.push('\n');
        s
    }

    fn scope_f64(r: &f64, g: &f64, b: &f64) -> (u32, u32, u32) {
        let scp_factor = 255.99;
        (
            (scp_factor * r) as u32,
            (scp_factor * g) as u32,
            (scp_factor * b) as u32,
        )
    }
}
