use crate::common::Image;

pub trait Writer {
    fn extension(&self) -> &str;
    fn write(&self, image: Image, path: &str);
}