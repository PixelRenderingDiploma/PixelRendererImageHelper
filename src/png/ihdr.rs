use crate::binary_serializable::BinarySerializable;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct IHDR {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

impl IHDR {
    pub fn bytes_per_pixel(&self) -> usize {
        match self.color_type {
            ColorType::Grayscale => self.bit_depth as usize / 8,
            ColorType::RGB => 3 * self.bit_depth as usize / 8,
            ColorType::Palette => self.bit_depth as usize / 8,
            ColorType::GrayscaleAlpha => 2 * self.bit_depth as usize / 8,
            ColorType::RGBA => 4 * self.bit_depth as usize / 8,
        }
    }

    pub fn has_alpha(&self) -> bool {
        match self.color_type {
            ColorType::GrayscaleAlpha | ColorType::RGBA => true,
            _ => false,
        }
    }
}

impl BinarySerializable for IHDR {
    fn read<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> where Self: Sized {
        let width = reader.read_u32::<BigEndian>()?;
        let height = reader.read_u32::<BigEndian>()?;
        let bit_depth = reader.read_u8()?;
        let color_type =  ColorType::try_from(reader.read_u8()?).unwrap();
        let compression_method = reader.read_u8()?;
        let filter_method = reader.read_u8()?;
        let interlace_method = reader.read_u8()?;
        
        Ok(IHDR {
            width,
            height,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        })
    }

    fn write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u32::<BigEndian>(self.width)?;
        writer.write_u32::<BigEndian>(self.height)?;
        writer.write_u8(self.bit_depth)?;
        writer.write_u8(self.color_type as u8)?;
        writer.write_u8(self.compression_method)?;
        writer.write_u8(self.filter_method)?;
        writer.write_u8(self.interlace_method)?;
        
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ColorType {
    Grayscale = 0,
    RGB = 2,
    Palette = 3,
    GrayscaleAlpha = 4,
    RGBA = 6,
}

impl std::convert::TryFrom<u8> for ColorType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorType::Grayscale),
            2 => Ok(ColorType::RGB),
            3 => Ok(ColorType::Palette),
            4 => Ok(ColorType::GrayscaleAlpha),
            6 => Ok(ColorType::RGBA),
            _ => Err(()),
        }
    }
}