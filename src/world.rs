use std::{
    ffi::OsString,
    sync::{Arc, Mutex},
};

use crate::NbtValue;

struct _World {
    path: OsString,
    data: Arc<Mutex<NbtValue>>,
}

struct World(Arc<Mutex<World>>);

impl World {
    pub fn get_region(&self, x: i32, y: i32, dim: Dimension) -> Region {
        todo!()
    }

    pub fn get_existing_region(&self, x: i32, y: i32, dim: Dimension) -> Option<Region> {
        todo!()
    }
    pub fn get_level_dat(&self) -> LevelDat {
        todo!()
    }
}

struct _Region {
    data: Arc<Mutex<NbtValue>>,
}

impl _Region {
    fn wrapped(self) -> Region {
        Region(Arc::new(Mutex::new(self)))
    }
}

struct Region(Arc<Mutex<_Region>>);

impl Region {
    /// panics if x or y > 31
    fn get_chunk(&self, x: u8, y: u8) -> Chunk {
        todo!()
    }
}

enum Dimension {
    Overworld,
    Nether,
    End,
}

struct _LevelDat {}

pub struct LevelDat(Arc<Mutex<_LevelDat>>);

struct _Chunk {}

pub struct Chunk(Arc<Mutex<_Chunk>>);
