use std::{collections::HashMap, fmt::Debug, iter::Peekable};

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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    pub fn from_bytes<'a>(input: &'a [u8]) -> Result<(String, NbtValue)> {
        let iter = input.iter();
        Self::from_bytes_iter(iter)
    }
    pub fn from_bytes_iter<'a, T>(mut iter: T) -> Result<(String, NbtValue)>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
        let name = Self::string_from_iter(&mut iter)?;
        Ok((name, Self::deserialize(&mut iter, tag)?))
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
        let size = Self::i32_from_iter(iter)? as usize;
        let mut output = Vec::with_capacity(size);
        for _ in 0..size {
            output.push(Self::i64_from_iter(iter)?);
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
            let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
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
            buf.push(iter.next().unwrap().to_owned());
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
        let size = Self::i32_from_iter(iter)? as usize;
        let mut output = Vec::with_capacity(size);
        for _ in 0..size {
            output.push(i8::from_be_bytes([iter
                .next()
                .ok_or(Error::Malformed(line!()))?
                .to_owned()]));
        }
        Ok(output)
    }

    fn to_bytes(&self, name: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        match self {
            NbtValue::Byte(v) => {
                buffer.push(TAG_BYTE);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::Short(v) => {
                buffer.push(TAG_SHORT);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::Int(v) => {
                buffer.push(TAG_INT);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::Long(v) => {
                buffer.push(TAG_LONG);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::Float(v) => {
                buffer.push(TAG_FLOAT);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::Double(v) => {
                buffer.push(TAG_DOUBLE);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&v.to_be_bytes());
            }
            NbtValue::ByteArray(v) => {
                buffer.push(TAG_BYTE_ARRAY);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&(v.len() as i32).to_be_bytes());
                buffer.extend(v.iter().map(|f| f.to_be_bytes()[0]));
            }
            NbtValue::String(v) => {
                buffer.push(TAG_BYTE_ARRAY);
                Self::push_name(&mut buffer, name);
                buffer.extend_from_slice(&(v.len() as u16).to_be_bytes());
                buffer.extend_from_slice(v.as_bytes());
            }
            NbtValue::List(v) => {
                buffer.push(TAG_LIST);
            }
            NbtValue::Compound(_) => todo!(),
            NbtValue::IntArray(_) => todo!(),
            NbtValue::LongArray(_) => todo!(),
        }
        buffer
    }
    fn push_name(buffer: &mut Vec<u8>, name: &str) {
        buffer.extend_from_slice(&(name.len() as u32).to_be_bytes());
        buffer.extend_from_slice(name.as_bytes());
    }
}
