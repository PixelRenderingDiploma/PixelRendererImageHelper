pub mod common;
pub mod ppm_reader;
pub mod ppm_writer;

pub mod png;
pub mod png_reader;
pub mod png_writer;

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

    #[test]
    fn ppm_read_ppm_write() {
        let ppm_reader = ppm_reader::PPMReader {};
        let image = ppm_reader.read("resources/6pixels.ppm").unwrap();

        let writer = ppm_writer::PPMWriter {};
        writer.write(image, "output/image.ppm");
    }
    
    #[test]
    fn png_read_ppm_write() {
        let png_reader = png_reader::PNGReader {};
        let image = png_reader.read("resources/defiltered.png").unwrap();

        let ppm_writer = ppm_writer::PPMWriter {};
        ppm_writer.write(image, "output/image.ppm");
    }

    #[test]
    fn png_read_ppm_write_transperent() {
        let png_reader = png_reader::PNGReader {};
        let image = png_reader.read("resources/PNG_transparency_demonstration_1.png").unwrap();

        let ppm_writer = ppm_writer::PPMWriter {};
        ppm_writer.write(image, "output/image.ppm");
    }

    #[test]
    fn png_read_ppm_write_interlace() {
        let png_reader = png_reader::PNGReader {};
        let image = png_reader.read("resources/pnglogo-grr.png").unwrap();

        let ppm_writer = ppm_writer::PPMWriter {};
        ppm_writer.write(image, "output/image.ppm");
    }
    
    #[test]
    fn ppm_read_png_write() {
        let ppm_reader = ppm_reader::PPMReader {};
        let image = ppm_reader.read("resources/6pixels.ppm").unwrap();

        let writer = png_writer::PNGWriter {};
        writer.write(image, "output/image.png");
    }
}
