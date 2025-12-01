pub(super) struct PpuRegisters {
    // Loopy registers (internal scroll/address state)
    current_vram_address: u16,
    temp_vram_address: u16,
    fine_x: u8,
    write_latch: bool,

    // Memory-mapped registers ($2000-$2003)
    ppuctrl: u8,     // $2000 - Control flags
    ppumask: u8,     // $2001 - Rendering flags
    ppustatus: u8,   // $2002 - Status flags (mostly read-only)
    pub(super) oam_address: u8, // $2003 - OAM read/write address
    
    // Internal state
    read_buffer: u8, // Buffered read for $2007
}

impl PpuRegisters {
    pub(super) fn new() -> Self {
        Self {
            current_vram_address: 0,
            temp_vram_address: 0,
            fine_x: 0,
            write_latch: false,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0xA0,  // Power-up: vblank set, sprite0 clear
            oam_address: 0,
            read_buffer: 0,
        }
    }
}