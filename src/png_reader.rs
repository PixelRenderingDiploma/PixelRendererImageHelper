use std::io::{Cursor, Read};

use flate2::read::ZlibDecoder;

use crate::binary_serializable::BinarySerializable;
use crate::common::*;
use crate::png::{PNG, ChunkType};
use crate::png::ihdr::IHDR;

pub struct PNGReader {

}

const ADAM7: [(usize, usize, usize, usize); 7] = [
    (0, 0, 8, 8), // Pass 1
    (4, 0, 8, 8), // Pass 2
    (0, 4, 4, 8), // Pass 3
    (2, 0, 4, 4), // Pass 4
    (0, 2, 2, 4), // Pass 5
    (1, 0, 2, 2), // Pass 6
    (0, 1, 1, 2), // Pass 7
];

impl PNGReader {
    fn paeth_predictor(&self, a: u8, b: u8, c: u8) -> u8 {
        let p = (a as i16 + b as i16 - c as i16) as i16;
        let pa = (p - a as i16).abs();
        let pb = (p - b as i16).abs();
        let pc = (p - c as i16).abs();
        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }

    fn unfilter_scanline(&self, filter_type: u8, scanline: &[u8], prev_scanline: Option<&[u8]>, bpp: usize) -> Vec<u8> {
        let mut unfiltered = Vec::with_capacity(scanline.len());
    
        match filter_type {
            0 => unfiltered.extend_from_slice(scanline), // None
            1 => { // Sub
                for i in 0..scanline.len() {
                    let left = if i < bpp { 0 } else { unfiltered[i - bpp] };
                    unfiltered.push(scanline[i].wrapping_add(left));
                }
            }
            2 => { // Up
                for i in 0..scanline.len() {
                    let above = prev_scanline.map_or(0, |prev| prev[i]);
                    unfiltered.push(scanline[i].wrapping_add(above));
                }
            }
            3 => { // Average
                for i in 0..scanline.len() {
                    let left = if i < bpp { 0 } else { unfiltered[i - bpp] };
                    let above = prev_scanline.map_or(0, |prev| prev[i]);
                    let avg = ((left as u16 + above as u16) / 2) as u8;
                    unfiltered.push(scanline[i].wrapping_add(avg));
                }
            }
            4 => { // Paeth
                for i in 0..scanline.len() {
                    let left = if i < bpp { 0 } else { unfiltered[i - bpp] };
                    let above = prev_scanline.map_or(0, |prev| prev[i]);
                    let above_left = if i < bpp { 0 } else { prev_scanline.map_or(0, |prev| prev[i - bpp]) };
                    unfiltered.push(scanline[i].wrapping_add(self.paeth_predictor(left, above, above_left)));
                }
            }
            _ => panic!("Unknown filter type"),
        }
    
        unfiltered
    }
}

impl Reader for PNGReader {
    fn read(&self, path: &str) -> std::io::Result<Image> {
        println!("Reading PNG file at: {}", path);
        let png = PNG::from_file(path);
        
        let ihdr_chunk = png.chunks.iter().find(|chunk| chunk.chunk_type == ChunkType::IHDR).unwrap();
        let mut cursor = Cursor::new(&ihdr_chunk.data);
        let ihdr = IHDR::read(&mut cursor).unwrap();
        println!("{:?}", ihdr);

        let mut concatenated = Vec::<u8>::new();
        for chunk in png.chunks {
            match chunk.chunk_type {
                ChunkType::IDAT => {
                    concatenated.extend(&chunk.data);
                }
                _ => {}
            }
        }

        let mut zlibdecoder = ZlibDecoder::<&[u8]>::new_with_buf(&concatenated, vec![0; 32 * 1024]);
        let mut decompressed = Vec::<u8>::new();
        zlibdecoder.read_to_end(&mut decompressed).expect("Can't decode");

        let width = ihdr.width as usize;
        let height = ihdr.height as usize;
        let bytes_per_pixel = ihdr.bytes_per_pixel();

        let scanline_length = 1 + width * bytes_per_pixel;

        let mut pixels: Vec<Vec<Color>> = Vec::with_capacity(height);
        let mut prev_scanline: Option<Vec<u8>> = None;

        if ihdr.interlace_method == 1 {
            pixels = vec![vec![Color::new(0, 0, 0, 0); width]; height];
            
            let mut offset = 0;

            for &(x_start, y_start, x_step, y_step) in ADAM7.iter() {
                let pass_width = (width - x_start + x_step - 1) / x_step;
                let pass_height = (height - y_start + y_step - 1) / y_step;
        
                for y in 0..pass_height {
                    let filter_type = decompressed[offset];
                    offset += 1;
        
                    let scanline = &decompressed[offset..offset + pass_width * bytes_per_pixel];
                    offset += pass_width * bytes_per_pixel;
        
                    let unfiltered_scanline = self.unfilter_scanline(filter_type, scanline, prev_scanline.as_deref(), bytes_per_pixel);
        
                    for x in 0..pass_width {
                        let pixel_index = x * bytes_per_pixel;
                        let output_x = x_start + x * x_step;
                        let output_y = y_start + y * y_step;

                        if output_x < width && output_y < height {
                            let color = Color::from_slice(&unfiltered_scanline[pixel_index..pixel_index + 3]);
                            pixels[output_y][output_x] = color;
                        }
                    }
        
                    prev_scanline = Some(unfiltered_scanline);
                }
            }
        } else {
            for scanline_start in (0..decompressed.len()).step_by(scanline_length) {
                let filter_type = decompressed[scanline_start];

                let scanline = &decompressed[scanline_start + 1..scanline_start + scanline_length];
                let unfiltered_scanline = self.unfilter_scanline(filter_type, scanline, prev_scanline.as_deref(), bytes_per_pixel);
                let mut row: Vec<Color> = Vec::with_capacity(width);
    
                for chunk in unfiltered_scanline.chunks(bytes_per_pixel) {
                    row.push(Color::from_slice(chunk));
                }
        
                prev_scanline = Some(unfiltered_scanline);
                pixels.push(row);
            }
        }

        Result::Ok(Image::from_mat(width, height, pixels))
    }
}