use super::mailbox::{constants::*, Mailbox};
use libc;
use std::error::Error;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pitch: u32,
    base_address: *mut libc::c_void,
    current_address: *mut libc::c_void,
}

impl Framebuffer {
    pub fn new(mb: &Mailbox) -> Result<Framebuffer, Box<dyn Error>> {
        
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

        mb.call(&mut message);

        if message[28] == 0 {
            return Err(String::from("Unable to allocate framebuffer").into());
        }

        let width = message[5];
        let height = message[6];
        let pitch = message[33];
        let address = message[28] & 0x3FFFFFFF;

        Ok(Framebuffer {
            width: width,
            height: height,
            pitch: pitch,
            base_address: address as *mut _,
            current_address: (address + height*pitch) as *mut _,
        })
    }

    pub fn swap(&mut self, mb: &Mailbox) {
        let mut y_offset = 0;

        if self.current_address == self.base_address {
            self.current_address = (self.base_address as u32 + self.height*self.pitch) as *mut _;
        } else {
            y_offset = self.height;
            self.current_address = self.base_address;
        }

        let mut message: [u32; 8] = [
            8*4,
            MBOX_REQUEST,
            MBOX_TAG_SET_VIRTUAL_OFFSET,
            8,
            8,
            0,        // x offset
            y_offset, // y offset
            MBOX_TAG_LAST,
        ];

        mb.call(&mut message);
    }

    pub fn close(&self, mb: &Mailbox) {

        let mut message: [u32; 7] = [
            7*4,
            MBOX_REQUEST,
            MBOX_TAG_RELEASE_BUFFER,
            4,
            4,
            0,
            MBOX_TAG_LAST,
        ];

        mb.call(&mut message);
    }
}
