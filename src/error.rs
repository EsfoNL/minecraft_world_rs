#[derive(Debug)]
pub enum Error {
    Unit,
    Malformed(u32),
    Custom(String),
    FileError,
    CompressionError,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(format!("{msg}"))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
