use std::io::Write;

use crc::{Crc, CRC_32_ISO_HDLC};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::{common::*, png::{ihdr::{ColorType, IHDR}, Chunk, ChunkType, PNG}};
use crate::binary_serializable::BinarySerializable;

pub struct PNGWriter {

}

impl PNGWriter {
    fn crc(&self, chunk_type: &[u8], chunk_data: &[u8]) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        
        digest.update(chunk_type);
        digest.update(chunk_data);
        
        digest.finalize()
    }
}

impl Writer for PNGWriter {
    fn extension(&self) -> &str {
        "png"
    }

    fn write(&self, image: Image, path: &str) {
        println!("Writing PNG file at path: {}", path);

        let ihdr = IHDR {
            width: image.width() as u32,
            height: image.height() as u32,
            bit_depth: 8,
            color_type: ColorType::RGB,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
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

        let pixels_data: Vec<u8> = image.pixels.iter().flat_map(|row|
            std::iter::once(0).chain(row.iter().flat_map(|color| [color.r, color.g, color.b]))
        ).collect::<Vec<u8>>();
        let mut compressed = Vec::<u8>::new();

        let mut zlibencoder = ZlibEncoder::new(&mut compressed, Compression::default());
        zlibencoder.write_all(&pixels_data).expect("Can't decode");
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