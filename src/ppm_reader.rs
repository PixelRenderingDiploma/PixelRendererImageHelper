use std::fs::read_to_string;
use crate::common::*;

pub struct PPMReader {

}

impl Reader for PPMReader {
    fn read(&self, path: &str) -> std::io::Result<Image> {
        println!("Reading PPM file");

        let content = read_to_string(path)?;

        let mut lines = content.lines();
        let p3 = lines.next().unwrap();
        if p3 != "P3" {
            return Result::Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid PPM file"));
        }

        let mut size_str = lines.next().unwrap().split_whitespace();
        let (width, height) = (size_str.next().unwrap().parse().unwrap(), size_str.next().unwrap().parse().unwrap());
        let _max_value: usize = lines.next().unwrap().parse().unwrap();

        let mut pixels = Vec::with_capacity(height);
        let mut pixel_str = lines.next().unwrap().split_whitespace();
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                let r = match pixel_str.next() { // Pixels data can be placed not in width len on line
                    Some(value) => value.parse().unwrap(),
                    None => {
                        pixel_str = match lines.next() {
                            Some(line) => line.split_whitespace(),
                            None => panic!("Not enough data"),
                        };
                        pixel_str.next().unwrap().parse().unwrap()
                    }
                };

                let g = pixel_str.next().unwrap().parse().unwrap();
                let b = pixel_str.next().unwrap().parse().unwrap();
                row.push(Color::from_rgb(r, g, b));
            }
            pixels.push(row);
        }

        Result::Ok(Image::from_mat(width, height, pixels))
    }
}