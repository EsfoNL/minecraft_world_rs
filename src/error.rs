#[derive(Debug)]
pub enum Error {
    Unit,
    Malformed(u32),
    Custom(String),
    FileError(std::io::Error),
    CompressionError,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
