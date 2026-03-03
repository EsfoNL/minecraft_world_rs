mod error;
mod nbt_value;
mod pylib;
mod world;

use std::{
    fmt::Debug,
    io::{Read, Write},
};

pub use error::{Error, Result};
pub use nbt_value::{Map, NbtList, NbtValue};
pub use pylib::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize)]
pub struct NbtFile {
    pub name: String,
    pub nbt: NbtValue,
}

impl NbtFile {
    pub fn to_writer<T>(&self, writer: T) -> std::io::Result<()>
    where
        T: Write,
    {
        self.nbt.to_writer(&self.name, writer)
    }
    pub fn to_compressed_writer<T>(&self, writer: T) -> std::io::Result<()>
    where
        T: Write,
    {
        self.nbt.to_compressed_writer(&self.name, writer)
    }

    pub fn from_reader<T>(reader: T) -> Result<Self>
    where
        T: Read + Debug,
    {
        let (name, nbt) = NbtValue::from_reader(reader)?;
        Ok(Self { name, nbt })
    }
    pub fn from_compressed_reader<T>(reader: T) -> Result<Self>
    where
        T: Read + Debug,
    {
        let (name, nbt) = NbtValue::from_compressed_reader(reader)?;
        Ok(Self { name, nbt })
    }
}
