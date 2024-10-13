use std::fmt::Display;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum PngmeParseError {
    ChunkTypeErr(String),
    ChunkErr(String),
    PngErr(String),
}

impl Display for PngmeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for PngmeParseError {}

#[test]
fn test() {
    println!(
        "{}, {:?}",
        PngmeParseError::ChunkTypeErr(String::from("123")),
        PngmeParseError::ChunkTypeErr(String::from("345"))
    )
}
