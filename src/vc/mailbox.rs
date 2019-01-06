use libc;
use std::error::Error;
use std::marker::PhantomData;

#[repr(align(16))]
pub struct U32Aligned16(pub u32);

#[allow(dead_code)]
pub mod constants {
    use super::U32Aligned16;

    pub const MBOX_REQUEST: U32Aligned16 = U32Aligned16(0);

    pub const MBOX_TAG_GET_FIRMWARE_REVISION: U32Aligned16 = U32Aligned16(0x00000001); // VideoCore: Get Firmware Revision (Response: Firmware Revision)
    pub const MBOX_TAG_GET_BOARD_MODEL: U32Aligned16 = U32Aligned16(0x00010001); // Hardware: Get Board Model (Response: Board Model)
    pub const MBOX_TAG_GET_BOARD_REVISION: U32Aligned16 = U32Aligned16(0x00010002); // Hardware: Get Board Revision (Response: Board Revision)
    pub const MBOX_TAG_GET_BOARD_MAC_ADDRESS: U32Aligned16 = U32Aligned16(0x00010003); // Hardware: Get Board MAC Address (Response: MAC Address In Network Byte Order)
    pub const MBOX_TAG_GET_BOARD_SERIAL: U32Aligned16 = U32Aligned16(0x00010004); // Hardware: Get Board Serial (Response: Board Serial)
    pub const MBOX_TAG_GET_ARM_MEMORY: U32Aligned16 = U32Aligned16(0x00010005); // Hardware: Get ARM Memory (Response: Base Address In Bytes, Size In Bytes)
    pub const MBOX_TAG_GET_VC_MEMORY: U32Aligned16 = U32Aligned16(0x00010006); // Hardware: Get VC Memory (Response: Base Address In Bytes, Size In Bytes)
    pub const MBOX_TAG_GET_CLOCKS: U32Aligned16 = U32Aligned16(0x00010007); // Hardware: Get Clocks (Response: Parent Clock ID (0 For A Root Clock), Clock ID)
    pub const MBOX_TAG_GET_POWER_STATE: U32Aligned16 = U32Aligned16(0x00020001); // Power: Get Power State (Response: Device ID, State)
    pub const MBOX_TAG_GET_TIMING: U32Aligned16 = U32Aligned16(0x00020002); // Power: Get Timing (Response: Device ID, Enable Wait Time In Microseconds)
    pub const MBOX_TAG_SET_POWER_STATE: U32Aligned16 = U32Aligned16(0x00028001); // Power: Set Power State (Response: Device ID, State)
    pub const MBOX_TAG_GET_CLOCK_STATE: U32Aligned16 = U32Aligned16(0x00030001); // Clocks: Get Clock State (Response: Clock ID, State)
    pub const MBOX_TAG_GET_CLOCK_RATE: U32Aligned16 = U32Aligned16(0x00030002); // Clocks: Get Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_VOLTAGE: U32Aligned16 = U32Aligned16(0x00030003); // Voltage: Get Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_MAX_CLOCK_RATE: U32Aligned16 = U32Aligned16(0x00030004); // Clocks: Get Max Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_MAX_VOLTAGE: U32Aligned16 = U32Aligned16(0x00030005); // Voltage: Get Max Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_TEMPERATURE: U32Aligned16 = U32Aligned16(0x00030006); // Voltage: Get Temperature (Response: Temperature ID, Value In Degrees C)
    pub const MBOX_TAG_GET_MIN_CLOCK_RATE: U32Aligned16 = U32Aligned16(0x00030007); // Clocks: Get Min Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_GET_MIN_VOLTAGE: U32Aligned16 = U32Aligned16(0x00030008); // Voltage: Get Min Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_GET_TURBO: U32Aligned16 = U32Aligned16(0x00030009); // Clocks: Get Turbo (Response: ID, Level)
    pub const MBOX_TAG_GET_MAX_TEMPERATURE: U32Aligned16 = U32Aligned16(0x0003000A); // Voltage: Get Max Temperature (Response: Temperature ID, Value In Degrees C)
    pub const MBOX_TAG_ALLOCATE_MEMORY: U32Aligned16 = U32Aligned16(0x0003000C); // Memory: Allocates Contiguous Memory On The GPU (Response: Handle)
    pub const MBOX_TAG_LOCK_MEMORY: U32Aligned16 = U32Aligned16(0x0003000D); // Memory: Lock Buffer In Place, And Return A Bus Address (Response: Bus Address)
    pub const MBOX_TAG_UNLOCK_MEMORY: U32Aligned16 = U32Aligned16(0x0003000E); // Memory: Unlock Buffer (Response: Status)
    pub const MBOX_TAG_RELEASE_MEMORY: U32Aligned16 = U32Aligned16(0x0003000F); // Memory: Free The Memory Buffer (Response: Status)
    pub const MBOX_TAG_EXECUTE_CODE: U32Aligned16 = U32Aligned16(0x00030010); // Memory: Calls The Function At Given (Bus) Address And With Arguments Given
    pub const MBOX_TAG_EXECUTE_QPU: U32Aligned16 = U32Aligned16(0x00030011); // QPU: Calls The QPU Function At Given (Bus) Address And With Arguments Given (Response: Number Of QPUs, Control, No Flush, Timeout In ms)
    pub const MBOX_TAG_ENABLE_QPU: U32Aligned16 = U32Aligned16(0x00030012); // QPU: Enables The QPU (Response: Enable State)
    pub const MBOX_TAG_GET_EDID_BLOCK: U32Aligned16 = U32Aligned16(0x00030020); // HDMI: Read Specificed EDID Block From Attached HDMI/DVI Device (Response: Block Number, Status, EDID Block (128 Bytes))
    pub const MBOX_TAG_SET_CLOCK_STATE: U32Aligned16 = U32Aligned16(0x00038001); // Clocks: Set Clock State (Response: Clock ID, State)
    pub const MBOX_TAG_SET_CLOCK_RATE: U32Aligned16 = U32Aligned16(0x00038002); // Clocks: Set Clock Rate (Response: Clock ID, Rate In Hz)
    pub const MBOX_TAG_SET_VOLTAGE: U32Aligned16 = U32Aligned16(0x00038003); // Voltage: Set Voltage (Response: Voltage ID, Value Offset From 1.2V In Units Of 0.025V)
    pub const MBOX_TAG_SET_TURBO: U32Aligned16 = U32Aligned16(0x00038009); // Clocks: Set Turbo (Response: ID, Level)
    pub const MBOX_TAG_ALLOCATE_BUFFER: U32Aligned16 = U32Aligned16(0x00040001); // Frame Buffer: Allocate Buffer (Response: Frame Buffer Base Address In Bytes, Frame Buffer Size In Bytes)
    pub const MBOX_TAG_BLANK_SCREEN: U32Aligned16 = U32Aligned16(0x00040002); // Frame Buffer: Blank Screen (Response: State)
    pub const MBOX_TAG_GET_PHYSICAL_DISPLAY: U32Aligned16 = U32Aligned16(0x00040003); // Frame Buffer: Get Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_GET_VIRTUAL_BUFFER: U32Aligned16 = U32Aligned16(0x00040004); // Frame Buffer: Get Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_GET_DEPTH: U32Aligned16 = U32Aligned16(0x00040005); // Frame Buffer: Get Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_GET_PIXEL_ORDER: U32Aligned16 = U32Aligned16(0x00040006); // Frame Buffer: Get Pixel Order (Response: State)
    pub const MBOX_TAG_GET_ALPHA_MODE: U32Aligned16 = U32Aligned16(0x00040007); // Frame Buffer: Get Alpha Mode (Response: State)
    pub const MBOX_TAG_GET_PITCH: U32Aligned16 = U32Aligned16(0x00040008); // Frame Buffer: Get Pitch (Response: Bytes Per Line)
    pub const MBOX_TAG_GET_VIRTUAL_OFFSET: U32Aligned16 = U32Aligned16(0x00040009); // Frame Buffer: Get Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_GET_OVERSCAN: U32Aligned16 = U32Aligned16(0x0004000A); // Frame Buffer: Get Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_GET_PALETTE: U32Aligned16 = U32Aligned16(0x0004000B); // Frame Buffer: Get Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_TEST_PHYSICAL_DISPLAY: U32Aligned16 = U32Aligned16(0x00044003); // Frame Buffer: Test Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_TEST_VIRTUAL_BUFFER: U32Aligned16 = U32Aligned16(0x00044004); // Frame Buffer: Test Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_TEST_DEPTH: U32Aligned16 = U32Aligned16(0x00044005); // Frame Buffer: Test Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_TEST_PIXEL_ORDER: U32Aligned16 = U32Aligned16(0x00044006); // Frame Buffer: Test Pixel Order (Response: State)
    pub const MBOX_TAG_TEST_ALPHA_MODE: U32Aligned16 = U32Aligned16(0x00044007); // Frame Buffer: Test Alpha Mode (Response: State)
    pub const MBOX_TAG_TEST_VIRTUAL_OFFSET: U32Aligned16 = U32Aligned16(0x00044009); // Frame Buffer: Test Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_TEST_OVERSCAN: U32Aligned16 = U32Aligned16(0x0004400A); // Frame Buffer: Test Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_TEST_PALETTE: U32Aligned16 = U32Aligned16(0x0004400B); // Frame Buffer: Test Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_RELEASE_BUFFER: U32Aligned16 = U32Aligned16(0x00048001); // Frame Buffer: Release Buffer (Response: Releases And Disables The Frame Buffer)
    pub const MBOX_TAG_SET_PHYSICAL_DISPLAY: U32Aligned16 = U32Aligned16(0x00048003); // Frame Buffer: Set Physical (Display) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_SET_VIRTUAL_BUFFER: U32Aligned16 = U32Aligned16(0x00048004); // Frame Buffer: Set Virtual (Buffer) Width/Height (Response: Width In Pixels, Height In Pixels)
    pub const MBOX_TAG_SET_DEPTH: U32Aligned16 = U32Aligned16(0x00048005); // Frame Buffer: Set Depth (Response: Bits Per Pixel)
    pub const MBOX_TAG_SET_PIXEL_ORDER: U32Aligned16 = U32Aligned16(0x00048006); // Frame Buffer: Set Pixel Order (Response: State)
    pub const MBOX_TAG_SET_ALPHA_MODE: U32Aligned16 = U32Aligned16(0x00048007); // Frame Buffer: Set Alpha Mode (Response: State)
    pub const MBOX_TAG_SET_VIRTUAL_OFFSET: U32Aligned16 = U32Aligned16(0x00048009); // Frame Buffer: Set Virtual Offset (Response: X In Pixels, Y In Pixels)
    pub const MBOX_TAG_SET_OVERSCAN: U32Aligned16 = U32Aligned16(0x0004800A); // Frame Buffer: Set Overscan (Response: Top In Pixels, Bottom In Pixels, Left In Pixels, Right In Pixels)
    pub const MBOX_TAG_SET_PALETTE: U32Aligned16 = U32Aligned16(0x0004800B); // Frame Buffer: Set Palette (Response: RGBA Palette Values (Index 0 To 255))
    pub const MBOX_TAG_GET_COMMAND_LINE: U32Aligned16 = U32Aligned16(0x00050001); // Config: Get Command Line (Response: ASCII Command Line String)
    pub const MBOX_TAG_GET_DMA_CHANNELS: U32Aligned16 = U32Aligned16(0x00060001); // Shared Resource Management: Get DMA Channels (Response: Bits 0-15: DMA channels 0-15)
    pub const MBOX_TAG_LAST: U32Aligned16 = U32Aligned16(0);

    pub const CLK_EMMC_ID: U32Aligned16 = U32Aligned16(0x1); // EMMC
    pub const CLK_UART_ID: U32Aligned16 = U32Aligned16(0x2); // UART
    pub const CLK_ARM_ID: U32Aligned16 = U32Aligned16(0x3); // ARM
    pub const CLK_CORE_ID: U32Aligned16 = U32Aligned16(0x4); // CORE
    pub const CLK_V3D_ID: U32Aligned16 = U32Aligned16(0x5); // V3D
    pub const CLK_H264_ID: U32Aligned16 = U32Aligned16(0x6); // H264
    pub const CLK_ISP_ID: U32Aligned16 = U32Aligned16(0x7); // ISP
    pub const CLK_SDRAM_ID: U32Aligned16 = U32Aligned16(0x8); // SDRAM
    pub const CLK_PIXEL_ID: U32Aligned16 = U32Aligned16(0x9); // PIXEL
    pub const CLK_PWM_ID: U32Aligned16 = U32Aligned16(0xA); // PWM
}

#[allow(dead_code)]
const IOCTL_MBOX_PROPERTY: libc::c_ulong = 3221513216;

pub struct Mailbox {
    fd: libc::c_int,
    _marker: PhantomData<*mut libc::c_void>,
}

impl Mailbox {
    pub fn new() -> Result<Mailbox, Box<dyn Error>> {
        let fd = unsafe { libc::open("/dev/vcio\0" as *const _ as *const _, 0) };
        if fd < 0 {
            return Err(String::from("Failed to open /dev/vcio").into());
        }

        Ok(Mailbox { fd: fd, _marker: PhantomData })
    }

    #[cfg(unix)]
    pub fn call(&self, buf: &mut [U32Aligned16]) {
        let ret = unsafe { libc::ioctl(self.fd, IOCTL_MBOX_PROPERTY, buf.as_mut_ptr()) };

        if ret < 0 {
            panic!("ioctl failed: {}", ret);
        }
    }

    #[cfg(windows)]
    pub fn call(&self, _buf: &mut [U32Aligned16]) {}
}

impl Drop for Mailbox {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
