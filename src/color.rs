use glam::Vec4;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    #[allow(unused)]
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[allow(unused)]
    pub fn from_hex(rgba: u32) -> Self {
        let bytes = rgba.to_be_bytes();
        Self {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }

    #[allow(unused)]
    pub fn from_vec4(v: Vec4) -> Self {
        Self {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: (v.w * 255.0) as u8,
        }
    }

    pub fn into_argb(&self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b])
    }

    // Constants
    #[allow(unused)]
    pub fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    #[allow(unused)]
    pub fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }
    #[allow(unused)]
    pub fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}
