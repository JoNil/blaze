#![allow(dead_code)]

use byteorder::{LittleEndian, WriteBytesExt};
use static_assertions::const_assert;
use std::io::Cursor;
use std::mem;

const CMD_HALT: u8 = 0x00;
const CMD_NO_OP: u8 = 0x01;
const CMD_FLUSH: u8 = 0x04;
const CMD_FLUSH_ALL_STATE: u8 = 0x05;
const CMD_START_TILE_BINNING: u8 = 0x06;
const CMD_INCREMENT_SEMAPHORE: u8 = 0x07;
const CMD_WAIT_ON_SEMAPHORE: u8 = 0x08;
const CMD_BRANCH_TO_SUB_LIST: u8 = 0x11;
const CMD_STORE_MULTI_SAMPLE: u8 = 0x18;
const CMD_STORE_MULTI_SAMPLE_END: u8 = 0x19;
const CMD_VERTEX_ARRAY_PRIMITIVES: u8 = 0x21;
const CMD_NV_SHADER_STATE: u8 = 0x41;
const CMD_CONFIGURATION_BITS: u8 = 0x60;
const CMD_CLIP_WINDOW: u8 = 0x66;
const CMD_VIEWPORT_OFFSET: u8 = 0x67;
const CMD_TILE_BINNING_MODE_CONFIGURATION: u8 = 0x70;
const CMD_TILE_RENDERING_MODE_CONFIGURATION: u8 = 0x71;
const CMD_CLEAR_COLORS: u8 = 0x72;
const CMD_TILE_COORDINATES: u8 = 0x73;
const CMD_STORE_TILE_BUFFER_GENERAL: u8 = 0x1C;

pub const PRIMITIVE_MODE_POINTS: u8 = 0;
pub const PRIMITIVE_MODE_LINES: u8 = 1;
pub const PRIMITIVE_MODE_LINE_LOOP: u8 = 2;
pub const PRIMITIVE_MODE_LINE_STRIP: u8 = 3;
pub const PRIMITIVE_MODE_TRIANGLES: u8 = 4;
pub const PRIMITIVE_MODE_TRIANGLE_STRIP: u8 = 5;
pub const PRIMITIVE_MODE_TRIANGLE_FAN: u8 = 6;

pub const NV_SHADER_STATE_FLAG_FRAGMENT_SHADER_SINGLE_THREADED: u8 = 0x1;
pub const NV_SHADER_STATE_FLAG_STRIDE_POINT_SIZE_INCLUDED_IN_SHADED_VERTEX_DATA: u8 = 0x2;
pub const NV_SHADER_STATE_FLAG_ENABLE_CLIPPING: u8 = 0x4;
pub const NV_SHADER_STATE_FLAG_CLIP_COORDINATES_HEADER_INCLUDED_IN_SHADED_VERTEX_DATA: u8 = 0x8;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct NvShaderState {
    pub flags: u8,
    pub stride: u8,
    pub fragment_shader_uniforms: u8,
    pub fragment_shader_varyings: u8,
    pub fragment_shader_code_address: u32,
    pub fragment_shader_uniforms_address: u32,
    pub vertex_data_address: u32,
}

const_assert!(NvShaderStateSize; mem::size_of::<NvShaderState>() == 16);
const_assert!(NvShaderStateAlign; mem::align_of::<NvShaderState>() == 16);

pub const CONFIGURATION_BITS_FLAGS8_ENABLE_FORWARD_FACING_PRIMITIVE: u8 = 0x01; // Enable Forward Facing Primitive
pub const CONFIGURATION_BITS_FLAGS8_ENABLE_REVERSE_FACING_PRIMITIVE: u8 = 0x02; // Enable Reverse Facing Primitive
pub const CONFIGURATION_BITS_FLAGS8_CLOCKWISE_PRIMITIVES: u8 = 0x04; // Clockwise Primitives
pub const CONFIGURATION_BITS_FLAGS8_ENABLE_DEPTH_OFFSET: u8 = 0x08; // Enable Depth Offset
pub const CONFIGURATION_BITS_FLAGS8_ANTIALIASED_POINTS_LINES: u8 = 0x10; // Antialiased Points & Lines (Not Actually Supported)
pub const CONFIGURATION_BITS_FLAGS8_COVERAGE_READ_TYPE_LEVEL_4_8: u8 = 0x00; // Coverage Read Type = 4*8-Bit Level
pub const CONFIGURATION_BITS_FLAGS8_COVERAGE_READ_TYPE_MASK_16: u8 = 0x20; // Coverage Read Type = 16-Bit Mask
pub const CONFIGURATION_BITS_FLAGS8_RASTERISER_OVERSAMPLE_MODE_NONE: u8 = 0x00; // Rasteriser Oversample Mode = None
pub const CONFIGURATION_BITS_FLAGS8_RASTERISER_OVERSAMPLE_MODE_4X: u8 = 0x40; // Rasteriser Oversample Mode = 4X
pub const CONFIGURATION_BITS_FLAGS8_RASTERISER_OVERSAMPLE_MODE_16X: u8 = 0x80; // Rasteriser Oversample Mode = 16X
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_PIPE_SELECT: u16 = 0x0001; // Coverage Pipe Select
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_UPDATE_MODE_NON_ZERO: u16 = 0x0000; // Coverage Update Mode = Non Zero
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_UPDATE_MODE_ODD: u16 = 0x0002; // Coverage Update Mode = Odd
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_UPDATE_MODE_OR: u16 = 0x0004; // Coverage Update Mode = OR
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_UPDATE_MODE_ZERO: u16 = 0x0006; // Coverage Update Mode = Zero
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_READ_MODE_CLEAR_ON_READ: u16 = 0x0000; // Coverage Read Mode = Clear On Read
pub const CONFIGURATION_BITS_FLAGS16_COVERAGE_READ_MODE_LEAVE_ON_READ: u16 = 0x0008; // Coverage Read Mode = Leave On Read
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_NEVER: u16 = 0x0000; // Depth-Test Function = Never
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_LT: u16 = 0x0010; // Depth-Test Function = Less Than (LT)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_EQ: u16 = 0x0020; // Depth-Test Function = Equal (EQ)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_LE: u16 = 0x0030; // Depth-Test Function = Less Equal (LE)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_GT: u16 = 0x0040; // Depth-Test Function = Greater Than (GT)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_NE: u16 = 0x0050; // Depth-Test Function = Not Equal (NE)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_GE: u16 = 0x0060; // Depth-Test Function = Greater Equal (GE)
pub const CONFIGURATION_BITS_FLAGS16_DEPTH_TEST_FUNCTION_ALWAYS: u16 = 0x0070; // Depth-Test Function = Always
pub const CONFIGURATION_BITS_FLAGS16_Z_UPDATES_ENABLE: u16 = 0x0080; // Z Updates Enable
pub const CONFIGURATION_BITS_FLAGS16_EARLY_Z_ENABLE: u16 = 0x0100; // Early Z Enable
pub const CONFIGURATION_BITS_FLAGS16_EARLY_Z_UPDATES_ENABLE: u16 = 0x0200; // Early Z Updates Enable

pub const TILE_BINNING_FLAGS_MULTISAMPLE_MODE_4X: u8 = 0x01; // Multisample Mode (4X)
pub const TILE_BINNING_FLAGS_BUFFER_COLOR_DEPTH_64: u8 = 0x02; // Tile Buffer 64-Bit Color Depth
pub const TILE_BINNING_FLAGS_AUTO_INITIALISE_TILE_STATE_DATA_ARRAY: u8 = 0x04; // Auto-Initialise Tile State Data Array
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_INITIAL_BLOCK_SIZE_32: u8 = 0x00; // Tile Allocation Initial Block Size = 32 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_INITIAL_BLOCK_SIZE_64: u8 = 0x08; // Tile Allocation Initial Block Size = 64 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_INITIAL_BLOCK_SIZE_128: u8 = 0x10; // Tile Allocation Initial Block Size = 128 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_INITIAL_BLOCK_SIZE_256: u8 = 0x18; // Tile Allocation Initial Block Size = 256 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_BLOCK_SIZE_32: u8 = 0x00; // Tile Allocation Block Size = 32 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_BLOCK_SIZE_64: u8 = 0x20; // Tile Allocation Block Size = 64 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_BLOCK_SIZE_128: u8 = 0x40; // Tile Allocation Block Size = 128 Bytes
pub const TILE_BINNING_FLAGS_TILE_ALLOCATION_BLOCK_SIZE_256: u8 = 0x60; // Tile Allocation Block Size = 256 Bytes
pub const TILE_BINNING_FLAGS_DOUBLE_BUFFER_IN_NON_MS_MODE: u8 = 0x80; // Double-Buffer In Non-MS Mode

pub const TILE_RENDER_FLAGS_MULTISAMPLE_MODE_4X: u16 = 0x0001; // Multisample Mode (4X)
pub const TILE_RENDER_FLAGS_BUFFER_COLOR_DEPTH_64: u16 = 0x0002; // Tile Buffer 64-Bit Color Depth (HDR Mode)
pub const TILE_RENDER_FLAGS_FRAME_BUFFER_COLOR_FORMAT_BGR565_DITHERED: u16 = 0x0000; // Non-HDR Frame Buffer Color Format = BGR565 Dithered
pub const TILE_RENDER_FLAGS_FRAME_BUFFER_COLOR_FORMAT_RGBA8888: u16 = 0x0004; // Non-HDR Frame Buffer Color Format = RGBA8888
pub const TILE_RENDER_FLAGS_FRAME_BUFFER_COLOR_FORMAT_BGR565_NO_DITHER: u16 = 0x0008; // Non-HDR Frame Buffer Color Format = BGR565 No Dither
pub const TILE_RENDER_FLAGS_DECIMATE_MODE_1X: u16 = 0x0000; // Decimate Mode = 1X
pub const TILE_RENDER_FLAGS_DECIMATE_MODE_4X: u16 = 0x0010; // Decimate Mode = 4X
pub const TILE_RENDER_FLAGS_DECIMATE_MODE_16X: u16 = 0x0020; // Decimate Mode = 16X
pub const TILE_RENDER_FLAGS_MEMORY_FORMAT_LINEAR: u16 = 0x0000; // Memory Format = Linear
pub const TILE_RENDER_FLAGS_MEMORY_FORMAT_T_FORMAT: u16 = 0x0040; // Memory Format = T-Format
pub const TILE_RENDER_FLAGS_MEMORY_FORMAT_LT_FORMAT: u16 = 0x0080; // Memory Format = LT-Format
pub const TILE_RENDER_FLAGS_ENABLE_VG_MASK_BUFFER: u16 = 0x0100; // Enable VG Mask Buffer
pub const TILE_RENDER_FLAGS_SELECT_COVERAGE_MODE: u16 = 0x0200; // Select Coverage Mode
pub const TILE_RENDER_FLAGS_EARLY_Z_UPDATE_DIRECTION_LT_LE: u16 = 0x0000; // Early-Z Update Direction = LT/LE
pub const TILE_RENDER_FLAGS_EARLY_Z_UPDATE_DIRECTION_GT_GE: u16 = 0x0400; // Early-Z Update Direction = GT/GE
pub const TILE_RENDER_FLAGS_EARLY_Z_EARLY_COV_DISABLE: u16 = 0x0800; // Early-Z/Early-Cov Disable
pub const TILE_RENDER_FLAGS_DOUBLE_BUFFER_IN_NON_MS_MODE: u16 = 0x1000; // Double-Buffer In Non-MS Mode

pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_NONE: u16 = 0x0000; // Buffer To Store = None
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_COLOR: u16 = 0x0001; // Buffer To Store = Color
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_Z_STENCIL: u16 = 0x0002; // Buffer To Store = Z/Stencil
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_Z_ONLY: u16 = 0x0003; // Buffer To Store = Z-Only
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_VG_MASK: u16 = 0x0004; // Buffer To Store = VG-Mask
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_STORE_FULL_DUMP: u16 = 0x0005; // Buffer To Store = Full Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_FORMAT_RASTER: u16 = 0x0000; // Format = Raster Format
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_FORMAT_T: u16 = 0x0010; // Format = T-Format
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_FORMAT_LT: u16 = 0x0020; // Format = LT-Format
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_MODE_SAMPLE: u16 = 0x0000; // Mode = Sample
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_MODE_DECIMATE_4X: u16 = 0x0040; // Mode = Decimate 4X
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_MODE_DECIMATE_16X: u16 = 0x0080; // Mode = Decimate 16X
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_COLOR_FORMAT_RGBA8888: u16 = 0x0000; // Pixel Color Format = RGBA8888
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_COLOR_FORMAT_BGR565_DITHERED: u16 = 0x0100; // Pixel Color Format = BGR565 Dithered
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_COLOR_FORMAT_BGR565_NO_DITHER: u16 = 0x0200; // Pixel Color Format = BGR565 No Dither
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_DISABLE_DOUBLE_BUFFER_SWAP: u16 = 0x1000; // Disable Double-Buffer Swap In Double Buffer Mode
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_DISABLE_COLOR_BUFFER_CLEAR: u16 = 0x2000; // Disable Color Buffer Clear On Store/Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_DISABLE_Z_STENCIL_BUFFER_CLEAR: u16 = 0x4000; // Disable Z/Stencil Buffer Clear On Store/Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS16_DISABLE_VG_MASK_BUFFER_CLEAR: u16 = 0x8000; // Disable VG-Mask Buffer Clear On Store/Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS32_DISABLE_COLOR_BUFFER_DUMP: u32 = 0x00000001; // Disable Color Buffer Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS32_DISABLE_Z_STENCIL_BUFFER_DUMP: u32 = 0x00000002; // Disable Z/Stencil Buffer Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS32_DISABLE_VG_MASK_BUFFER_DUMP: u32 = 0x00000004; // Disable VG-Mask Buffer Dump
pub const STORE_TILE_BUFFER_GENERAL_FLAGS32_LAST_TILE_OF_FRAME: u32 = 0x00000008; // Last Tile Of Frame

pub struct CommandBuilder<'a> {
    cursor: Cursor<&'a mut [u8]>,
}

impl<'a> CommandBuilder<'a> {
    pub fn new(storage: &'a mut [u8]) -> CommandBuilder<'a> {
        CommandBuilder {
            cursor: Cursor::new(storage),
        }
    }

    pub fn end(self) -> u32 {
        self.cursor.position() as u32
    }

    pub fn halt(&mut self) {
        self.cursor.write_u8(CMD_HALT).unwrap();
    }

    pub fn no_op(&mut self) {
        self.cursor.write_u8(CMD_NO_OP).unwrap();
    }

    // Add Return-From-Sub-List To Tile Lists & Then Flush Tile Lists To Memory (B)
    pub fn flush(&mut self) {
        self.cursor.write_u8(CMD_FLUSH).unwrap();
    }

    // Same As Flush, But Preceded By The Forced Writing Of The Current State To The Tile Lists (B)
    pub fn flush_all_state(&mut self) {
        self.cursor.write_u8(CMD_FLUSH_ALL_STATE).unwrap();
    }

    // Advances State Counter So That Initial State Items Actually Go Into Tile Lists (B)
    pub fn start_tile_binning(&mut self) {
        self.cursor.write_u8(CMD_START_TILE_BINNING).unwrap();
    }

    // After Tile Lists Are Flushed Or Last Tile Written
    pub fn increment_semaphore(&mut self) {
        self.cursor.write_u8(CMD_INCREMENT_SEMAPHORE).unwrap();
    }

    // Wait For Frame To Complete In Other Thread
    pub fn wait_on_semaphore(&mut self) {
        self.cursor.write_u8(CMD_WAIT_ON_SEMAPHORE).unwrap();
    }

    pub fn branch_to_sub_list(&mut self, address: u32) {
        self.cursor.write_u8(CMD_BRANCH_TO_SUB_LIST).unwrap();
        self.cursor.write_u32::<LittleEndian>(address).unwrap();
    }

    pub fn store_multi_sample(&mut self) {
        self.cursor.write_u8(CMD_STORE_MULTI_SAMPLE).unwrap();
    }

    pub fn store_multi_sample_end(&mut self) {
        self.cursor.write_u8(CMD_STORE_MULTI_SAMPLE_END).unwrap();
    }

    pub fn vertex_array_primitives(
        &mut self,
        primitive_mode: u8,
        length: u32, // Length (Number Of Vertices)
        index: u32,
    ) // Index Of First Vertex
    {
        self.cursor.write_u8(CMD_VERTEX_ARRAY_PRIMITIVES).unwrap();
        self.cursor.write_u8(primitive_mode).unwrap();
        self.cursor.write_u32::<LittleEndian>(length).unwrap();
        self.cursor.write_u32::<LittleEndian>(index).unwrap();
    }

    pub fn nv_shader_state(&mut self, nv_shader_state_bus_address: u32) {
        self.cursor.write_u8(CMD_NV_SHADER_STATE).unwrap();
        self.cursor
            .write_u32::<LittleEndian>(nv_shader_state_bus_address)
            .unwrap();
    }

    pub fn configuration_bits(&mut self, flags8: u8, flags16: u16) {
        self.cursor.write_u8(CMD_CONFIGURATION_BITS).unwrap();
        self.cursor.write_u8(flags8).unwrap();
        self.cursor.write_u16::<LittleEndian>(flags16).unwrap();
    }

    pub fn clip_window(&mut self, left: u16, bottom: u16, width: u16, height: u16) {
        self.cursor.write_u8(CMD_CLIP_WINDOW).unwrap();
        self.cursor.write_u16::<LittleEndian>(left).unwrap();
        self.cursor.write_u16::<LittleEndian>(bottom).unwrap();
        self.cursor.write_u16::<LittleEndian>(width).unwrap();
        self.cursor.write_u16::<LittleEndian>(height).unwrap();
    }

    pub fn viewport_offset(&mut self, x: i16, y: i16) {
        self.cursor.write_u8(CMD_VIEWPORT_OFFSET).unwrap();
        self.cursor.write_i16::<LittleEndian>(x).unwrap();
        self.cursor.write_i16::<LittleEndian>(y).unwrap();
    }

    pub fn tile_binning_mode_configuration(
        &mut self,
        address: u32,     // Tile Allocation Memory Address
        size: u32,        // Tile Allocation Memory Size
        baseaddress: u32, // Tile State Data Array Base Address (16-Byte Aligned, Size Of 48 Bytes * Num Tiles)
        width: u8,
        height: u8,
        flags: u8,
    ) {
        self.cursor
            .write_u8(CMD_TILE_BINNING_MODE_CONFIGURATION)
            .unwrap();
        self.cursor.write_u32::<LittleEndian>(address).unwrap();
        self.cursor.write_u32::<LittleEndian>(size).unwrap();
        self.cursor.write_u32::<LittleEndian>(baseaddress).unwrap();
        self.cursor.write_u8(width).unwrap();
        self.cursor.write_u8(height).unwrap();
        self.cursor.write_u8(flags).unwrap();
    }

    pub fn tile_rendering_mode_configuration(
        &mut self,
        address: u32,
        width: u16,
        height: u16,
        flags: u16,
    ) {
        self.cursor
            .write_u8(CMD_TILE_RENDERING_MODE_CONFIGURATION)
            .unwrap();
        self.cursor.write_u32::<LittleEndian>(address).unwrap();
        self.cursor.write_u16::<LittleEndian>(width).unwrap();
        self.cursor.write_u16::<LittleEndian>(height).unwrap();
        self.cursor.write_u16::<LittleEndian>(flags).unwrap();
    }

    pub fn clear_colors(
        &mut self,
        clearcolor: u64, // Clear Color (2X RGBA8888 Or RGBA16161616)
        clearvgmask: u8,
        clearzs: u32,
        clearstencil: u8,
    ) {
        self.cursor.write_u8(CMD_CLEAR_COLORS).unwrap();
        self.cursor.write_u64::<LittleEndian>(clearcolor).unwrap();
        self.cursor
            .write_u32::<LittleEndian>((clearvgmask as u32 * 0x1000000) + clearzs)
            .unwrap();
        self.cursor.write_u8(clearstencil).unwrap();
    }

    pub fn tile_coordinates(&mut self, column: i8, row: i8) {
        self.cursor.write_u8(CMD_TILE_COORDINATES).unwrap();
        self.cursor.write_i8(column).unwrap();
        self.cursor.write_i8(row).unwrap();
    }

    pub fn store_tile_buffer_general(&mut self, flags16: u16, flags32: u32, address: u32) {
        self.cursor.write_u8(CMD_STORE_TILE_BUFFER_GENERAL).unwrap();
        self.cursor.write_u16::<LittleEndian>(flags16).unwrap();
        self.cursor
            .write_u32::<LittleEndian>(flags32 << 24 | address)
            .unwrap();
    }
}
