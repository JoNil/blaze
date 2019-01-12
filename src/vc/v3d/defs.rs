#![allow(dead_code)]

// Raspberry Pi VideoCoreIV

// V3D Register Address Map
pub const BASE: u32 = 0x3FC00000; // Address ($20C00000 PHYSICAL, $7EC00000 BUS)
pub const IDENT0: u32 =  0x00000; // 0 (V3D Block Identity)
pub const IDENT1: u32 =  0x00004; // 1 (V3D Configuration A)
pub const IDENT2: u32 =  0x00008; // 2 (V3D Configuration B)
pub const IDENT3: u32 =  0x0000C; // 3 (V3D Configuration C)
pub const SCRATCH: u32 = 0x00010; // Register
pub const L2CACTL: u32 = 0x00020; // Cache Control
pub const SLCACTL: u32 = 0x00024; // Cache Control
pub const INTCTL: u32 =  0x00030; // Control
pub const INTENA: u32 =  0x00034; // Enables
pub const INTDIS: u32 =  0x00038; // Disables
pub const CT0CS: u32 =   0x00100; // List Executor Thread 0 Control & Status
pub const CT1CS: u32 =   0x00104; // List Executor Thread 1 Control & Status
pub const CT0EA: u32 =   0x00108; // List Executor Thread 0 End Address
pub const CT1EA: u32 =   0x0010C; // List Executor Thread 1 End Address
pub const CT0CA: u32 =   0x00110; // List Executor Thread 0 Current Address
pub const CT1CA: u32 =   0x00114; // List Executor Thread 1 Current Address
pub const CT0RA0: u32 =  0x00118; // List Executor Thread 0 Return Address
pub const CT1RA0: u32 =  0x0011C; // List Executor Thread 1 Return Address
pub const CT0LC: u32 =   0x00120; // List Executor Thread 0 List Counter
pub const CT1LC: u32 =   0x00124; // List Executor Thread 1 List Counter
pub const CT0PC: u32 =   0x00128; // List Executor Thread 0 Primitive List Counter
pub const CT1PC: u32 =   0x0012C; // List Executor Thread 1 Primitive List Counter
pub const PCS: u32 =     0x00130; // Control & Status
pub const BFC: u32 =     0x00134; // Mode Flush Count
pub const RFC: u32 =     0x00138; // Mode Frame Count
pub const BPCA: u32 =    0x00300; // Address Of Binning Memory Pool
pub const BPCS: u32 =    0x00304; // Size Of Binning Memory Pool
pub const BPOA: u32 =    0x00308; // Of Overspill Binning Memory Block
pub const BPOS: u32 =    0x0030C; // Of Overspill Binning Memory Block
pub const BXCF: u32 =    0x00310; // Debug
pub const SQRSV0: u32 =  0x00410; // QPUs 0-7
pub const SQRSV1: u32 =  0x00414; // QPUs 8-15
pub const SQCNTL: u32 =  0x00418; // Scheduler Control
pub const SQCSTAT: u32 = 0x0041C; // Scheduler State
pub const SRQPC: u32 =   0x00430; // User Program Request Program Address
pub const SRQUA: u32 =   0x00434; // User Program Request Uniforms Address
pub const SRQUL: u32 =   0x00438; // User Program Request Uniforms Length
pub const SRQCS: u32 =   0x0043C; // User Program Request Control & Status
pub const VPACNTL: u32 = 0x00500; // Allocator Control
pub const VPMBASE: u32 = 0x00504; // Base (User) Memory Reservation
pub const PCTRC: u32 =   0x00670; // Counter Clear
pub const PCTRE: u32 =   0x00674; // Counter Enables
pub const PCTR0: u32 =   0x00680; // Counter Count 0
pub const PCTRS0: u32 =  0x00684; // Counter Mapping 0
pub const PCTR1: u32 =   0x00688; // Counter Count 1
pub const PCTRS1: u32 =  0x0068C; // Counter Mapping 1
pub const PCTR2: u32 =   0x00690; // Counter Count 2
pub const PCTRS2: u32 =  0x00694; // Counter Mapping 2
pub const PCTR3: u32 =   0x00698; // Counter Count 3
pub const PCTRS3: u32 =  0x0069C; // Counter Mapping 3
pub const PCTR4: u32 =   0x006A0; // Counter Count 4
pub const PCTRS4: u32 =  0x006A4; // Counter Mapping 4
pub const PCTR5: u32 =   0x006A8; // Counter Count 5
pub const PCTRS5: u32 =  0x006AC; // Counter Mapping 5
pub const PCTR6: u32 =   0x006B0; // Counter Count 6
pub const PCTRS6: u32 =  0x006B4; // Counter Mapping 6
pub const PCTR7: u32 =   0x006B8; // Counter Count 7
pub const PCTRS7: u32 =  0x006BC; // Counter Mapping 7
pub const PCTR8: u32 =   0x006C0; // Counter Count 8
pub const PCTRS8: u32 =  0x006C4; // Counter Mapping 8
pub const PCTR9: u32 =   0x006C8; // Counter Count 9
pub const PCTRS9: u32 =  0x006CC; // Counter Mapping 9
pub const PCTR10: u32 =  0x006D0; // Counter Count 10
pub const PCTRS10: u32 = 0x006D4; // Counter Mapping 10
pub const PCTR11: u32 =  0x006D8; // Counter Count 11
pub const PCTRS11: u32 = 0x006DC; // Counter Mapping 11
pub const PCTR12: u32 =  0x006E0; // Counter Count 12
pub const PCTRS12: u32 = 0x006E4; // Counter Mapping 12
pub const PCTR13: u32 =  0x006E8; // Counter Count 13
pub const PCTRS13: u32 = 0x006EC; // Counter Mapping 13
pub const PCTR14: u32 =  0x006F0; // Counter Count 14
pub const PCTRS14: u32 = 0x006F4; // Counter Mapping 14
pub const PCTR15: u32 =  0x006F8; // Counter Count 15
pub const PCTRS15: u32 = 0x006FC; // Counter Mapping 15
pub const DBCFG: u32 =   0x00E00; //
pub const DBSCS: u32 =   0x00E04; // Control & Status
pub const DBSCFG: u32 =  0x00E08; // Configure
pub const DBSSR: u32 =   0x00E0C; // SR
pub const DBSDR0: u32 =  0x00E10; // R0
pub const DBSDR1: u32 =  0x00E14; // R1
pub const DBSDR2: u32 =  0x00E18; // R2
pub const DBSDR3: u32 =  0x00E1C; // R3
pub const DBQRUN: u32 =  0x00E20; // Run
pub const DBQHLT: u32 =  0x00E24; // Halt
pub const DBQSTP: u32 =  0x00E28; // Step
pub const DBQITE: u32 =  0x00E2C; // Interrupt Enables
pub const DBQITC: u32 =  0x00E30; // Interrupt Control
pub const DBQGHC: u32 =  0x00E34; // GHC
pub const DBQGHG: u32 =  0x00E38; // GHG
pub const DBQGHH: u32 =  0x00E3C; // GHH
pub const DBGE: u32 =    0x00F00; // Error Signals
pub const FDBGO: u32 =   0x00F04; // Overrun Error Signals
pub const FDBGB: u32 =   0x00F08; // Interface Ready & Stall Signals, FEP Busy Signals
pub const FDBGR: u32 =   0x00F0C; // Internal Ready Signals
pub const FDBGS: u32 =   0x00F10; // Internal Stall Input Signals
pub const ERRSTAT: u32 = 0x00F20; // Error Signals (VPM, VDW, VCD, VCM, L2C)


// V3D Identity Registers

// V3D_IDENT0: V3D Identification 0 (V3D Block Identity) Register Description
pub const IDENT0_IDSTR: u32 = 0x00FFFFFF; // V3D ID String (Reads As "V3D") READ
pub const IDENT0_TVER: u32 =  0xFF000000; // V3D Technology Version (Reads Technology Version = 2) READ

// V3D_IDENT1: V3D Identification 1 (V3D Configuration A) Register Description
pub const IDENT1_REVR: u32 =  0x0000000F; // V3D Revision READ
pub const IDENT1_NSLC: u32 =  0x000000F0; // Number Of Slices READ
pub const IDENT1_QUPS: u32 =  0x00000F00; // Number Of QPUs Per Slice READ
pub const IDENT1_TUPS: u32 =  0x0000F000; // Number Of TMUs Per Slice READ
pub const IDENT1_NSEM: u32 =  0x00FF0000; // Number Of Semaphores READ
pub const IDENT1_HDRT: u32 =  0x0F000000; // HDR Support (0 = Not Supported, 1 = Supported) READ
pub const IDENT1_VPMSZ: u32 = 0xF0000000; // VPM Memory Size (Multiples Of 1K, 0 => 16K) READ

// V3D_IDENT2: V3D Identification 2 (V3D Configuration B) Register Description
pub const IDENT2_VRISZ: u32 = 0x0000000F; // VRI Memory Size (0 = Half Size, 1 = Full Size) READ
pub const IDENT2_TLBSZ: u32 = 0x000000F0; // Tile Buffer Size (0 = Quarter Size, 1 = Half Size, 2 = Full Size (32x32msm)) READ
pub const IDENT2_TLBDB: u32 = 0x00000F00; // Tile Buffer Double-Buffer Mode Support (0 = Not Supported, 1 = Supported) READ


// V3D Miscellaneous Registers

// V3D_SCRATCH: V3D Scratch Register Description
pub const SCRATCH_SCRATCH: u32 = 0xFFFFFFFF; // Scratch Register (Read/Write Registers For General Purposes) READ/WRITE


// V3D Cache Control Registers

// V3D_L2CACTL: V3D L2 Cache Control Register Description
pub const L2CACTL_L2CENA: u32 = 0x00000001; // L2 Cache Enable (Reads State Of Cache Enable Bit, Write To Enable The L2 Cache) READ/WRITE
pub const L2CACTL_L2CDIS: u32 = 0x00000002; // L2 Cache Disable (Write To Disable The L2 Cache) WRITE
pub const L2CACTL_L2CCLR: u32 = 0x00000004; // L2 Cache Clear (Write To Clear The L2 Cache) WRITE

// V3D_SLCACTL: V3D Slices Cache Control Register Description
pub const SLCACTL_ICCS0_TO_ICCS3: u32 =   0x0000000F; // Instruction Cache Clear Bits (Write To Clear Instruction Cache) WRITE
pub const SLCACTL_UCCS0_TO_UCCS3: u32 =   0x00000F00; // Uniforms Cache Clear Bits (Write To Clear Uniforms Cache) WRITE
pub const SLCACTL_T0CCS0_TO_T0CCS3: u32 = 0x000F0000; // TMU0 Cache Clear Bits (Write To Clear TMU0 Cache) WRITE
pub const SLCACTL_T1CCS0_TO_T1CCS3: u32 = 0x0F000000; // TMU1 Cache Clear Bits (Write To Clear TMU1 Cache) WRITE


// V3D Pipeline Interrupt Control

// V3D_INTCTL: V3D Interrupt Control Register Description
pub const INTCTL_INT_FRDONE: u32 =   0x00000001; // Render Mode Frame Done Interrupt Status (Set When All Tiles Of The Frame Have Been Written To Memory) READ/WRITE
pub const INTCTL_INT_FLDONE: u32 =   0x00000002; // Binning Mode Flush Done Interrupt Status (Set When Binning Is Complete With All Tile Lists Flushed To Memory) READ/WRITE
pub const INTCTL_INT_OUTOMEM: u32 =  0x00000004; // Binner Out Of Memory Interrupt Status (Set While The Binner Needs More Memory To Complete) READ/WRITE
pub const INTCTL_INT_SPILLUSE: u32 = 0x00000008; // Binner Used Overspill Memory Interrupt Status (Set When The Binner Starts Using The (Valid) Overspill Memory Buffer) READ/WRITE

// V3D_INTENA: V3D Interrupt Enables Register Description
pub const INTENA_EI_FRDONE: u32 =   0x00000001; // Render Mode Frame Done Interrupt Enable (Set When The INT_FRDONE Interrupt Is Set) READ/WRITE
pub const INTENA_EI_FLDONE: u32 =   0x00000002; // Binning Mode Flush Done Interrupt Enable (Set When The INT_FLDONE Interrupt Is Set) READ/WRITE
pub const INTENA_EI_OUTOMEM: u32 =  0x00000004; // Binner Out Of Memory Interrupt Enable (Set When The INT_OUTOMEM Interrupt Is Set) READ/WRITE
pub const INTENA_EI_SPILLUSE: u32 = 0x00000008; // Binner Used Overspill Memory Interrupt Enable (Set When The INT_SPILLUSE Interrupt Is Set) READ/WRITE

// V3D_INTDIS: V3D Interrupt Disables Register Description
pub const INTDIS_DI_FRDONE: u32 =   0x00000001; // Render Mode Frame Done Interrupt Disable (Set When The INT_FRDONE Interrupt Is Set) READ/WRITE
pub const INTDIS_DI_FLDONE: u32 =   0x00000002; // Binning Mode Flush Done Interrupt Disable (Set When The INT_FLDONE Interrupt Is Set) READ/WRITE
pub const INTDIS_DI_OUTOMEM: u32 =  0x00000004; // Binner Out Of Memory Interrupt Disable (Set When The INT_OUTOMEM Interrupt Is Set) READ/WRITE
pub const INTDIS_DI_SPILLUSE: u32 = 0x00000008; // Binner Used Overspill Memory Interrupt Disable (Set When The INT_SPILLUSE Interrupt Is Set) READ/WRITE


// V3D Control List Executor Registers (Per Thread)

// V3D_CTnCS: V3D Control List Executor Thread n Control & Status Register Description
pub const CTNCS_CTMODE: u32 = 0x00000001; // Control Thread Mode (Binning Mode Thread Only) READ
pub const CTNCS_CTERR: u32 =  0x00000008; // Control Thread Error (Set When Stopped With An Error, Cleared On Restarting) READ
pub const CTNCS_CTSUBS: u32 = 0x00000010; // Control Thread Sub-Mode READ/WRITE
pub const CTNCS_CTRUN: u32 =  0x00000020; // Control Thread Run READ/WRITE
pub const CTNCS_CTRTSD: u32 = 0x00000300; // Return Stack Depth (Number Of Levels Of List Nesting) READ
pub const CTNCS_CTSEMA: u32 = 0x00007000; // Counting Semaphore (Current State Of The Counting Semaphore For This Thread) READ
pub const CTNCS_CTRSTA: u32 = 0x00008000; // Reset Bit (Writing 1 Stops The Control Thread & Resets All Bits In This Register) WRITE

// V3D_CTnEA: V3D Control List Executor Thread n End Address Register Description
pub const CTNEA_CTLEA: u32 = 0xFFFFFFFF; // Control List End Address (Set To The Byte Address After The Last Record In The Control List) READ/WRITE

// V3D_CTnCA: V3D Control List Executor Thread n Current Address Register Description
pub const CTNCA_CTLCA: u32 = 0xFFFFFFFF; // Control List Current Address (Points To The Address Of The Current Record In The Control List) READ/WRITE

// V3D_CTnRA0: V3D Control List Executor Thread n Return Address Register Description
pub const CTNRA0_CTLRA: u32 = 0xFFFFFFFF; // Control List Return Address 0 (Address On Return Address Stack) READ

// V3D_CTnLC: V3D Control List Executor Thread n List Counter Register Description
pub const CTNLC_CTLSLCS: u32 = 0x0000FFFF; // Sub-list Counter (Count Of Return Commands Encountered) READ/WRITE
pub const CTNLC_CTLLCM: u32 =  0xFFFF0000; // Major List Counter (Count Of Flush Commands Encountered) READ/WRITE

// V3D_CTnPC: V3D Control List Executor Thread n Primitive List Counter Register Description
pub const CTNPC_CTLPC: u32 = 0xFFFFFFFF; // Primitive List Counter (Count Of Primitives Remaining Whilst Processing A Primitive List) READ


// V3D Pipeline Registers

// V3D_PCS: V3D Pipeline Control & Status Register Description
pub const PCS_BMACTIVE: u32 = 0x00000001; // Binning Mode Active (Set While Binning Pipeline Is In Use) READ
pub const PCS_BMBUSY: u32 =   0x00000002; // Binning Mode Busy (Set While Any Binning Operations Are Actually In Progress) READ
pub const PCS_RMACTIVE: u32 = 0x00000004; // Rendering Mode Active (Set While Rendering Pipeline Is In Use) READ
pub const PCS_RMBUSY: u32 =   0x00000008; // Rendering Mode Busy (Set While Any Rendering Operations Are Actually In Progress) READ
pub const PCS_BMOOM: u32 =    0x00000100; // Binning Mode Out Of Memory (Set When PTB Runs Out Of Binning Memory While Binning) READ

// V3D_BFC: V3D Binning Mode Flush Count Register Description
pub const BFC_BMFCT: u32 = 0x000000FF; // Flush Count (Count Increments In Binning Mode Once PTB Has Flushed All Tile Lists To Mem & PTB Has Finished With Tile State Data Array) READ/WRITE

// V3D_RFC: V3D Rendering Mode Frame Count Register Description
pub const RFC_RMFCT: u32 = 0x000000FF; // Frame Count (Count Increments In Rendering Mode When Last Tile Store Operation Of Frame Completes, The Tile Has Fully Written Out To Mem) READ/WRITE

// V3D_BPCA: V3D Current Address Of Binning Memory Pool Register Description
pub const BPCA_BMPCA: u32 = 0xFFFFFFFF; // Current Pool Address (The Address Of The Current Allocation Pointer In The Binning Memory Pool) READ

// V3D_BPCS: V3D Remaining Size Of Binning Memory Pool Register Description
pub const BPCS_BMPRS: u32 = 0xFFFFFFFF; // Size Of Pool Remaining (The Number Of Bytes Remaining In The Binning Memory Pool) READ

// V3D_BPOA: V3D Address Of Overspill Binning Memory Block Register Description
pub const BPOA_BMPOA: u32 = 0xFFFFFFFF; // Address Of Overspill Memory Block For Binning (Address Of Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out) READ/WRITE

// V3D_BPOS: V3D Size Of Overspill Binning Memory Block Register Description
pub const BPOS_BMPOS: u32 = 0xFFFFFFFF; // Size Of Overspill Memory Block For Binning (Number Of Bytes Of Additional Mem That PTB Can Use For Binning Once Initial Pool Runs Out) READ/WRITE


// V3D QPU Scheduler Registers

// V3D_SQRSV0: V3D Reserve QPUs 0-7 Register Description
pub const SQRSV0_QPURSV0: u32 = 0x0000000F; // Reservation Settings For QPU 0 READ/WRITE
pub const SQRSV0_QPURSV1: u32 = 0x000000F0; // Reservation Settings For QPU 1 READ/WRITE
pub const SQRSV0_QPURSV2: u32 = 0x00000F00; // Reservation Settings For QPU 2 READ/WRITE
pub const SQRSV0_QPURSV3: u32 = 0x0000F000; // Reservation Settings For QPU 3 READ/WRITE
pub const SQRSV0_QPURSV4: u32 = 0x000F0000; // Reservation Settings For QPU 4 READ/WRITE
pub const SQRSV0_QPURSV5: u32 = 0x00F00000; // Reservation Settings For QPU 5 READ/WRITE
pub const SQRSV0_QPURSV6: u32 = 0x0F000000; // Reservation Settings For QPU 6 READ/WRITE
pub const SQRSV0_QPURSV7: u32 = 0xF0000000; // Reservation Settings For QPU 7 READ/WRITE

// V3D_SQRSV1: V3D Reserve QPUs 8-15 Register Description
pub const SQRSV1_QPURSV8: u32 =  0x0000000F; // Reservation Settings For QPU 8 READ/WRITE
pub const SQRSV1_QPURSV9: u32 =  0x000000F0; // Reservation Settings For QPU 9 READ/WRITE
pub const SQRSV1_QPURSV10: u32 = 0x00000F00; // Reservation Settings For QPU 10 READ/WRITE
pub const SQRSV1_QPURSV11: u32 = 0x0000F000; // Reservation Settings For QPU 11 READ/WRITE
pub const SQRSV1_QPURSV12: u32 = 0x000F0000; // Reservation Settings For QPU 12 READ/WRITE
pub const SQRSV1_QPURSV13: u32 = 0x00F00000; // Reservation Settings For QPU 13 READ/WRITE
pub const SQRSV1_QPURSV14: u32 = 0x0F000000; // Reservation Settings For QPU 14 READ/WRITE
pub const SQRSV1_QPURSV15: u32 = 0xF0000000; // Reservation Settings For QPU 15 READ/WRITE

// V3D_SQCNTL: V3D QPU Scheduler Control Register Description
pub const SQCNTL_VSRBL: u32 = 0x00000003; // Vertex Shader Scheduling Bypass Limit READ/WRITE
pub const SQCNTL_CSRBL: u32 = 0x0000000C; // Coordinate Shader Scheduling Bypass Limit READ/WRITE

// V3D_SRQPC: V3D QPU User Program Request Program Address Register Description
pub const SRQPC_QPURQPC: u32 = 0xFFFFFFFF; // Program Address (Writing This Register Queues A Request To Run A Program Starting At The Given Address) WRITE

// V3D_SRQUA: V3D QPU User Program Request Uniforms Address Register Description
pub const SRQUA_QPURQUA: u32 = 0xFFFFFFFF; // Uniforms Address (Contains The Address Of The Uniforms Stream For The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE

// V3D_SRQUL: V3D QPU User Program Request Uniforms Length Register Description
pub const SRQUL_QPURQUL: u32 = 0x00000FFF; // Uniforms Length (Contains The Max Length Of The Uniforms Stream For The Next User Program To Be Queued Via A Write To V3DRQPC) READ/WRITE

// V3D_SRQCS: V3D QPU User Program Request Control & Status Register Description
pub const SRQCS_QPURQL: u32 =   0x0000003F; // Queue Length (Contains The Number Of Program Requests Currently Queued) READ/WRITE
pub const SRQCS_QPURQERR: u32 = 0x00000080; // Queue Error (Set When A Request Has Been Made When The Queue Is Full) READ/WRITE
pub const SRQCS_QPURQCM: u32 =  0x0000FF00; // Count Of User Program Requests Made (Contains The Total Number Of User Program Requests Made, Modulo 256) READ/WRITE
pub const SRQCS_QPURQCC: u32 =  0x00FF0000; // Count Of User Programs Completed (Contains The Total Number Of User Programs That Have Run & Completed, Modulo 256) READ/WRITE


// V3D VPM Registers

// V3D_VPACNTL: V3D VPM Allocator Control Register Description
pub const VPACNTL_VPARALIM: u32 = 0x00000007; // Rendering VPM Allocation Limit (Limits The Amount Of VPM Memory Allocated To Rendering Mode) READ/WRITE
pub const VPACNTL_VPABALIM: u32 = 0x00000038; // Binning VPM Allocation Limit (Limits The Amount Of VPM Memory Allocated To Binning Mode) READ/WRITE
pub const VPACNTL_VPARATO: u32 =  0x000001C0; // Rendering VPM Allocation Timeout (Sets A Timeout For Raising The Priority Of Rendering Mode Allocation Requests) READ/WRITE
pub const VPACNTL_VPABATO: u32 =  0x00000E00; // Binning VPM Allocation Timeout (Sets A Timeout For Raising The Priority Of Binning Mode Allocation Requests) READ/WRITE
pub const VPACNTL_VPALIMEN: u32 = 0x00001000; // Enable VPM Allocation Limits (Enables VPM Memory Allocation Limiting Using VPARALIM & VPABALIM) READ/WRITE
pub const VPACNTL_VPATOEN: u32 =  0x00002000; // Enable VPM Allocation Timeout (Enables VPM Memory Allocation Timeout Using VPARATO & VPABATO) READ/WRITE

// V3D_VPMBASE: V3D VPM Base (User) Memory Reservation Register Description
pub const VPMBASE_VPMURSV: u32 = 0x0000001F; // VPM Memory Reserved For User Programs (Contains Amount Of VPM Mem Reserved For All User Programs, In Multiples Of 256 Bytes) READ/WRITE


// V3D QPU Interrupt Control

// V3D_DBQITE: V3D QPU Interrupt Enables Register Description
pub const DBQITE_IE_QPU0_TO_IE_QPU15: u32 = 0x0000FFFF; // QPU Interrupt Enable bits (Set Bit To Allow QPU To Generate An Interrupt) READ/WRITE

// V3D_DBQITC: V3D QPU Interrupt Control Register Description
pub const DBQITC_IC_QPU0_TO_IC_QPU15: u32 = 0x0000FFFF; // QPU Interrupt Control Bits (Reads When Interrupt Is Latched, Write To Clear Interrupt) READ/WRITE


// V3D Performance Counters

// V3D Sources For Performance Counters
pub const COUNT_ID_0: u32 =   0; // FEP Valid Primitives That Result In No Rendered Pixels, For All Rendered Tiles
pub const COUNT_ID_1: u32 =   1; // FEP Valid Primitives For All Rendered Tiles (Primitives May Be Counted In More Than One Tile)
pub const COUNT_ID_2: u32 =   2; // FEP Early-Z/Near/Far Clipped Quads
pub const COUNT_ID_3: u32 =   3; // FEP Valid Quads
pub const COUNT_ID_4: u32 =   4; // TLB Quads With No Pixels Passing The Stencil Test
pub const COUNT_ID_5: u32 =   5; // TLB Quads With No Pixels Passing The Z & Stencil Tests
pub const COUNT_ID_6: u32 =   6; // TLB Quads With Any Pixels Passing The Z & Stencil Tests
pub const COUNT_ID_7: u32 =   7; // TLB Quads With All Pixels Having Zero Coverage
pub const COUNT_ID_8: u32 =   8; // TLB Quads With Any Pixels Having Non-Zero Coverage
pub const COUNT_ID_9: u32 =   9; // TLB Quads With Valid Pixels Written To Color Buffer
pub const COUNT_ID_10: u32 = 10; // PTB Primitives Discarded By Being Outside The Viewport
pub const COUNT_ID_11: u32 = 11; // PTB Primitives That Need Clipping
pub const COUNT_ID_12: u32 = 12; // PSE Primitives That Are Discarded Because They Are Reversed
pub const COUNT_ID_13: u32 = 13; // QPU Total Idle Clock Cycles For All QPUs
pub const COUNT_ID_14: u32 = 14; // QPU Total Clock Cycles For All QPUs Doing Vertex/Coordinate Shading
pub const COUNT_ID_15: u32 = 15; // QPU Total Clock Cycles For All QPUs Doing Fragment Shading
pub const COUNT_ID_16: u32 = 16; // QPU Total Clock Cycles For All QPUs Executing Valid Instructions
pub const COUNT_ID_17: u32 = 17; // QPU Total Clock Cycles For All QPUs Stalled Waiting For TMUs
pub const COUNT_ID_18: u32 = 18; // QPU Total Clock Cycles For All QPUs Stalled Waiting For Scoreboard
pub const COUNT_ID_19: u32 = 19; // QPU Total Clock Cycles For All QPUs Stalled Waiting For Varyings
pub const COUNT_ID_20: u32 = 20; // QPU Total Instruction Cache Hits For All Slices
pub const COUNT_ID_21: u32 = 21; // QPU Total Instruction Cache Misses For All Slices
pub const COUNT_ID_22: u32 = 22; // QPU Total Uniforms Cache Hits For All Slices
pub const COUNT_ID_23: u32 = 23; // QPU Total Uniforms Cache Misses For All Slices
pub const COUNT_ID_24: u32 = 24; // TMU Total Texture Quads Processed
pub const COUNT_ID_25: u32 = 25; // TMU Total Texture Cache Misses (Number Of Fetches From Memory/L2 Cache)
pub const COUNT_ID_26: u32 = 26; // VPM Total Clock Cycles VDW Is Stalled Waiting For VPM Access
pub const COUNT_ID_27: u32 = 27; // VPM Total Clock Cycles VCD Is Stalled Waiting For VPM Access
pub const COUNT_ID_28: u32 = 28; // L2C Total Level 2 Cache Hits
pub const COUNT_ID_29: u32 = 29; // L2C Total Level 2 Cache Misses

// V3D_PCTRC: V3D Performance Counter Clear Register Description
pub const PCTRC_CTCLR0_CTCLR15: u32 = 0x0000FFFF; // Performance Counter Clear Bits (Write To Clear The Performance Counter) WRITE

// V3D_PCTRE: V3D Performance Counter Enables Register Description
pub const PCTRE_CTEN0_CTEN15: u32 = 0x0000FFFF; // Performance Counter Enable Bits (0 = Counter Disabled, 1 = Performance Counter Enabled To Count) READ/WRITE

// V3D_PCTRn: V3D Performance Counter Count n Register Description
pub const PCTRN_PCTR: u32 = 0xFFFFFFFF; // Performance Count (Count Value) READ/WRITE

// V3D_PCTRSn: V3D Performance Counter Mapping n Register Description
pub const PCTRSN_PCTRS: u32 = 0x0000001F; // Performance Counter Device ID READ/WRITE


// V3D Error & Diagnostic Registers

// V3D_BXCF: V3D Binner Debug Register Description
pub const BXCF_FWDDISA: u32 =  0x00000001; // Disable Forwarding In State Cache READ/WRITE
pub const BXCF_CLIPDISA: u32 = 0x00000002; // Disable Clipping READ/WRITE

// V3D_DBGE: V3D PSE Error Signals Register Description
pub const DBGE_VR1_A: u32 =        0x00000002; // Error A Reading VPM READ
pub const DBGE_VR1_B: u32 =        0x00000004; // Error B Reading VPM READ
pub const DBGE_MULIP0: u32 =       0x00010000; // Error Mulip 0 READ
pub const DBGE_MULIP1: u32 =       0x00020000; // Error Mulip 1 READ
pub const DBGE_MULIP2: u32 =       0x00040000; // Error Mulip 2 READ
pub const DBGE_IPD2_VALID: u32 =   0x00080000; // Error IPD2 Valid READ
pub const DBGE_IPD2_FPDUSED: u32 = 0x00100000; // Error IPD2 FPD Used READ

// V3D_FDBGO: V3D FEP Overrun Error Signals Register Description
pub const FDBGO_WCOEFF_FIFO_FULL: u32 = 0x00000002; // Not An Error READ
pub const FDBGO_XYRELZ_FIFO_FULL: u32 = 0x00000004; // Not An Error READ
pub const FDBGO_QBFR_FIFO_ORUN: u32 =   0x00000008; // Error READ
pub const FDBGO_QBSZ_FIFO_ORUN: u32 =   0x00000010; // Error READ
pub const FDBGO_XYFO_FIFO_ORUN: u32 =   0x00000020; // Error READ
pub const FDBGO_FIXZ_ORUN: u32 =        0x00000040; // Error READ
pub const FDBGO_XYRELO_FIFO_ORUN: u32 = 0x00000080; // Error READ
pub const FDBGO_XYRELW_FIFO_ORUN: u32 = 0x00000400; // Error READ
pub const FDBGO_ZCOEFF_FIFO_FULL: u32 = 0x00000800; // Not An Error
pub const FDBGO_REFXY_FIFO_ORUN: u32 =  0x00001000; // Error READ
pub const FDBGO_DEPTHO_FIFO_ORUN: u32 = 0x00002000; // Error READ
pub const FDBGO_DEPTHO_ORUN: u32 =      0x00004000; // Error READ
pub const FDBGO_EZVAL_FIFO_ORUN: u32 =  0x00008000; // Error READ
pub const FDBGO_EZREQ_FIFO_ORUN: u32 =  0x00020000; // Error READ

// V3D_FDBGB: V3D FEP Interface Ready & Stall Signals, FEP Busy Signals Register Description
pub const FDBGB_EDGES_STALL: u32 =        0x00000001; // Stall READ
pub const FDBGB_EDGES_READY: u32 =        0x00000002; // Ready READ
pub const FDBGB_EDGES_ISCTRL: u32 =       0x00000004; // READ
pub const FDBGB_EDGES_CTRLID: u32 =       0x00000038; // READ
pub const FDBGB_ZRWPE_STALL: u32 =        0x00000040; // Stall READ
pub const FDBGB_ZRWPE_READY: u32 =        0x00000080; // Ready READ
pub const FDBGB_EZ_DATA_READY: u32 =      0x00800000; // Ready READ
pub const FDBGB_EZ_XY_READY: u32 =        0x02000000; // Ready READ
pub const FDBGB_RAST_BUSY: u32 =          0x04000000; // Busy READ
pub const FDBGB_QXYF_FIFO_OP_READY: u32 = 0x08000000; // Ready READ
pub const FDBGB_XYFO_FIFO_OP_READY: u32 = 0x10000000; // Ready READ

// V3D_FDBGR: V3D FEP Internal Ready Signals Register Description
pub const FDBGR_QXYF_FIFO_READY: u32 =   0x00000001; // Ready READ
pub const FDBGR_EZREQ_FIFO_READY: u32 =  0x00000002; // Ready READ
pub const FDBGR_EZVAL_FIFO_READY: u32 =  0x00000004; // Ready READ
pub const FDBGR_DEPTHO_FIFO_READY: u32 = 0x00000008; // Ready READ
pub const FDBGR_REFXY_FIFO_READY: u32 =  0x00000010; // Ready READ
pub const FDBGR_ZCOEFF_FIFO_READY: u32 = 0x00000020; // Ready READ
pub const FDBGR_XYRELW_FIFO_READY: u32 = 0x00000040; // Ready READ
pub const FDBGR_WCOEFF_FIFO_READY: u32 = 0x00000080; // Ready READ
pub const FDBGR_XYRELO_FIFO_READY: u32 = 0x00000800; // Ready READ
pub const FDBGR_ZO_FIFO_READY: u32 =     0x00002000; // Ready READ
pub const FDBGR_XYFO_FIFO_READY: u32 =   0x00004000; // Ready READ
pub const FDBGR_RAST_READY: u32 =        0x00010000; // Ready READ
pub const FDBGR_RAST_LAST: u32 =         0x00020000; // Last READ
pub const FDBGR_DEPTHO_READY: u32 =      0x00040000; // Ready READ
pub const FDBGR_EZLIM_READY: u32 =       0x00080000; // Ready READ
pub const FDBGR_XYNRM_READY: u32 =       0x00100000; // Ready READ
pub const FDBGR_XYNRM_LAST: u32 =        0x00200000; // Last READ
pub const FDBGR_XYRELZ_FIFO_READY: u32 = 0x00400000; // Ready READ
pub const FDBGR_XYRELZ_FIFO_LAST: u32 =  0x00800000; // Last READ
pub const FDBGR_INTERPZ_READY: u32 =     0x01000000; // Ready READ
pub const FDBGR_INTERPRW_READY: u32 =    0x08000000; // Ready READ
pub const FDBGR_RECIPW_READY: u32 =      0x10000000; // Ready READ
pub const FDBGR_FIXZ_READY: u32 =        0x40000000; // Ready READ

// V3D_FDBGS: V3D FEP Internal Stall Input Signals Register Description
pub const FDBGS_EZTEST_IP_QSTALL: u32 =     0x00000001; // Stall READ
pub const FDBGS_EZTEST_IP_PRSTALL: u32 =    0x00000002; // Stall READ
pub const FDBGS_EZTEST_IP_VLFSTALL: u32 =   0x00000004; // Stall READ
pub const FDBGS_EZTEST_STALL: u32 =         0x00000008; // Stall READ
pub const FDBGS_EZTEST_VLF_OKNOVALID: u32 = 0x00000010; // Valid READ
pub const FDBGS_EZTEST_QREADY: u32 =        0x00000020; // Ready READ
pub const FDBGS_EZTEST_ANYQF: u32 =         0x00000040; // READ
pub const FDBGS_EZTEST_ANYQVALID: u32 =     0x00000080; // Valid READ
pub const FDBGS_QXYF_FIFO_OP1_VALID: u32 =  0x00000100; // Valid READ
pub const FDBGR_QXYF_FIFO_OP1_LAST: u32 =   0x00000200; // Last READ
pub const FDBGR_QXYF_FIFO_OP1_DUMMY: u32 =  0x00000400; // Dummy READ
pub const FDBGR_QXYF_FIFO_OP_LAST: u32 =    0x00000800; // Last READ
pub const FDBGS_QXYF_FIFO_OP_VALID: u32 =   0x00001000; // Valid READ
pub const FDBGS_EZREQ_FIFO_OP_VALID: u32 =  0x00002000; // Valid READ
pub const FDBGS_XYNRM_IP_STALL: u32 =       0x00004000; // Stall READ
pub const FDBGS_EZLIM_IP_STALL: u32 =       0x00008000; // Stall READ
pub const FDBGS_DEPTHO_FIFO_IP_STALL: u32 = 0x00010000; // Stall READ
pub const FDBGS_INTERPZ_IP_STALL: u32 =     0x00020000; // Stall READ
pub const FDBGS_XYRELZ_FIFO_IP_STALL: u32 = 0x00040000; // Stall READ
pub const FDBGS_INTERPW_IP_STALL: u32 =     0x00400000; // Stall READ
pub const FDBGS_RECIPW_IP_STALL: u32 =      0x02000000; // Stall READ
pub const FDBGS_ZO_FIFO_IP_STALL: u32 =     0x10000000; // Stall READ

// V3D_ERRSTAT: V3D Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C) Register Description
pub const ERRSTAT_VPAEABB: u32 =  0x00000001; // VPM Allocator Error - Allocating Base While Busy READ
pub const ERRSTAT_VPAERGS: u32 =  0x00000002; // VPM Allocator Error - Request Too Big READ
pub const ERRSTAT_VPAEBRGL: u32 = 0x00000004; // VPM Allocator Error - Binner Request Greater Than Limit READ
pub const ERRSTAT_VPAERRGL: u32 = 0x00000008; // VPM Allocator Error - Renderer Request Greater Than Limit READ
pub const ERRSTAT_VPMEWR: u32 =   0x00000010; // VPM Error - Write Range READ
pub const ERRSTAT_VPMERR: u32 =   0x00000020; // VPM Error - Read Range READ
pub const ERRSTAT_VPMERNA: u32 =  0x00000040; // VPM Error - Read Non-Allocated READ
pub const ERRSTAT_VPMEWNA: u32 =  0x00000080; // VPM Error - Write Non-Allocated READ
pub const ERRSTAT_VPMEFNA: u32 =  0x00000100; // VPM Error - Free Non-Allocated READ
pub const ERRSTAT_VPMEAS: u32 =   0x00000200; // VPM Error - Allocated Size Error READ
pub const ERRSTAT_VDWE: u32 =     0x00000400; // VDW Error - Address Overflows READ
pub const ERRSTAT_VCDE: u32 =     0x00000800; // VCD Error - FIFO Pointers Out Of Sync READ
pub const ERRSTAT_VCDI: u32 =     0x00001000; // VCD Idle READ
pub const ERRSTAT_VCMRE: u32 =    0x00002000; // VCM Error (Renderer) READ
pub const ERRSTAT_VCMBE: u32 =    0x00004000; // VCM Error (Binner) READ
pub const ERRSTAT_L2CARE: u32 =   0x00008000; // L2C AXI Receive Fifo Overrun Error READ