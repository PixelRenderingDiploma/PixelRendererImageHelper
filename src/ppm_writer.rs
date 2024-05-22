use std::fs;

use crate::common::*;

pub struct PPMWriter {

}

impl Writer for PPMWriter {
    fn extension(&self) -> &str {
        "ppm"
    }

    fn write(&self, image: Image, path: &str) {
        println!("Writing PPM file at path: {}", path);

        let header_size = 2 + 3 + 3;
        let data_size = image.width() * image.height() * 3 * 4;
        let mut data = String::with_capacity(header_size + data_size);
        
        data.push_str("P3\n");
        data.push_str((image.width().to_string() + " " + image.height().to_string().as_str() + "\n").as_str());
        data.push_str((std::u8::MAX.to_string() + "\n").as_str());

        for i in 0..image.height() {
            for j in 0..image.width() {
                data.push_str(image.pixels[i][j].r.to_string().as_str());
                data.push_str(" ");
                data.push_str(image.pixels[i][j].g.to_string().as_str());
                data.push_str(" ");
                data.push_str(image.pixels[i][j].b.to_string().as_str());
                data.push_str(" ");
            }
            data.push_str("\n");
        }

        fs::write(path, data).expect("Can't save output PPM file");
    }
}