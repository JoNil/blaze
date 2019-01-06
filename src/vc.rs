mod framebuffer;
mod mailbox;

use self::framebuffer::Framebuffer;
use self::mailbox::Mailbox;
use std::error::Error;

pub struct Vc {
    pub mb: Mailbox,
    pub fb: Framebuffer,
}

impl Vc {
    pub fn new() -> Result<Vc, Box<dyn Error>> {
        let mb = Mailbox::new()?;
        let fb = Framebuffer::new(&mb)?;

        Ok(Vc { mb: mb, fb: fb })
    }
}
