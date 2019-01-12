mod framebuffer;
mod mailbox;
mod memory;

use self::framebuffer::Framebuffer;
use std::error::Error;

pub struct Vc {
    pub fb: Framebuffer,
}

impl Vc {
    pub fn new() -> Result<Vc, Box<dyn Error>> {
        
        let fb = Framebuffer::new()?;

        Ok(Vc { fb })
    }
}
