use std::io;
use std::io::{Read, Write};

pub trait BinarySerializable {
    fn read<R: Read>(reader: &mut R) -> io::Result<Self> where Self: Sized;
    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}