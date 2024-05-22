use std::io;

impl<R: io::Read + ?Sized> ReadToStringExact for R {}

pub trait ReadToStringExact: io::Read {
    #[inline]
    fn read_to_string_exact(&mut self, size: usize) -> io::Result<String> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    #[inline]
    fn read_to_string_lossy_exact(&mut self, size: usize) -> io::Result<String> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}