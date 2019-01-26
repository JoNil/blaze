use crate::vc::mailbox::{constants::*, mailbox_call};
use lazy_static::lazy_static;
use libc;
use std::error::Error;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::sync::Mutex;

const PAGE_SIZE: u32 = 4 * 1024;
const BUS_ADDRESSES_L2CACHE_ENABLED: u32 = 0x40000000;
const BUS_ADDRESSES_L2CACHE_DISABLED: u32 = 0xC0000000;

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
    bus_address: u32,
    size: u32,
}

impl Allocation {
    pub fn get_bus_address_l2_disabled(&self) -> u32 {
        self.bus_address | BUS_ADDRESSES_L2CACHE_DISABLED
    }

    pub fn get_bus_address_l2_enabled(&self) -> u32 {
        self.bus_address | BUS_ADDRESSES_L2CACHE_ENABLED
    }
}

impl Drop for Allocation {
    #[cfg(unix)]
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.base_address, self.size as usize);
        }
    }

    #[cfg(windows)]
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct GpuAllocation<T: Copy> {
    gpu_memory: GpuMemory,
    allocation: Allocation,
    _marker: PhantomData<T>,
}

impl<T: Copy> GpuAllocation<T> {
    pub fn as_slice(&self) -> &[T] {
        assert!(self.allocation.size % mem::size_of::<T>() as u32 == 0);
        unsafe {
            slice::from_raw_parts(
                self.allocation.address as *const _,
                self.allocation.size as usize / mem::size_of::<T>(),
            )
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        assert!(self.allocation.size % mem::size_of::<T>() as u32 == 0);
        unsafe {
            slice::from_raw_parts_mut(
                self.allocation.address as *mut _,
                self.allocation.size as usize / mem::size_of::<T>(),
            )
        }
    }

    pub fn len(&self) -> u32 {
        self.allocation.size / mem::size_of::<T>() as u32
    }

    pub fn get_bus_address_l2_disabled(&self) -> u32 {
        self.gpu_memory.bus_address | BUS_ADDRESSES_L2CACHE_DISABLED
    }

    pub fn get_bus_address_l2_enabled(&self) -> u32 {
        self.gpu_memory.bus_address | BUS_ADDRESSES_L2CACHE_ENABLED
    }
}

pub struct Memory {
    fd: libc::c_int,
    _marker: PhantomData<*mut libc::c_void>,
}

unsafe impl Send for Memory {}

impl Memory {

    #[cfg(unix)]
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

    #[cfg(windows)]
    pub fn new() -> Result<Memory, Box<dyn Error>> {
        Err(String::from("This is not a pi").into())
    }

    #[cfg(unix)]
    fn map_physical_memory(
        &self,
        bus_address: u32,
        size: u32,
    ) -> Result<Allocation, Box<dyn Error>> {
        let offset = bus_address % PAGE_SIZE;
        let base = bus_address - offset;

        let base_address = unsafe {
            libc::mmap(
                0 as *mut _,
                (size + offset) as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                self.fd,
                base as _,
            )
        };

        if base_address.is_null() {
            return Err(String::from("Unable to map memory").into());
        }

        Ok(Allocation {
            address: unsafe { base_address.offset(offset as isize) },
            base_address: base_address,
            bus_address: bus_address,
            size: size,
        })
    }

    #[cfg(windows)]
    fn map_physical_memory(
        &self,
        _bus_address: u32,
        _size: u32,
    ) -> Result<Allocation, Box<dyn Error>> {
        Err(String::from("This is not a pi").into())
    }

    fn allocate_gpu_memory<T: Copy>(&self, count: u32) -> Result<GpuAllocation<T>, Box<dyn Error>> {
        let size = count * mem::size_of::<T>() as u32;

        let gpu_memory = GpuMemory::new(size)?;
        let allocation = self.map_physical_memory(gpu_memory.bus_address, size)?;

        Ok(GpuAllocation {
            gpu_memory: gpu_memory,
            allocation: allocation,
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

lazy_static! {
    static ref MEMORY: Mutex<Memory> = { Mutex::new(Memory::new().unwrap()) };
}

pub fn map_physical_memory(address: u32, size: u32) -> Result<Allocation, Box<dyn Error>> {
    MEMORY.lock().unwrap().map_physical_memory(address, size)
}

pub fn allocate_gpu_memory<T: Copy>(count: u32) -> Result<GpuAllocation<T>, Box<dyn Error>> {
    MEMORY.lock().unwrap().allocate_gpu_memory(count)
}
