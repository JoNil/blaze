use super::mailbox::{constants::*, mailbox_call};
use super::memory::{Allocation, map_physical_memory};
use libc;
use std::error::Error;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pitch: u32,
    current_offset: u32,

    mapped_fb: Allocation,
}

impl Framebuffer {
    pub fn new() -> Result<Framebuffer, Box<dyn Error>>
    {
        let mut message: [u32; 35] = [
            35 * 4,
            MBOX_REQUEST,
            MBOX_TAG_SET_PHYSICAL_DISPLAY,
            8,
            8,
            1280, // FrameBufferInfo.width
            720,  // FrameBufferInfo.height
            MBOX_TAG_SET_VIRTUAL_BUFFER,
            8,
            8,
            1280, // FrameBufferInfo.virtual_width
            1440, // FrameBufferInfo.virtual_height
            MBOX_TAG_SET_VIRTUAL_OFFSET,
            8,
            8,
            0, // FrameBufferInfo.x_offset
            0, // FrameBufferInfo.y.offset
            MBOX_TAG_SET_DEPTH,
            4,
            4,
            32, // FrameBufferInfo.depth
            MBOX_TAG_SET_PIXEL_ORDER,
            4,
            4,
            0, //RGB, not BGR preferably
            MBOX_TAG_ALLOCATE_BUFFER,
            8,
            8,
            4096, // FrameBufferInfo.pointer
            0,    // FrameBufferInfo.size
            MBOX_TAG_GET_PITCH,
            4,
            4,
            0, // FrameBufferInfo.pitch
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);

        if message[28] == 0 {
            return Err(String::from("Unable to allocate framebuffer").into());
        }

        let width = message[5];
        let height = message[6];
        let pitch = message[33];
        let address = message[28] & 0x3FFFFFFF;

        let mapped_fb = map_physical_memory(address, pitch * 2 * height)?;

        Ok(Framebuffer {
            width: width,
            height: height,
            pitch: pitch,
            current_offset: height * pitch,

            mapped_fb: mapped_fb,
        })
    }

    pub fn clear(&mut self) {
        const COLOR: u32 = 0xff240A30;

        let ptr: *mut u8 =
            unsafe { (self.mapped_fb.address as *mut u8).offset(self.current_offset as isize) };

        for i in 0..self.height {
            let mut line: *mut u8 = unsafe { ptr.offset((self.pitch * i) as isize) };

            for _j in 0..self.width {
                unsafe { *(line as *mut u32) = COLOR };
                line = unsafe { line.offset(4) };
            }
        }
    }

    pub fn draw(&mut self, x: u32, y: u32) {
        const COLOR: u32 = 0xff00ff00;

        let ptr: *mut u8 = unsafe {
            (self.mapped_fb.address as *mut u8)
                .offset((self.current_offset + 4 * x + self.pitch * y) as isize)
        };

        for i in 0..100 {
            let mut line: *mut u8 = unsafe { ptr.offset((self.pitch * i) as isize) };

            for _j in 0..100 {
                unsafe { *(line as *mut u32) = COLOR };
                line = unsafe { line.offset(4) };
            }
        }
    }

    pub fn swap(&mut self) {
        let mut y_offset = 0;

        if self.current_offset == 0 {
            self.current_offset = self.height * self.pitch;
        } else {
            y_offset = self.height;
            self.current_offset = 0;
        }

        let mut message: [u32; 8] = [
            8 * 4,
            MBOX_REQUEST,
            MBOX_TAG_SET_VIRTUAL_OFFSET,
            8,
            8,
            0,        // x offset
            y_offset, // y offset
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);
    }
}
