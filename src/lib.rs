pub mod common;
pub mod ppm_reader;
pub mod ppm_writer;


mod read_to_string_exact;
mod binary_serializable;

#[cfg(test)]
mod tests {
    use super::*;
    use common::Reader;
    use common::Writer;    

    #[test]
    fn ppm_write() {
        let image = common::Image::from_mat(8, 8, vec![vec![common::Color::from_rgb(100, 0, 100); 8]; 8]);
        let writer = ppm_writer::PPMWriter {};
        writer.write(image, "output/image.ppm");
    }
    
}
