use std::collections::HashMap;

use crate::{Map, NbtValue};
use pretty_assertions::assert_eq;

#[test]
fn test_nbt1() {
    assert_eq!(
        NbtValue::from_reader(include_bytes!("./../testdata/test_nbt").as_slice()),
        Ok((
            String::from("hello world"),
            NbtValue::Compound(Map::from([(
                "name".to_string(),
                NbtValue::String(String::from("Bananrama"))
            )]))
        ))
    )
}
#[test]
fn test_compressed() {
    assert_eq!(
        NbtValue::from_compressed_reader(include_bytes!("./../testdata/level.dat").as_slice())
            .unwrap(),
        {
            let crate::NbtFile { name, nbt } = rmp_serde::decode::from_slice::<crate::NbtFile>(
                include_bytes!("./../testdata/level.dat.msgpack"),
            )
            .unwrap();
            (name, nbt)
        }
    )
}
