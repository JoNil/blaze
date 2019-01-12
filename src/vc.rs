mod framebuffer;
mod mailbox;
mod memory;

use self::framebuffer::Framebuffer;
use self::memory::Memory;
use std::error::Error;

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
