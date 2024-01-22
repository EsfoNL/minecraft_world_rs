use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use flate2::Compression;

use crate::{Error, Result};

const TAG_END: u8 = 0;
const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, Serialize, Deserialize)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(NbtList),
    Compound(HashMap<String, NbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NbtList {
    ByteList(Vec<i8>),
    ShortList(Vec<i16>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
    FloatList(Vec<f32>),
    DoubleList(Vec<f64>),
    ByteArrayList(Vec<Vec<i8>>),
    StringList(Vec<String>),
    ListList(Vec<NbtList>),
    CompoundList(Vec<HashMap<String, NbtValue>>),
    IntArrayList(Vec<Vec<i32>>),
    LongArrayList(Vec<Vec<i64>>),
    EmptyList,
}

impl NbtValue {
    pub fn from_reader<T>(reader: T) -> Result<(String, NbtValue)>
    where
        T: Read + Debug,
    {
        let binding = reader
            .bytes()
            .map(|e| e.map_err(|e| Error::FileError(e)))
            .collect::<Result<Vec<u8>>>()?;
        let mut iter = binding.iter();
        let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
        let name = Self::string_from_iter(&mut iter)?;
        Ok((name, Self::deserialize(&mut iter, tag)?))
    }

    pub fn from_compressed_reader<T>(reader: T) -> Result<(String, NbtValue)>
    where
        T: Read + Debug,
    {
        let bytes = flate2::read::GzDecoder::new(reader);
        Self::from_reader(bytes)
    }

    fn deserialize<'a, T>(iter: &mut T, tag: u8) -> Result<NbtValue>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        match tag {
            TAG_BYTE => Ok(NbtValue::Byte(Self::i8_from_iter(iter)?)),
            TAG_SHORT => Ok(NbtValue::Short(Self::i16_from_iter(iter)?)),
            TAG_INT => Ok(NbtValue::Int(Self::i32_from_iter(iter)?)),
            TAG_LONG => Ok(NbtValue::Long(Self::i64_from_iter(iter)?)),
            TAG_FLOAT => Ok(NbtValue::Float(Self::f32_from_iter(iter)?)),
            TAG_DOUBLE => Ok(NbtValue::Double(Self::f64_from_iter(iter)?)),
            TAG_BYTE_ARRAY => Ok(NbtValue::ByteArray(Self::byte_array_from_iter(iter)?)),
            TAG_STRING => Ok(NbtValue::String(Self::string_from_iter(iter)?)),
            TAG_LIST => Ok(NbtValue::List(Self::list_from_iter(iter)?)),
            TAG_COMPOUND => Ok(NbtValue::Compound(Self::compound_from_iter(iter)?)),
            TAG_INT_ARRAY => Ok(NbtValue::IntArray(Self::int_array_from_iter(iter)?)),
            TAG_LONG_ARRAY => Ok(NbtValue::LongArray(Self::long_array_from_iter(iter)?)),
            _ => Err(Error::Malformed(line!())),
        }
    }

    fn long_array_from_iter<'a, T>(iter: &mut T) -> Result<Vec<i64>>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let size = Self::i32_from_iter(&mut *iter)? as usize;
        let mut output = Vec::with_capacity(size);
        for _ in 0..size {
            output.push(Self::i64_from_iter(&mut *iter)?);
        }
        Ok(output)
    }

    fn int_array_from_iter<'a, T>(iter: &mut T) -> Result<Vec<i32>>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let size = Self::i32_from_iter(iter)? as usize;
        let mut output = Vec::with_capacity(size);
        for _ in 0..size {
            output.push(Self::i32_from_iter(iter)?)
        }
        Ok(output)
    }

    fn compound_from_iter<'a, T>(iter: &mut T) -> Result<HashMap<String, NbtValue>>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let mut output = HashMap::new();
        loop {
            let tag = (iter).next().ok_or(Error::Malformed(line!()))?.to_owned();
            if tag == TAG_END {
                break;
            } else {
                let key = Self::string_from_iter(iter)?;
                let value = Self::deserialize(iter, tag)?;
                output.insert(key, value);
            }
        }
        Ok(output)
    }

    fn list_from_iter<'a, T>(iter: &mut T) -> Result<NbtList>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
        let size = Self::i32_from_iter(iter)? as usize;
        match tag {
            TAG_BYTE => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::i8_from_iter(iter)?);
                }
                Ok(NbtList::ByteList(output))
            }
            TAG_SHORT => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::i16_from_iter(iter)?);
                }
                Ok(NbtList::ShortList(output))
            }
            TAG_INT => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::i32_from_iter(iter)?);
                }
                Ok(NbtList::IntList(output))
            }
            TAG_LONG => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::i64_from_iter(iter)?);
                }
                Ok(NbtList::LongList(output))
            }
            TAG_FLOAT => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::f32_from_iter(iter)?);
                }
                Ok(NbtList::FloatList(output))
            }
            TAG_DOUBLE => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::f64_from_iter(iter)?);
                }
                Ok(NbtList::DoubleList(output))
            }
            TAG_BYTE_ARRAY => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::byte_array_from_iter(iter)?);
                }
                Ok(NbtList::ByteArrayList(output))
            }
            TAG_STRING => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::string_from_iter(iter)?);
                }
                Ok(NbtList::StringList(output))
            }
            TAG_LIST => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::list_from_iter(iter)?);
                }
                Ok(NbtList::ListList(output))
            }
            TAG_COMPOUND => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::compound_from_iter(iter)?);
                }
                Ok(NbtList::CompoundList(output))
            }
            TAG_INT_ARRAY => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::int_array_from_iter(iter)?);
                }
                Ok(NbtList::IntArrayList(output))
            }
            TAG_LONG_ARRAY => {
                let mut output = Vec::new();
                for _ in 0..size {
                    output.push(Self::long_array_from_iter(iter)?);
                }
                Ok(NbtList::LongArrayList(output))
            }
            TAG_END => Ok(NbtList::EmptyList),
            _ => Err(Error::Malformed(line!())),
        }
    }

    /// Call after getting tag type
    fn string_from_iter<'a, T>(iter: &mut T) -> Result<String>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let length = Self::u16_from_iter(iter)? as usize;
        let mut buf = Vec::with_capacity(length);
        for _ in 0..length {
            buf.push(iter.next().ok_or(Error::Malformed(line!()))?.to_owned());
        }
        let string = String::from_utf8(buf).map_err(|_| Error::Malformed(line!()))?;
        Ok(string)
    }

    fn u16_from_iter<'a, T>(iter: &mut T) -> Result<u16>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(u16::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn i8_from_iter<'a, T>(iter: &mut T) -> Result<i8>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(i8::from_be_bytes([iter
            .next()
            .ok_or(Error::Malformed(line!()))?
            .to_owned()]))
    }

    fn i16_from_iter<'a, T>(iter: &mut T) -> Result<i16>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(i16::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn i32_from_iter<'a, T>(iter: &mut T) -> Result<i32>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(i32::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn i64_from_iter<'a, T>(iter: &mut T) -> Result<i64>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(i64::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn f64_from_iter<'a, T>(iter: &mut T) -> Result<f64>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(f64::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn f32_from_iter<'a, T>(iter: &mut T) -> Result<f32>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        Ok(f32::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]))
    }

    fn byte_array_from_iter<'a, T>(iter: &mut T) -> Result<Vec<i8>>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        eprintln!("{iter:?}");
        let size = Self::i32_from_iter(iter)? as usize;
        eprintln!("{size}");
        let mut output = Vec::with_capacity(size);
        for _ in 0..size {
            output.push(i8::from_be_bytes([iter
                .next()
                .ok_or(Error::Malformed(line!()))?
                .to_owned()]));
        }
        Ok(output)
    }

    pub fn to_writer<T>(&self, name: &str, mut writer: T) -> std::io::Result<()>
    where
        T: Write,
    {
        self.serialize(name, &mut writer)?;
        writer.flush()?;
        Ok(())
    }

    pub fn to_compressed_writer<T>(&self, name: &str, writer: T) -> std::io::Result<()>
    where
        T: Write,
    {
        let mut encoder = flate2::write::GzEncoder::new(writer, Compression::new(9));
        self.serialize(name, &mut encoder)?;
        encoder.flush()?;
        drop(encoder);
        Ok(())
    }

    pub fn serialize<T>(&self, name: &str, buffer: &mut T) -> std::io::Result<()>
    where
        T: Write,
    {
        match self {
            NbtValue::Byte(v) => {
                buffer.write_all(&[TAG_BYTE])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::Short(v) => {
                buffer.write_all(&[TAG_SHORT])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::Int(v) => {
                buffer.write_all(&[TAG_INT])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::Long(v) => {
                buffer.write_all(&[TAG_LONG])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::Float(v) => {
                buffer.write_all(&[TAG_FLOAT])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::Double(v) => {
                buffer.write_all(&[TAG_DOUBLE])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&v.to_be_bytes())?;
            }
            NbtValue::ByteArray(v) => {
                buffer.write_all(&[TAG_BYTE_ARRAY])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                buffer.write_all(
                    &v.iter()
                        .map(|f| f.to_be_bytes())
                        .flatten()
                        .collect::<Vec<_>>(),
                )?;
            }
            NbtValue::String(v) => {
                buffer.write_all(&[TAG_STRING])?;
                Self::push_string(buffer, name)?;
                Self::push_string(buffer, v)?;
            }
            NbtValue::List(v) => {
                buffer.write_all(&[TAG_LIST])?;
                Self::push_string(buffer, name)?;
                Self::serialize_list(buffer, v)?;
            }
            NbtValue::Compound(v) => {
                buffer.write_all(&[TAG_COMPOUND])?;
                Self::push_string(buffer, name)?;
                for (key, value) in v.iter() {
                    value.serialize(key, buffer)?;
                }
                buffer.write_all(&[TAG_END])?;
            }
            NbtValue::IntArray(v) => {
                buffer.write_all(&[TAG_INT_ARRAY])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                buffer.write_all(
                    &v.iter()
                        .map(|f| f.to_be_bytes())
                        .flatten()
                        .collect::<Vec<_>>(),
                )?;
            }
            NbtValue::LongArray(v) => {
                buffer.write_all(&[TAG_INT_ARRAY])?;
                Self::push_string(buffer, name)?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                buffer.write_all(
                    &v.iter()
                        .map(|f| f.to_be_bytes())
                        .flatten()
                        .collect::<Vec<_>>(),
                )?;
            }
        }
        Ok(())
    }

    fn serialize_list<T>(buffer: &mut T, list: &NbtList) -> std::io::Result<()>
    where
        T: Write,
    {
        match list {
            NbtList::ByteList(v) => {
                buffer.write_all(&[TAG_BYTE])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::ShortList(v) => {
                buffer.write_all(&[TAG_SHORT])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::IntList(v) => {
                buffer.write_all(&[TAG_INT])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::LongList(v) => {
                buffer.write_all(&[TAG_LONG])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::FloatList(v) => {
                buffer.write_all(&[TAG_FLOAT])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::DoubleList(v) => {
                buffer.write_all(&[TAG_DOUBLE])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v.iter() {
                    buffer.write_all(&i.to_be_bytes())?;
                }
            }
            NbtList::ByteArrayList(v) => {
                buffer.write_all(&[TAG_BYTE_ARRAY])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                    buffer.write_all(
                        &i.iter()
                            .map(|e| e.to_be_bytes().into_iter())
                            .flatten()
                            .collect::<Vec<_>>(),
                    )?;
                }
            }
            NbtList::StringList(v) => {
                buffer.write_all(&[TAG_STRING])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    Self::push_string(buffer, &i)?;
                }
            }
            NbtList::ListList(v) => {
                buffer.write_all(&[TAG_LIST])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    Self::serialize_list(buffer, i)?;
                }
            }
            NbtList::CompoundList(v) => {
                buffer.write_all(&[TAG_COMPOUND])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    for (key, value) in i.iter() {
                        value.serialize(key, buffer)?;
                    }
                    buffer.write_all(&[TAG_END])?;
                }
            }
            NbtList::IntArrayList(v) => {
                buffer.write_all(&[TAG_INT_ARRAY])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                    buffer.write_all(
                        &i.iter()
                            .map(|e| e.to_be_bytes().into_iter())
                            .flatten()
                            .collect::<Vec<_>>(),
                    )?;
                }
            }
            NbtList::LongArrayList(v) => {
                buffer.write_all(&[TAG_INT_ARRAY])?;
                buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                for i in v {
                    buffer.write_all(&(v.len() as i32).to_be_bytes())?;
                    buffer.write_all(
                        &i.iter()
                            .map(|e| e.to_be_bytes().into_iter())
                            .flatten()
                            .collect::<Vec<_>>(),
                    )?;
                }
            }
            NbtList::EmptyList => {
                buffer.write_all(&[TAG_END])?;
                buffer.write_all(&[0])?;
                buffer.write_all(&[0])?;
                buffer.write_all(&[0])?;
                buffer.write_all(&[0])?;
            }
        }
        Ok(())
    }

    fn push_string<T>(buffer: &mut T, name: &str) -> std::io::Result<()>
    where
        T: Write,
    {
        buffer.write_all(&(name.len() as u16).to_be_bytes())?;
        buffer.write_all(name.as_bytes())?;
        Ok(())
    }
}
