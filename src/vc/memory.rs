use libc;
use std::error::Error;
use std::marker::PhantomData;

const PAGE_SIZE: u32 = 4 * 1024;

pub struct Allocation<'a> {
    pub address: *mut libc::c_void,
    pub size: u32,
    _marker: PhantomData<&'a Memory>,
}

impl<'a> Drop for Allocation<'a> {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.address, self.size as usize);
        }
    }
}

pub struct Memory {
    fd: libc::c_int,
    _marker: PhantomData<*mut libc::c_void>,
}

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

    pub fn map_physical_memory<'a, 'b>(
        &'a self,
        address: u32,
        size: u32,
    ) -> Result<Allocation<'b>, Box<dyn Error>>
    where
        'a: 'b,
    {
        let offset = address % PAGE_SIZE;
        let base = address - offset;

        let mapped_address = unsafe {
            libc::mmap(
                0 as *mut _,
                size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                self.fd,
                base as i32,
            )
        };

        if mapped_address.is_null() {
            return Err(String::from("Unable to map memory").into());
        }

        Ok(Allocation {
            address: mapped_address,
            size: size,
            _marker: PhantomData,
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
