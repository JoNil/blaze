mod command_builder;
mod defs;

use crate::vc::mailbox::{constants::*, mailbox_call};
use crate::vc::memory::allocate_gpu_memory;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    let mut message: [u32; 17] = [
        17 * 4,
        MBOX_REQUEST,
        MBOX_TAG_SET_CLOCK_RATE,
        8,
        8,
        CLK_CORE_ID,
        400 * 1000 * 1000,
        MBOX_TAG_SET_CLOCK_RATE,
        8,
        8,
        CLK_V3D_ID,
        300 * 1000 * 1000,
        MBOX_TAG_ENABLE_QPU,
        4,
        4,
        1,
        MBOX_TAG_LAST,
    ];

    mailbox_call(&mut message);

    let mem1 = allocate_gpu_memory(32)?;
    let mem2 = allocate_gpu_memory(32)?;
    let mem3 = allocate_gpu_memory(32)?;

    dbg!(mem1);
    dbg!(mem2);
    dbg!(mem3);

    Ok(())
}
