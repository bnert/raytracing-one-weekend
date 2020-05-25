use bytes::{BytesMut, BufMut};
use std::io::{Write, Result};

pub struct ColorRGB {
    r: u32, // 4 bytes
    g: u32,
    b: u32
}

impl ColorRGB {
    pub fn new() -> ColorRGB {
        ColorRGB{ r: 0, g: 0, b: 0 }
    }

    pub fn create(r: u32, g: u32, b: u32) -> ColorRGB {
        ColorRGB{ r: r, g: g, b: b }
    }

    pub fn write_ppm_row(&self, writer: &mut impl Write) -> Result<usize> {
        let mut buf = BytesMut::with_capacity(16);
        let rs = self.r.to_string();
        let gs = self.g.to_string();
        let bs = self.b.to_string();

        buf.put(rs.as_bytes());
        buf.put_u8(b' ');
        buf.put(gs.as_bytes());
        buf.put_u8(b' ');
        buf.put(bs.as_bytes());
        buf.put_u8(b'\n');
        writer.write(&buf)
    }
}
