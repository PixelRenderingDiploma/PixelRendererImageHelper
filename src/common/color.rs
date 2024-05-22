use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn from_array(arr: [u8; 4]) -> Self {
        Color{ r: arr[0], g: arr[1], b: arr[2], a: arr[3] }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut color = Color::black();

        if slice.len() > 0 {
            color.r = slice[0];
        }
        if slice.len() > 1 {
            color.g = slice[1];
        }
        if slice.len() > 2 {
            color.b = slice[2];
        }
        if slice.len() > 3 {
            color.a = slice[3];
        }

        color
    }

    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            (self.r as f32 * rhs) as u8,
            (self.g as f32 * rhs) as u8, 
            (self.b as f32 * rhs) as u8,
            self.a
        )
    }
}