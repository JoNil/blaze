mod command_builder;
mod defs;

use crate::vc::framebuffer::Framebuffer;
use crate::vc::mailbox::{constants::*, mailbox_call};
use crate::vc::memory::allocate_gpu_memory;
use crate::vc::v3d::command_builder::*;
use std::error::Error;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    x: u16, // X In 12.4 Fixed Point
    y: u16, // Y In 12.4 Fixed Point
    z: f32,
    one_over_w: f32,
    varying_r: f32,
    varying_g: f32,
    varying_b: f32,
}

pub fn init(fb: &Framebuffer) -> Result<(), Box<dyn Error>> {
    
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

    let vertex_buffer = {
        let vertices = &[
            Vertex {
                x: 320 * 16,
                y: 32 * 16,
                z: 1.0,
                one_over_w: 1.0,
                varying_r: 1.0,
                varying_g: 0.0,
                varying_b: 0.0,
            },
            Vertex {
                x: 32 * 16,
                y: 448 * 16,
                z: 1.0,
                one_over_w: 1.0,
                varying_r: 0.0,
                varying_g: 1.0,
                varying_b: 0.0,
            },
            Vertex {
                x: 608 * 16,
                y: 448 * 16,
                z: 1.0,
                one_over_w: 1.0,
                varying_r: 0.0,
                varying_g: 0.0,
                varying_b: 1.0,
            },
        ];

        let mut vertex_buffer = allocate_gpu_memory::<Vertex>(vertices.len() as u32)?;
        vertex_buffer.as_mut_slice().copy_from_slice(vertices);
        vertex_buffer
    };

    let fragment_shader_buffer = {
        let fragment_shader = &[
            0x958E0DBF,
            0xD1724823, // mov r0, vary; mov r3.8d, 1.0
            0x818E7176,
            0x40024821, // fadd r0, r0, r5; mov r1, vary
            0x818E7376,
            0x10024862, // fadd r1, r1, r5; mov r2, vary
            0x819E7540,
            0x114248A3, // fadd r2, r2, r5; mov r3.8a, r0
            0x809E7009,
            0x115049E3, // nop; mov r3.8b, r1
            0x809E7012,
            0x116049E3, // nop; mov r3.8c, r2
            0x159E76C0,
            0x30020BA7, // mov tlbc, r3; nop; thrend
            0x009E7000,
            0x100009E7, // nop; nop; nop
            0x009E7000,
            0x500009E7, // nop; nop; sbdone
        ];

        let mut fragment_shader_buffer = allocate_gpu_memory::<u32>(fragment_shader.len() as u32)?;
        fragment_shader_buffer.as_mut_slice().copy_from_slice(fragment_shader);
        fragment_shader_buffer
    };

    let shader_program = {
        let mut shader_program = allocate_gpu_memory::<NvShaderState>(1)?;

        shader_program.as_mut_slice()[0] = NvShaderState {
            flags: 0,
            stride: 6 * 4,
            fragment_shader_uniforms: 0,
            fragment_shader_varyings: 3,
            fragment_shader_code_address: fragment_shader_buffer.get_bus_address_l2_disabled(),
            fragment_shader_uniforms_address: 0,
            vertex_data_address: vertex_buffer.get_bus_address_l2_disabled(),
        };

        shader_program
    };

    let bin_memory = allocate_gpu_memory::<u8>(2 * 1024 * 1024)?;
    let bin_base = allocate_gpu_memory::<u8>(48*(4096/32)*(4096/32))?;

    let (binning_command_buffer, binning_command_buffer_end) = {

        let mut binning_command_buffer = allocate_gpu_memory::<u8>(4096)?;

        let mut cb = CommandBuilder::new(binning_command_buffer.as_mut_slice());
        cb.tile_binning_mode_configuration(
            bin_memory.get_bus_address_l2_disabled(),
            bin_memory.len(),
            bin_base.get_bus_address_l2_disabled(),
            ((fb.width() + 63) / 64) as u8,
            ((fb.height() + 63) / 64) as u8,
            TILE_BINNING_FLAGS_AUTO_INITIALISE_TILE_STATE_DATA_ARRAY
        );

        cb.start_tile_binning();
        cb.increment_semaphore();

        cb.clip_window(0, 0, fb.width() as u16, fb.height() as u16);

        cb.configuration_bits(
                CONFIGURATION_BITS_FLAGS8_ENABLE_FORWARD_FACING_PRIMITIVE |
                CONFIGURATION_BITS_FLAGS8_ENABLE_REVERSE_FACING_PRIMITIVE,
                CONFIGURATION_BITS_FLAGS16_EARLY_Z_UPDATES_ENABLE);

        cb.viewport_offset(0, 0);

        cb.nv_shader_state(shader_program.get_bus_address_l2_disabled());
        cb.vertex_array_primitives(PRIMITIVE_MODE_TRIANGLES, 3, 0);
        
        cb.flush();

        let end = cb.end();

        (binning_command_buffer, end)
    };

    Ok(())
}
