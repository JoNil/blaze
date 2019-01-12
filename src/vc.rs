mod framebuffer;
mod mailbox;
mod memory;

use self::framebuffer::Framebuffer;
use self::mailbox::Mailbox;
use std::error::Error;
use self::memory::Memory;

pub struct Vc {
    pub mem: Memory,
    pub mb: Mailbox,
    pub fb: Framebuffer,
}

impl Vc {
    pub fn new() -> Result<Vc, Box<dyn Error>> {
        let mem = Memory::new()?;
        let mb = Mailbox::new()?;
        let fb = Framebuffer::new(&mb)?;

        Ok(Vc { mem, mb, fb })
    }
}
