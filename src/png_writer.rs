use std::io::Write;

use crc::{Crc, CRC_32_ISO_HDLC};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::{common::*, png::{ihdr::{ColorType, IHDR}, Chunk, ChunkType, PNG, paeth_predictor}};
use crate::binary_serializable::BinarySerializable;

pub struct Settings {
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub interlace_method: u8,
}

pub struct PNGWriter {
    pub settings: Settings
}

impl PNGWriter {
    fn crc(&self, chunk_type: &[u8], chunk_data: &[u8]) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        
        digest.update(chunk_type);
        digest.update(chunk_data);
        
        digest.finalize()
    }

    fn filter_scanline(&self, filter_type: u8, scanline: &[u8], prev_scanline: &[u8], bpp: usize) -> Vec<u8> {
        let mut filtered = Vec::with_capacity(scanline.len());

        match filter_type {
            0 => filtered = scanline.to_vec(), // None
            1 => { // Sub
                for (i, &pixel) in scanline.iter().enumerate() {
                    let sub = if i >= bpp { pixel.wrapping_sub(scanline[i - bpp]) } else { pixel };
                    filtered.push(sub);
                }
            }
            2 => { // Up
                for (pixel, &prev_pixel) in scanline.iter().zip(prev_scanline.iter()) {
                    filtered.push(pixel.wrapping_sub(prev_pixel));
                }
            }
            3 => { // Average
                for (i, &pixel) in scanline.iter().enumerate() {
                    let left = if i >= bpp { scanline[i - bpp] } else { 0 };
                    let above = prev_scanline[i];
                    let avg = ((left as u16 + above as u16) / 2) as u8;
                    filtered.push(pixel.wrapping_sub(avg));
                }
            }
            4 => { // Paeth
                for (i, &pixel) in scanline.iter().enumerate() {
                    let left = if i >= bpp { scanline[i - bpp] } else { 0 };
                    let above = prev_scanline[i];
                    let upper_left = if i >= bpp { prev_scanline[i - bpp] } else { 0 };
                    let paeth = paeth_predictor(left, above, upper_left);
                    filtered.push(pixel.wrapping_sub(paeth));
                }
            }
            _ => panic!("Unknown filter type"),
        }
    
        filtered
    }

    fn compressed_data_len(&self, data: &[u8]) -> usize {
        let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).expect("Failed to write data");
        let compressed_data = encoder.finish().expect("Failed to finish compression");
        compressed_data.len()
    }
}

impl Writer for PNGWriter {
    fn extension(&self) -> &str {
        "png"
    }

    fn write(&self, image: Image, path: &str) {
        println!("Writing PNG file at path: {}", path);

        let width = image.width();
        let height = image.height();

        let ihdr = IHDR {
            width: width as u32,
            height: height as u32,
            bit_depth: self.settings.bit_depth,
            color_type: self.settings.color_type,
            compression_method: 0,
            filter_method: 0,
            interlace_method: self.settings.interlace_method,
        };

        let mut ihdr_data: Vec<u8> = Vec::new();
        ihdr.write(&mut ihdr_data).unwrap();

        let ihdr_crc = self.crc(b"IHDR", &ihdr_data); 
        let ihdr_chunk = Chunk {
            length: 13,
            chunk_type: ChunkType::IHDR,
            data: ihdr_data,
            crc: ihdr_crc,
        };

        let pixels_data: Vec<Vec<u8>> = image.pixels.iter().map(|row|
            row.iter().flat_map(|color|
                return match self.settings.color_type {
                    ColorType::Grayscale      => vec![color.r],
                    ColorType::RGB            => vec![color.r, color.g, color.b],
                    ColorType::Palette        => unimplemented!("Palette color type is not supported yet"),
                    ColorType::GrayscaleAlpha => vec![color.r, color.a],
                    ColorType::RGBA           => vec![color.r, color.g, color.b, color.a],
                }
            ).collect::<Vec<u8>>()
        ).collect::<Vec<Vec<u8>>>();

        let bpp = ihdr.bytes_per_pixel();
        let mut finilized = Vec::<u8>::with_capacity((image.width() + 1) * image.height() * bpp); // E.g. filtered and optionally interlaced
        if ihdr.interlace_method == 1 {
            panic!("Interlaced PNGs are not supported yet");
        } else {
            for (i, row) in pixels_data.iter().enumerate() {                
                let prev_row = if i == 0 {
                    vec![0; row.len()]
                } else {
                    pixels_data[i-1].to_vec()
                };
        
                let mut best_filter = 0;
                let mut best_filtered_row = self.filter_scanline(0, row, &prev_row, bpp);
                let mut best_compressed_size = self.compressed_data_len(&best_filtered_row);
        
                for filter_type in 1..=4 {
                    let filtered_row = self.filter_scanline(filter_type, row, &prev_row, bpp);
                    let compressed_size = self.compressed_data_len(&filtered_row);
                    if compressed_size < best_compressed_size {
                        best_filter = filter_type;
                        best_filtered_row = filtered_row;
                        best_compressed_size = compressed_size;
                    }
                }
        
                finilized.push(best_filter as u8);
                finilized.extend(best_filtered_row);
            }
        }

        let mut compressed = Vec::<u8>::new();
        let mut zlibencoder = ZlibEncoder::new(&mut compressed, Compression::default());
        zlibencoder.write_all(&finilized).expect("Can't decode");
        zlibencoder.finish().expect("Can't finish");

        let idat = compressed;
        let idat_crc = self.crc(b"IDAT", &idat); 
        let idat_chunk = Chunk {
            length: idat.len() as u32,
            chunk_type: ChunkType::IDAT,
            data: idat,
            crc: idat_crc,
        };

        let iend_crc = self.crc(b"IEND", &[]); 
        let iend_chunk = Chunk {
            length: 0,
            chunk_type: ChunkType::IEND,
            data: vec![],
            crc: iend_crc,
        };
        
        let png = PNG {
            chunks: vec![ihdr_chunk, idat_chunk, iend_chunk],
        };

        png.to_file(path)
    }
}