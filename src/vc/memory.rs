use lazy_static::lazy_static;
use libc;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::Mutex;

const PAGE_SIZE: u32 = 4 * 1024;

pub struct Allocation {
    pub address: *mut libc::c_void,
    base_address: *mut libc::c_void,
    size: u32,
}

impl Drop for Allocation {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.base_address, self.size as usize);
        }
    }
}

pub struct Memory {
    fd: libc::c_int,
    _marker: PhantomData<*mut libc::c_void>,
}

unsafe impl Send for Memory {}

impl Memory {
    pub fn new() -> Result<Memory, Box<dyn Error>> {
        let fd = unsafe {
            libc::open(
                b"/dev/mem\0" as *const _ as *const _,
                libc::O_RDWR | libc::O_SYNC,
            )
        };
        if fd < 0 {
            return Err(String::from("Unable to open memory").into());
        }

        Ok(Memory {
            fd: fd,
            _marker: PhantomData,
        })
    }

    fn map_physical_memory(&self, address: u32, size: u32) -> Result<Allocation, Box<dyn Error>> {
        let offset = address % PAGE_SIZE;
        let base = address - offset;

        let base_address = unsafe {
            libc::mmap(
                0 as *mut _,
                (size + offset) as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                self.fd,
                base as i32,
            )
        };

        if base_address.is_null() {
            return Err(String::from("Unable to map memory").into());
        }

        Ok(Allocation {
            address: unsafe { base_address.offset(offset as isize) },
            base_address: base_address,
            size: size,
        })
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

lazy_static! {
    static ref MEMORY: Mutex<Memory> = { Mutex::new(Memory::new().unwrap()) };
}

pub fn map_physical_memory(address: u32, size: u32) -> Result<Allocation, Box<dyn Error>> {
    MEMORY.lock().unwrap().map_physical_memory(address, size)
}
