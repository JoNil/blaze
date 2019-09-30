mod input;
mod vc;

use crate::input::Input;
use crate::vc::framebuffer::Framebuffer;
use crate::vc::mailbox::{constants::*, mailbox_call};
use crate::vc::memory::{allocate_gpu_memory, GpuAllocation};
use crate::vc::v3d::{command_builder::*, V3d};
use rand::random;
use static_assertions::const_assert;
use std::error::Error;
use std::fs;
use std::mem;
use std::thread;
use std::time;

#[cfg(unix)]
mod signal_panic {
    use libc;
    use std::{mem, ptr};

    extern "C" fn signal_handler(_: libc::c_int) {
        panic!("Exit");
    }

    pub fn setup() {
        unsafe {
            let action = libc::sigaction {
                sa_sigaction: signal_handler as extern "C" fn(libc::c_int) as libc::sighandler_t,
                sa_mask: mem::zeroed(),
                sa_flags: 0,
                sa_restorer: None,
            };
            libc::sigaction(libc::SIGINT, &action as *const _, ptr::null_mut());
        }
    }
}

#[cfg(windows)]
mod signal_panic {
    pub fn setup() {}
}

fn stop_cursor_blink() -> Result<(), Box<dyn Error>> {
    fs::write("/sys/class/graphics/fbcon/cursor_blink", "0")?;
    fs::write("/proc/sys/kernel/printk", "1")?;
    Ok(())
}

#[derive(Copy, Clone, Debug, Default)]
struct Vertex {
    x: u16, // X In 12.4 Fixed Point
    y: u16, // Y In 12.4 Fixed Point
    z: f32,
    one_over_w: f32,
    varying_r: f32,
    varying_g: f32,
    varying_b: f32,
}

const_assert!(Vertex; mem::size_of::<Vertex>() == 24);

struct RenderState {
    vertex_buffer: GpuAllocation<Vertex>,
    fragment_shader_buffer: GpuAllocation<u32>,
    shader_program: GpuAllocation<NvShaderState>,
    bin_memory: GpuAllocation<u8>,
    bin_base: GpuAllocation<u8>,
    binning_command_buffer: GpuAllocation<u8>,
    binning_command_buffer_end: u32,
    render_command_buffer: GpuAllocation<u8>,
    render_command_buffer_end: u32,
}

impl RenderState {
    fn new(fb: &Framebuffer) -> Result<RenderState, Box<dyn Error>> {
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
            dbg!(vertex_buffer.get_bus_address_l2_disabled());
            vertex_buffer
        };

        let fragment_shader_buffer = {
            let fragment_shader = &[
                0x958E0DBF, 0xD1724823, // mov r0, vary; mov r3.8d, 1.0
                0x818E7176, 0x40024821, // fadd r0, r0, r5; mov r1, vary
                0x818E7376, 0x10024862, // fadd r1, r1, r5; mov r2, vary
                0x819E7540, 0x114248A3, // fadd r2, r2, r5; mov r3.8a, r0
                0x809E7009, 0x115049E3, // nop; mov r3.8b, r1
                0x809E7012, 0x116049E3, // nop; mov r3.8c, r2
                0x159E76C0, 0x30020BA7, // mov tlbc, r3; nop; thrend
                0x009E7000, 0x100009E7, // nop; nop; nop
                0x009E7000, 0x500009E7, // nop; nop; sbdone
            ];

            let mut fragment_shader_buffer =
                allocate_gpu_memory::<u32>(fragment_shader.len() as u32)?;
            fragment_shader_buffer
                .as_mut_slice()
                .copy_from_slice(fragment_shader);
            dbg!(fragment_shader_buffer.get_bus_address_l2_disabled());
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

            dbg!(shader_program.get_bus_address_l2_disabled());

            shader_program
        };

        let bin_memory = allocate_gpu_memory::<u8>(4 * 1024 * 1024)?;
        let bin_base = allocate_gpu_memory::<u8>(4 * 1024 * 1024)?;

        let binning_command_buffer = allocate_gpu_memory::<u8>(1024 * 1024)?;
        let render_command_buffer = allocate_gpu_memory::<u8>(16 * 1024)?;

        dbg!(bin_memory.get_bus_address_l2_disabled());
        dbg!(bin_base.get_bus_address_l2_disabled());
        dbg!(binning_command_buffer.get_bus_address_l2_disabled());
        dbg!(render_command_buffer.get_bus_address_l2_disabled());

        Ok(RenderState {
            vertex_buffer,
            fragment_shader_buffer,
            shader_program,
            bin_memory,
            bin_base,
            binning_command_buffer,
            binning_command_buffer_end: 0,
            render_command_buffer,
            render_command_buffer_end: 0,
        })
    }

    fn update_command_buffer(&mut self, fb: &Framebuffer) {
        {
            let mut cb = CommandBuilder::new(self.render_command_buffer.as_mut_slice());

            //cb.wait_on_semaphore();

            let color = (random::<u32>() | 0xff000000) as u64;

            cb.clear_colors((color << 32) | color, 0, 0, 0);

            cb.tile_rendering_mode_configuration(
                fb.allocation().get_bus_address_l2_disabled(),
                fb.width() as u16,
                fb.height() as u16,
                TILE_RENDER_FLAGS_FRAME_BUFFER_COLOR_FORMAT_RGBA8888,
            );

            cb.tile_coordinates(0, 0);
            cb.store_tile_buffer_general(
                STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_COLOR,
                0,
                fb.allocation().get_bus_address_l2_disabled(),
            );

            let column_count = (fb.width() + 63) / 64;
            let row_count = (fb.height() + 63) / 64;

            for x in 0..column_count {
                for y in 0..row_count {
                    if x == column_count - 1 && y == row_count - 1 {
                        cb.tile_coordinates(x as i8, y as i8);
                        cb.branch_to_sub_list(
                            self.bin_memory.get_bus_address_l2_disabled()
                                + ((y * column_count + x) * 32),
                        );
                        cb.store_multi_sample_end();
                    } else {
                        cb.tile_coordinates(x as i8, y as i8);
                        cb.branch_to_sub_list(
                            self.bin_memory.get_bus_address_l2_disabled()
                                + ((y * column_count + x) * 32),
                        );
                        cb.store_multi_sample();
                    }
                }
            }

            self.render_command_buffer_end = cb.end();
        }

        {
            let mut cb = CommandBuilder::new(self.binning_command_buffer.as_mut_slice());

            cb.tile_binning_mode_configuration(
                self.bin_memory.get_bus_address_l2_disabled(),
                self.bin_memory.len(),
                self.bin_base.get_bus_address_l2_disabled(),
                ((fb.width() + 63) / 64 + 1) as u8,
                ((fb.height() + 63) / 64 + 1) as u8,
                TILE_BINNING_FLAGS_AUTO_INITIALISE_TILE_STATE_DATA_ARRAY,
            );

            cb.start_tile_binning();
            //cb.increment_semaphore();

            cb.clip_window(0, 0, fb.width() as u16, fb.height() as u16);

            cb.configuration_bits(
                CONFIGURATION_BITS_FLAGS8_ENABLE_FORWARD_FACING_PRIMITIVE
                    | CONFIGURATION_BITS_FLAGS8_ENABLE_REVERSE_FACING_PRIMITIVE,
                0,
            );

            cb.viewport_offset(0, 0);

            cb.nv_shader_state(self.shader_program.get_bus_address_l2_disabled());
            cb.vertex_array_primitives(PRIMITIVE_MODE_TRIANGLES, 3, 0);

            cb.flush();

            self.binning_command_buffer_end = cb.end();
        }
    }

    fn draw(&self, v3d: &mut V3d) {
        dbg!(v3d.errstat());

        dbg!(v3d.ct0cs());
        dbg!(v3d.ct1cs());

        dbg!(v3d.pcs());

        dbg!(v3d.sqrsv0());
        dbg!(v3d.sqrsv1());

        dbg!(v3d.l2cactl());

        dbg!(v3d.set_l2cactl(2));
        dbg!(v3d.set_l2cactl(4));

        dbg!(v3d.l2cactl());

        dbg!(v3d.intctl());
        dbg!(v3d.set_intctl(0xf));
        dbg!(v3d.intctl());

        // Reset
        v3d.set_ct0cs(0x8000);
        v3d.set_ct1cs(0x8000);

        v3d.set_ct0cs(0x0020);
        v3d.set_ct1cs(0x0020);

        v3d.set_ct0cs(0x0010);
        v3d.set_ct1cs(0x0010);

        dbg!(v3d.ct0cs());
        dbg!(v3d.ct1cs());

        dbg!(v3d.bpcs());

        v3d.set_ct0ca(self.binning_command_buffer.get_bus_address_l2_disabled());
        v3d.set_ct0ea(
            self.binning_command_buffer.get_bus_address_l2_disabled()
                + self.binning_command_buffer_end,
        );

        dbg!(v3d.bpcs());

        let mut count = 0;

        while v3d.bfc() != 1 {
            dbg!(v3d.pcs());
            count += 1;
        }
        v3d.set_bfc(1);

        dbg!(count);
        dbg!(v3d.bpcs());

        println!("2");

        dbg!(v3d.pcs());

        v3d.set_ct1ca(self.render_command_buffer.get_bus_address_l2_disabled());
        v3d.set_ct1ea(
            self.render_command_buffer.get_bus_address_l2_disabled()
                + self.render_command_buffer_end,
        );

        println!("3");

        let mut count = 0;

        let mut last_address = 0;

        while v3d.rfc() != 1 {
            {
                let address = v3d.ct1ca();
                if address != last_address {
                    dbg!(v3d.pcs());
                    last_address = dbg!(address);
                }
            }
            count += 1;
        }
        v3d.set_rfc(1);

        dbg!(count);

        println!("4");
        dbg!(v3d.bpcs());

        thread::sleep(time::Duration::from_millis(1000));

        println!("5");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    signal_panic::setup();
    stop_cursor_blink()?;

    let mut fb = Framebuffer::new()?;
    let mut v3d = V3d::new()?;
    let mut render = RenderState::new(&fb)?;

    dbg!(v3d.ident0());

    let mut input = Input::new()?;

    let mut last = time::Instant::now();
    let mut measure_time = 0;
    let mut average_frame_time = 0.0;

    loop {
        input.poll();

        render.update_command_buffer(&fb);

        render.draw(&mut v3d);

        fb.swap();

        let frame_time = {
            let now = time::Instant::now();
            let frame_duration = now.duration_since(last);
            last = now;
            frame_duration.as_secs() as f32 + frame_duration.subsec_nanos() as f32 * 1e-9
        };

        measure_time += 1;
        average_frame_time = average_frame_time * 0.95 + frame_time * 0.05;

        if measure_time == 100 {
            measure_time = 0;
            println!("{}", average_frame_time);
        }

        thread::sleep(time::Duration::from_millis(10));
    }
}
