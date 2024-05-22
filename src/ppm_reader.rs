use crate::common::*;

pub struct PPMReader {

}

impl Reader for PPMReader {
    fn read(&self, path: &str) -> std::io::Result<Image> {
        println!("Reading PPM file");
        Result::Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented"))
    }
}