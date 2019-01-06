use super::mailbox::{constants::*, Mailbox};
use libc;
use std::error::Error;

pub struct Framebuffer {
    width: u32,
    height: u32,
    pitch: u32,
    current_offset: u32,

    mem_fd: libc::c_int,
    mapped_address: *mut u32,
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

        println!("{:?}", address as *const libc::c_void);

        let fd = unsafe {
            libc::open(
                b"/dev/mem\0" as *const _ as *const _,
                libc::O_RDWR | libc::O_SYNC,
            )
        };
        if fd < 0 {
            return Err(String::from("Unable to open memory").into());
        }

        let page_size = 4 * 1024;

        let offset = address % page_size;
        let base = address - offset;

        let mapped_address = unsafe {
            libc::mmap(
                0 as *mut _,
                (pitch * 2 * height + offset) as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                base as i32,
            )
        };

        println!("{:?}", mapped_address);

        if mapped_address.is_null() {
            return Err(String::from("Unable to map memory").into());
        }

        Ok(Framebuffer {
            width: width,
            height: height,
            pitch: pitch,
            current_offset: height * pitch,

            mem_fd: fd,
            mapped_address: mapped_address as *mut _,
        })
    }

    pub fn clear(&mut self) {
        const COLOR: u32 = 0xff240A30;

        let ptr: *mut u8 =
            unsafe { (self.mapped_address as *mut u8).offset(self.current_offset as isize) };

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
            (self.mapped_address as *mut u8)
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

    pub fn swap(&mut self, mb: &Mailbox) {
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

        mb.call(&mut message);
    }
}
