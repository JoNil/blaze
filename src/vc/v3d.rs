mod command_builder;
mod defs;

use crate::vc::mailbox::{constants::*, mailbox_call};
use crate::vc::memory::allocate_gpu_memory;
use std::error::Error;
use std::mem;

#[derive(Copy, Clone)]
struct Vertex {
    x: u16, // X In 12.4 Fixed Point
    y: u16, // Y In 12.4 Fixed Point
    z: f32,
    one_over_w: f32,
    varying_r: f32,
    varying_g: f32,
    varying_b: f32,
}

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

    let mut vertex_data_memory = allocate_gpu_memory((3 * mem::size_of::<Vertex>()) as u32)?;
    {
        let vertex_data_slice = vertex_data_memory.map_slice_mut::<Vertex>();
        vertex_data_slice[0] = Vertex {
            x: 320 * 16,
            y: 32 * 16,
            z: 1.0,
            one_over_w: 1.0,
            varying_r: 1.0,
            varying_g: 0.0,
            varying_b: 0.0,
        };
        vertex_data_slice[1] = Vertex {
            x: 32 * 16,
            y: 448 * 16,
            z: 1.0,
            one_over_w: 1.0,
            varying_r: 0.0,
            varying_g: 1.0,
            varying_b: 0.0,
        };
        vertex_data_slice[2] = Vertex {
            x: 608 * 16,
            y: 448 * 16,
            z: 1.0,
            one_over_w: 1.0,
            varying_r: 0.0,
            varying_g: 0.0,
            varying_b: 1.0,
        };
    }

    let mem2 = allocate_gpu_memory(32)?;
    let mem3 = allocate_gpu_memory(32)?;

    dbg!(vertex_data_memory);
    dbg!(mem2);
    dbg!(mem3);

    Ok(())
}
