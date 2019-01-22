pub mod command_builder;
mod defs;

use crate::vc::memory::{Allocation, map_physical_memory};
use self::defs::*;
use std::error::Error;
use std::sync::Mutex;

pub struct V3d {
    allocation: Allocation,
}

impl V3d {
    pub fn new() -> Result<V3d, Box<dyn Error>> {
        Ok(V3d {
            allocation: map_physical_memory(BASE, 0x1000)?,
        })
    }
}
