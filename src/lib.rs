#![recursion_limit = "256"]
mod de;
mod error;
mod nbt_value;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use nbt_value::NbtValue;
// pub use ser::{to_bytes, Serializer};

struct World {
    path: String,
}

// struct Chunck {
//     nbt: Nbt,
// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::NbtValue;

    #[test]
    fn deserialize() {
        use serde::Deserialize;
        #[derive(Deserialize, Debug)]
        struct TestStruct {
            data: String,
            more: Vec<i32>,
        }
        let (name, data) = NbtValue::from_bytes(include_bytes!("test_nbt")).unwrap();
        assert_eq!(
            data,
            NbtValue::Compound(HashMap::from(
                [(String::from("Nice"), NbtValue::Byte(-69)),]
            ))
        )
    }
}
