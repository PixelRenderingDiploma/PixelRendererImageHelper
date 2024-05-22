use crate::common::*;

pub struct Image {
    width: usize,
    height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Color::new(0, 0, 0, 0); width]; height];
        Image { width, height, pixels }
    }

    pub fn from_mat(width: usize, height: usize, pixels: Vec<Vec<Color>>) -> Self {
        Image { width, height, pixels }
    }

    pub fn from_reader<T: Reader>(reader: T, path: &str) -> std::io::Result<Image> {
        reader.read(path)
    }
}

impl Image {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}