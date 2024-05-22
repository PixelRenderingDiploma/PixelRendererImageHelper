pub mod ihdr;
pub mod idat;
pub mod plte;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::binary_serializable::*;
use crate::read_to_string_exact::ReadToStringExact;

const MAGIC: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

pub struct PNG {
    pub chunks: Vec<Chunk>,
}

impl PNG {
    pub fn new() -> PNG {
        PNG {
            chunks: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> PNG {
        let mut png = PNG::new();

        let file = File::open(path).expect("Unable to read png file");
        let mut reader = BufReader::new(file);

        let _magic = reader.read_to_string_lossy_exact(8).expect("Can't read file magic");

        loop {
            match Chunk::read(&mut reader) {
                Ok(chunk) => {
                    let end = chunk.chunk_type == ChunkType::IEND;

                    println!("Chunk: {:?}", chunk);
                    png.chunks.push(chunk);

                    if end { break; }
                }
                Err(e) => {
                    println!("Can't read chunk: {:?}", e);
                    break;
                }
            }
        }
        
        png
    }

    pub fn to_file(&self, path: &str) {
        let file = File::create(path).expect("Unable to read png file");
        let mut writer = BufWriter::new(file);

        let _ = writer.write(&MAGIC);

        for chunk in self.chunks.iter() {
            chunk.write(&mut writer).expect("Can't write chunk");
        }
    }
}

pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("length", &self.length)
            .field("chunk_type", &self.chunk_type)
            .field("crc", &self.crc)
            .finish()
    }
}

impl BinarySerializable for Chunk {
    fn read<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> where Self: Sized {
        let length = reader.read_u32::<BigEndian>()?;
        let chunk_type = match reader.read_to_string_lossy_exact(4)?.as_str() {
            "IHDR" => ChunkType::IHDR,
            "PLTE" => ChunkType::PLTE,
            "IDAT" => ChunkType::IDAT,
            "IEND" => ChunkType::IEND,
            str => ChunkType::Other(str.to_string())
        };

        let mut data = vec![0u8; length as usize]; 
        reader.read_exact(&mut data).expect("msg");

        let crc = reader.read_u32::<BigEndian>()?;
        
        Ok(Chunk {
            length,
            chunk_type,
            data: data,
            crc,
        })
    }

    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u32::<BigEndian>(self.length)?;
        match self.chunk_type {
            ChunkType::IHDR => writer.write_all(b"IHDR")?,
            ChunkType::PLTE => writer.write_all(b"PLTE")?,
            ChunkType::IDAT => writer.write_all(b"IDAT")?,
            ChunkType::IEND => writer.write_all(b"IEND")?,
            ChunkType::Other(ref s) => writer.write_all(s.as_bytes())?
        }

        writer.write_all(&self.data)?;
        writer.write_u32::<BigEndian>(self.crc)?;
        
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,
    
    Other(String),
}