use libc;
use std::error::Error;

struct Mailbox {
    fd: libc::c_int,
}

impl Mailbox {
    fn new() -> Result<Mailbox, Box<dyn Error>> {
        let fd = unsafe { libc::open("/dev/vcio\0" as *const _ as *const _, 0) };
        if fd < 0 {
            return Err(String::from("Failed to open /dev/vcio").into());
        }

        Ok(Mailbox {
            fd: fd,
        })
    }
}

impl Drop for Mailbox {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }
}
