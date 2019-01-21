use crate::vc::mailbox::{constants::*, mailbox_call};
use lazy_static::lazy_static;
use libc;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::Mutex;

const PAGE_SIZE: u32 = 4 * 1024;

#[derive(Debug)]
struct GpuMemoryHandle {
    handle: u32,
    _marker: PhantomData<*mut libc::c_void>,
}

impl GpuMemoryHandle {
    fn new(size: u32) -> Result<GpuMemoryHandle, Box<dyn Error>> {
        let mut message: [u32; 9] = [
            9 * 4,
            MBOX_REQUEST,
            MBOX_TAG_ALLOCATE_MEMORY,
            12,
            4,
            size,
            16,
            MEM_FLAG_HINT_PERMALOCK | MEM_FLAG_ZERO,
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);

        let handle = message[5];

        if handle == size {
            return Err(String::from("Unable to allocate memory").into());
        }

        Ok(GpuMemoryHandle {
            handle: handle,
            _marker: PhantomData,
        })
    }
}

impl Drop for GpuMemoryHandle {
    fn drop(&mut self) {
        let mut message: [u32; 7] = [
            7 * 4,
            MBOX_REQUEST,
            MBOX_TAG_RELEASE_MEMORY,
            4,
            4,
            self.handle,
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);

        assert!(message[5] == 0);
    }
}

#[derive(Debug)]
struct GpuMemory {
    handle: GpuMemoryHandle,
    bus_address: u32,
    _marker: PhantomData<*mut libc::c_void>,
}

impl GpuMemory {
    fn new(size: u32) -> Result<GpuMemory, Box<dyn Error>> {
        let handle = GpuMemoryHandle::new(size)?;

        let mut message: [u32; 7] = [
            7 * 4,
            MBOX_REQUEST,
            MBOX_TAG_LOCK_MEMORY,
            4,
            4,
            handle.handle,
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);

        let bus_address = message[5] & 0x3FFFFFFF;

        if bus_address == handle.handle {
            return Err(String::from("Unable to lock memory").into());
        }

        Ok(GpuMemory {
            handle: handle,
            bus_address: bus_address,
            _marker: PhantomData,
        })
    }
}

impl Drop for GpuMemory {
    fn drop(&mut self) {
        let mut message: [u32; 7] = [
            7 * 4,
            MBOX_REQUEST,
            MBOX_TAG_UNLOCK_MEMORY,
            4,
            4,
            self.handle.handle,
            MBOX_TAG_LAST,
        ];

        mailbox_call(&mut message);

        assert!(message[5] == 0);
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GpuAllocation {
    gpu_memory: GpuMemory,
    allocation: Allocation,
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

    fn allocate_gpu_memory(&self, size: u32) -> Result<GpuAllocation, Box<dyn Error>> {
        let gpu_memory = GpuMemory::new(size)?;
        let allocation = self.map_physical_memory(gpu_memory.bus_address, size)?;

        Ok(GpuAllocation {
            gpu_memory: gpu_memory,
            allocation: allocation,
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

pub fn allocate_gpu_memory(size: u32) -> Result<GpuAllocation, Box<dyn Error>> {
    MEMORY.lock().unwrap().allocate_gpu_memory(size)
}
