#![recursion_limit = "256"]
mod de;
mod error;
mod nbt_value;
mod pylib;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use nbt_value::{NbtList, NbtValue};
pub use pylib::*;

struct World {
    path: String,
}
