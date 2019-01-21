use lazy_static::lazy_static;
use libc;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::Mutex;

#[allow(dead_code)]
pub mod constants {
    pub const MBOX_REQUEST: u32 = 0;

    pub const MBOX_TAG_GET_FIRMWARE_REVISION: u32 = 0x00000001; // VideoCore: Get Firmware Revision (Response: Firmware Revision)
    pub const MBOX_TAG_GET_BOARD_MODEL: u32 = 0x00010001; // Hardware: Get Board Model (Response: Board Model)
    pub const MBOX_TAG_GET_BOARD_REVISION: u32 = 0x00010002; // Hardware: Get Board Revision (Response: Board Revision)
    pub const MBOX_TAG_GET_BOARD_MAC_ADDRESS: u32 = 0x00010003; // Hardware: Get Board MAC Address (Response: MAC Address In Network Byte Order)
    pub const MBOX_TAG_GET_BOARD_SERIAL: u32 = 0x00010004; // Hardware: Get Board Serial (Response: Board Serial)
    pub const MBOX_TAG_GET_ARM_MEMORY: u32 = 0x00010005; // Hardware: Get ARM Memory (Response: Base Address In Bytes, Size In Bytes)
    pub const MBOX_TAG_GET_VC_MEMORY: u32 = 0x00010006; // Hardware: Get VC Memory (Response: Base Address In Bytes, Size In Bytes)
    pub const MBOX_TAG_GET_CLOCKS: u32 = 0x00010007; // Hardware: Get Clocks (Response: Parent Clock ID (0 For A Root Clock), Clock ID)
    pub const MBOX_TAG_GET_POWER_STATE: u32 = 0x00020001; // Power: Get Power State (Response: Device ID, State)
    pub const MBOX_TAG_GET_TIMING: u32 = 0x00020002; // Power: Get Timing (Response: Device ID, Enable Wait Time In Microseconds)
    pub const MBOX_TAG_SET_POWER_STATE: u32 = 0x00028001; // Power: Set Power State (Response: Device ID, State)
    pub const MBOX_TAG_GET_CLOCK_STATE: u32 = 0x00030001; // Clocks: Get Clock State (Response: Clock ID, State)
    pub const MBOX_TAG_GET_CLOCK_RATE: u32 = 0x00030002; // Clocks: Get Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_VOLTAGE: u32 = 0x00030003; // Voltage: Get Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_MAX_CLOCK_RATE: u32 = 0x00030004; // Clocks: Get Max Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_MAX_VOLTAGE: u32 = 0x00030005; // Voltage: Get Max Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_TEMPERATURE: u32 = 0x00030006; // Voltage: Get Temperature (Response: Temperature ID, Value In Degrees C)
    pub const MBOX_TAG_GET_MIN_CLOCK_RATE: u32 = 0x00030007; // Clocks: Get Min Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_MIN_VOLTAGE: u32 = 0x00030008; // Voltage: Get Min Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_TURBO: u32 = 0x00030009; // Clocks: Get Turbo (Response: ID, Level)
    pub const MBOX_TAG_GET_MAX_TEMPERATURE: u32 = 0x0003000A; // Voltage: Get Max Temperature (Response: Temperature ID, Value In Degrees C)
    pub const MBOX_TAG_ALLOCATE_MEMORY: u32 = 0x0003000C; // Memory: Allocates Contiguous Memory On The GPU (Response: Handle)
    pub const MBOX_TAG_LOCK_MEMORY: u32 = 0x0003000D; // Memory: Lock Buffer In Place, And Return A Bus Address (Response: Bus Address)
    pub const MBOX_TAG_UNLOCK_MEMORY: u32 = 0x0003000E; // Memory: Unlock Buffer (Response: Status)
    pub const MBOX_TAG_RELEASE_MEMORY: u32 = 0x0003000F; // Memory: Free The Memory Buffer (Response: Status)
    pub const MBOX_TAG_EXECUTE_CODE: u32 = 0x00030010; // Memory: Calls The Function At Given (Bus) Address And With Arguments Given
    pub const MBOX_TAG_EXECUTE_QPU: u32 = 0x00030011; // QPU: Calls The QPU Function At Given (Bus) Address And With Arguments Given (Response: Number Of QPUs, Control, No Flush, Timeout In ms)
    pub const MBOX_TAG_ENABLE_QPU: u32 = 0x00030012; // QPU: Enables The QPU (Response: Enable State)
    pub const MBOX_TAG_GET_EDID_BLOCK: u32 = 0x00030020; // HDMI: Read Specificed EDID Block From Attached HDMI/DVI Device (Response: Block Number, Status, EDID Block (128 Bytes))
    pub const MBOX_TAG_SET_CLOCK_STATE: u32 = 0x00038001; // Clocks: Set Clock State (Response: Clock ID, State)
    pub const MBOX_TAG_SET_CLOCK_RATE: u32 = 0x00038002; // Clocks: Set Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_SET_VOLTAGE: u32 = 0x00038003; // Voltage: Set Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_SET_TURBO: u32 = 0x00038009; // Clocks: Set Turbo (Response: ID, Level)
    pub const MBOX_TAG_ALLOCATE_BUFFER: u32 = 0x00040001; // Frame Buffer: Allocate Buffer (Response: Frame Buffer Base Address In Bytes, Frame Buffer Size In Bytes)
    pub const MBOX_TAG_BLANK_SCREEN: u32 = 0x00040002; // Frame Buffer: Blank Screen (Response: State)
    pub const MBOX_TAG_GET_PHYSICAL_DISPLAY: u32 = 0x00040003; // Frame Buffer: Get Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_GET_VIRTUAL_BUFFER: u32 = 0x00040004; // Frame Buffer: Get Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_GET_DEPTH: u32 = 0x00040005; // Frame Buffer: Get Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_GET_PIXEL_ORDER: u32 = 0x00040006; // Frame Buffer: Get Pixel Order (Response: State)
    pub const MBOX_TAG_GET_ALPHA_MODE: u32 = 0x00040007; // Frame Buffer: Get Alpha Mode (Response: State)
    pub const MBOX_TAG_GET_PITCH: u32 = 0x00040008; // Frame Buffer: Get Pitch (Response: Bytes Per Line)
    pub const MBOX_TAG_GET_VIRTUAL_OFFSET: u32 = 0x00040009; // Frame Buffer: Get Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_GET_OVERSCAN: u32 = 0x0004000A; // Frame Buffer: Get Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_GET_PALETTE: u32 = 0x0004000B; // Frame Buffer: Get Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_TEST_PHYSICAL_DISPLAY: u32 = 0x00044003; // Frame Buffer: Test Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_TEST_VIRTUAL_BUFFER: u32 = 0x00044004; // Frame Buffer: Test Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_TEST_DEPTH: u32 = 0x00044005; // Frame Buffer: Test Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_TEST_PIXEL_ORDER: u32 = 0x00044006; // Frame Buffer: Test Pixel Order (Response: State)
    pub const MBOX_TAG_TEST_ALPHA_MODE: u32 = 0x00044007; // Frame Buffer: Test Alpha Mode (Response: State)
    pub const MBOX_TAG_TEST_VIRTUAL_OFFSET: u32 = 0x00044009; // Frame Buffer: Test Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_TEST_OVERSCAN: u32 = 0x0004400A; // Frame Buffer: Test Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_TEST_PALETTE: u32 = 0x0004400B; // Frame Buffer: Test Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_RELEASE_BUFFER: u32 = 0x00048001; // Frame Buffer: Release Buffer (Response: Releases And Disables The Frame Buffer)
    pub const MBOX_TAG_SET_PHYSICAL_DISPLAY: u32 = 0x00048003; // Frame Buffer: Set Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_SET_VIRTUAL_BUFFER: u32 = 0x00048004; // Frame Buffer: Set Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_SET_DEPTH: u32 = 0x00048005; // Frame Buffer: Set Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_SET_PIXEL_ORDER: u32 = 0x00048006; // Frame Buffer: Set Pixel Order (Response: State)
    pub const MBOX_TAG_SET_ALPHA_MODE: u32 = 0x00048007; // Frame Buffer: Set Alpha Mode (Response: State)
    pub const MBOX_TAG_SET_VIRTUAL_OFFSET: u32 = 0x00048009; // Frame Buffer: Set Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_SET_OVERSCAN: u32 = 0x0004800A; // Frame Buffer: Set Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_SET_PALETTE: u32 = 0x0004800B; // Frame Buffer: Set Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_GET_COMMAND_LINE: u32 = 0x00050001; // Config: Get Command Line (Response: ASCII Command Line String)
    pub const MBOX_TAG_GET_DMA_CHANNELS: u32 = 0x00060001; // Shared Resource Management: Get DMA Channels (Response: Bits 0-15: DMA channels 0-15)
    pub const MBOX_TAG_LAST: u32 = 0;

    pub const CLK_EMMC_ID: u32 = 0x1; // EMMC
    pub const CLK_UART_ID: u32 = 0x2; // UART
    pub const CLK_ARM_ID: u32 = 0x3; // ARM
    pub const CLK_CORE_ID: u32 = 0x4; // CORE
    pub const CLK_V3D_ID: u32 = 0x5; // V3D
    pub const CLK_H264_ID: u32 = 0x6; // H264
    pub const CLK_ISP_ID: u32 = 0x7; // ISP
    pub const CLK_SDRAM_ID: u32 = 0x8; // SDRAM
    pub const CLK_PIXEL_ID: u32 = 0x9; // PIXEL
    pub const CLK_PWM_ID: u32 = 0xA; // PWM

    pub const MEM_FLAG_DISCARDABLE: u32 = 1 << 0; // can be resized to 0 at any time. Use for cached data
    pub const MEM_FLAG_NORMAL: u32 = 0 << 2; // normal allocating alias. Don't use from ARM
    pub const MEM_FLAG_DIRECT: u32 = 1 << 2; // 0xC alias uncached
    pub const MEM_FLAG_COHERENT: u32 = 2 << 2; // 0x8 alias. Non-allocating in L2 but coherent
    pub const MEM_FLAG_L1_NONALLOCATING: u32 = MEM_FLAG_DIRECT | MEM_FLAG_COHERENT; // Allocating in L2
    pub const MEM_FLAG_ZERO: u32 = 1 << 4; // initialise buffer to all zeros
    pub const MEM_FLAG_NO_INIT: u32 = 1 << 5; // don't initialise (default is initialise to all ones
    pub const MEM_FLAG_HINT_PERMALOCK: u32 = 1 << 6; // Likely to be locked for long periods of time.
}

#[allow(dead_code)]
const IOCTL_MBOX_PROPERTY: libc::c_ulong = 3221513216;

struct Mailbox {
    fd: libc::c_int,
    _marker: PhantomData<*mut libc::c_void>,
}

unsafe impl Send for Mailbox {}

impl Mailbox {
    fn new() -> Result<Mailbox, Box<dyn Error>> {
        let fd = unsafe { libc::open(b"/dev/vcio\0" as *const _ as *const _, 0) };
        if fd < 0 {
            return Err(String::from("Failed to open /dev/vcio").into());
        }

        Ok(Mailbox {
            fd: fd,
            _marker: PhantomData,
        })
    }

    #[cfg(unix)]
    fn call(&self, buf: &mut [u32]) {
        let ret = unsafe { libc::ioctl(self.fd, IOCTL_MBOX_PROPERTY, buf.as_mut_ptr()) };

        if ret < 0 {
            panic!("ioctl failed: {}", ret);
        }
    }

    #[cfg(windows)]
    fn call(&self, _buf: &mut [u32]) {}
}

impl Drop for Mailbox {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

lazy_static! {
    static ref MAILBOX: Mutex<Mailbox> = { Mutex::new(Mailbox::new().unwrap()) };
}

pub fn mailbox_call(buf: &mut [u32]) {
    MAILBOX.lock().unwrap().call(buf)
}
