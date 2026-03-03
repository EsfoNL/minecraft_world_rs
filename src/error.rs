#[derive(Debug)]
pub enum Error {
    Unit,
    Malformed(u32),
    Custom(String),
    FileError(std::io::Error),
    CompressionError,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Malformed(l0), Self::Malformed(r0)) => l0 == r0,
            (Self::Custom(l0), Self::Custom(r0)) => l0 == r0,
            (Self::FileError(l0), Self::FileError(r0)) => l0.kind() == r0.kind(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
