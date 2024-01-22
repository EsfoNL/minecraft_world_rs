use minecraft_world::Result;
use std::fmt::Debug;
use std::io::{Read, Write};

use clap::Parser;
use minecraft_world::NbtValue;
use serde::{Deserialize, Serialize};
#[derive(Parser)]
enum Options {
    FromJson,
    ToJson,
    CompressedFromJson,
    CompressedToJson,
}

#[derive(Serialize, Deserialize)]
struct NbtFile {
    name: String,
    nbt: NbtValue,
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

fn main() {
    let cmd = Options::parse();
    match cmd {
        Options::FromJson => serde_json::de::from_reader::<_, NbtFile>(std::io::stdin())
            .unwrap()
            .to_writer(std::io::stdout())
            .unwrap(),
        Options::ToJson => serde_json::ser::to_writer(
            std::io::stdout(),
            &NbtFile::from_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
        Options::CompressedFromJson => serde_json::de::from_reader::<_, NbtFile>(std::io::stdin())
            .unwrap()
            .to_compressed_writer(std::io::stdout())
            .unwrap(),
        Options::CompressedToJson => serde_json::ser::to_writer(
            std::io::stdout(),
            &NbtFile::from_compressed_reader(std::io::stdin()).unwrap(),
        )
        .unwrap(),
    }
}
