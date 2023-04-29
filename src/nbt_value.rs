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
    List(Vec<NbtValue>),
    Compound(HashMap<String, NbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
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
        let name = Self::get_tag_name(&mut iter)?;
        Ok((name, Self::deserialize(&mut iter, tag)?))
    }

    fn deserialize<'a, T>(iter: &mut T, tag: u8) -> Result<NbtValue>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        match tag {
            TAG_BYTE => Ok(NbtValue::Byte(i8::from_be_bytes([iter
                .next()
                .ok_or(Error::Malformed(line!()))?
                .to_owned()]))),
            TAG_SHORT => Ok(NbtValue::Short(i16::from_be_bytes([
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            ]))),
            TAG_INT => Ok(NbtValue::Int(Self::i32_from_iter(iter)?)),
            TAG_LONG => Ok(NbtValue::Long(i64::from_be_bytes([
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            ]))),
            TAG_FLOAT => Ok(NbtValue::Float(f32::from_be_bytes([
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            ]))),
            TAG_DOUBLE => Ok(NbtValue::Double(f64::from_be_bytes([
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            ]))),
            TAG_BYTE_ARRAY => {
                let size = Self::i32_from_iter(iter)? as usize;
                let mut output = Vec::with_capacity(size);
                for _ in 0..size {
                    output.push(i8::from_be_bytes([iter
                        .next()
                        .ok_or(Error::Malformed(line!()))?
                        .to_owned()]));
                }
                Ok(NbtValue::ByteArray(output))
            }
            TAG_STRING => {
                let size = u16::from_be_bytes([
                    iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                    iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
                ]) as usize;
                let mut output = Vec::with_capacity(size);
                for _ in 0..size {
                    output.push(iter.next().ok_or(Error::Malformed(line!()))?.to_owned());
                }
                Ok(NbtValue::String(
                    String::from_utf8(output).map_err(|_| Error::Malformed(line!()))?,
                ))
            }
            TAG_LIST => {
                let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
                let size = Self::i32_from_iter(iter)? as usize;
                let mut output = Vec::with_capacity(size);
                for _ in 0..size {
                    output.push(Self::deserialize(iter, tag)?)
                }
                Ok(NbtValue::List(output))
            }
            TAG_COMPOUND => {
                let mut output = HashMap::new();
                loop {
                    let tag = iter.next().ok_or(Error::Malformed(line!()))?.to_owned();
                    if tag == TAG_END {
                        break;
                    } else {
                        let key = Self::get_tag_name(iter)?;
                        let value = Self::deserialize(iter, tag)?;
                        output.insert(key, value);
                    }
                }
                Ok(NbtValue::Compound(output))
            }
            TAG_INT_ARRAY => {
                let size = Self::i32_from_iter(iter)? as usize;
                let mut output = Vec::with_capacity(size);
                for _ in 0..size {
                    output.push(Self::i32_from_iter(iter)?)
                }
                Ok(NbtValue::IntArray(output))
            }
            TAG_LONG_ARRAY => {
                todo!("long array")
            }
            _ => Err(Error::Malformed(line!())),
        }
    }

    /// Call after getting tag type
    fn get_tag_name<'a, T>(iter: &mut T) -> Result<String>
    where
        T: Iterator<Item = &'a u8> + Debug,
    {
        let length = u16::from_be_bytes([
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
            iter.next().ok_or(Error::Malformed(line!()))?.to_owned(),
        ]) as usize;
        let mut buf = Vec::with_capacity(length);
        for _ in 0..length {
            buf.push(iter.next().unwrap().to_owned());
        }
        let string = String::from_utf8(buf).map_err(|_| Error::Malformed(line!()))?;
        Ok(string)
        //.map_err(|e| Error::Malformed(line!()))
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
}
