mod framebuffer;
mod mailbox;
mod memory;

use self::framebuffer::Framebuffer;
use std::error::Error;
use self::memory::Memory;

pub struct Vc {
    pub mem: Memory,
    pub fb: Framebuffer,
}

impl Vc {
    pub fn new() -> Result<Vc, Box<dyn Error>> {
        let mem = Memory::new()?;
        let fb = Framebuffer::new()?;

        Ok(Vc { mem, fb })
    }
}
