use super::Image;

pub trait Reader {
    fn read(&self, path: &str) -> std::io::Result<Image>;
}