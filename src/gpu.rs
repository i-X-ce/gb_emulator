use crate::memory_bus::MemoryBus;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

fn tilePixelValueToColor(value: TilePixelValue) -> [u8; 3]{
    match value {
        TilePixelValue::Zero => [0, 0, 0],
        TilePixelValue::One => [85, 85, 85],
        TilePixelValue::Two => [175, 175, 175],
        TilePixelValue::Three => [255, 255, 255],
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LcdControlregisters {
    // 0xFF40
    enabled: bool,
    window_tile_map: bool,
    window_enabled: bool,
    tiles: bool,
    bg_tile_map: bool,
    obj_size: bool,
    obj_enabled: bool,
    bg_window_enabled: bool,
}

impl LcdControlregisters {
    pub fn new() -> Self {
        LcdControlregisters {
            enabled: false,
            window_tile_map: false,
            tiles: false,
            bg_tile_map: false,
            window_enabled: false,
            obj_size: false,
            obj_enabled: false,
            bg_window_enabled: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LcdStatusregisters {
    // 0xFF41
    lyc_int_select: bool,
    mode2_int_select: bool,
    mode1_int_select: bool,
    mode0_int_select: bool,
    lyc_eq_ly: bool,
    lyc_ppu_mode: u8,
}

impl LcdStatusregisters {
    pub fn new() -> Self {
        LcdStatusregisters {
            lyc_int_select: false,
            mode2_int_select: false,
            mode1_int_select: false,
            mode0_int_select: false,
            lyc_eq_ly: false,
            lyc_ppu_mode: 0,
        }
    }
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    pub ly: u8,  // 0xFF44
    pub lyc: u8, // 0xFF45
    pub control: LcdControlregisters,
    pub status: LcdStatusregisters,
    tile_set: [Tile; 384],
    scanline_counter: u16,
    frame: [u8; 160 * 3 * 144],
}

impl std::convert::From<LcdControlregisters> for u8 {
    fn from(r: LcdControlregisters) -> u8 {
        (if r.enabled { 1 } else { 0 }) << 7
            | (if r.window_tile_map { 1 } else { 0 }) << 6
            | (if r.window_enabled { 1 } else { 0 }) << 5
            | (if r.tiles { 1 } else { 0 }) << 4
            | (if r.bg_tile_map { 1 } else { 0 }) << 3
            | (if r.obj_size { 1 } else { 0 }) << 2
            | (if r.obj_enabled { 1 } else { 0 }) << 1
            | (if r.bg_window_enabled { 1 } else { 0 }) << 0
    }
}

impl std::convert::From<LcdStatusregisters> for u8 {
    fn from(r: LcdStatusregisters) -> u8 {
        (if r.lyc_int_select { 1 } else { 0 }) << 6
            | (if r.mode2_int_select { 1 } else { 0 }) << 5
            | (if r.mode1_int_select { 1 } else { 0 }) << 4
            | (if r.mode0_int_select { 1 } else { 0 }) << 3
            | (if r.lyc_eq_ly { 1 } else { 0 }) << 2
            | r.lyc_ppu_mode & 0x03
    }
}

impl std::convert::From<u8> for LcdControlregisters {
    fn from(byte: u8) -> Self {
        let enabled = ((byte >> 7) & 0x01) != 0;
        let window_tile_map = ((byte >> 6) & 0x01) != 0;
        let window_enabled = ((byte >> 5) & 0x01) != 0;
        let tiles = ((byte >> 4) & 0x01) != 0;
        let bg_tile_map = ((byte >> 3) & 0x01) != 0;
        let obj_size = ((byte >> 2) & 0x01) != 0;
        let obj_enabled = ((byte >> 1) & 0x01) != 0;
        let bg_window_enabled = ((byte >> 0) & 0x01) != 0;

        LcdControlregisters {
            enabled,
            window_tile_map,
            window_enabled,
            tiles,
            bg_tile_map,
            obj_size,
            obj_enabled,
            bg_window_enabled,
        }
    }
}

impl std::convert::From<u8> for LcdStatusregisters {
    fn from(byte: u8) -> Self {
        let lyc_int_select = ((byte >> 6) & 0x01) != 0;
        let mode2_int_select = ((byte >> 5) & 0x01) != 0;
        let mode1_int_select = ((byte >> 4) & 0x01) != 0;
        let mode0_int_select = ((byte >> 3) & 0x01) != 0;
        let lyc_eq_ly = ((byte >> 2) & 0x01) != 0;
        let lyc_ppu_mode = byte & 0x03;

        LcdStatusregisters {
            lyc_int_select,
            mode2_int_select,
            mode1_int_select,
            mode0_int_select,
            lyc_eq_ly,
            lyc_ppu_mode,
        }
    }
}

impl GPU {
    pub fn new() -> Self {
        GPU {
            vram: [0; VRAM_SIZE],
            ly: 0,
            lyc: 0,
            control: LcdControlregisters::new(),
            status: LcdStatusregisters::new(),
            tile_set: [empty_tile(); 384],
            scanline_counter: 0,
            frame: [0 as u8; 160 * 3 * 144],
        }
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index >= 0x1800 {
            return;
        }

        let normalized_index = index & 0xFFFE;
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];
        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub fn update(&mut self, cycles: u16) {
        // setLCDStatus();
        // if (!isLCDEnabled()) return;

        self.scanline_counter += cycles;

        if (self.scanline_counter >= 456) {
            self.scanline_counter -= 456;
            self.ly += 1;
            let currentline = 0;
            if currentline == 144 {
                // VBlank
            } else if currentline > 153 {
                self.ly = 0;
            } else if currentline < 144 {
                self.draw_scan_line(currentline);
            }
        }
    }

    fn draw_scan_line(&mut self, line: u8) {
        //1ライン描画
        //self.frame
    }

    fn draw_all(&mut self){
        for addr in 0x9800..=0x9BFF{
            let addr = addr as usize - VRAM_BEGIN;
            let index = self.vram[addr] as usize;
            let tile = self.tile_set[index];
            let i = addr - 0x1800;
            let x = i % 32 * 8;
            let y = i / 32 * 8;
            
            for ty in 0..8 {
                for tx in 0..8 {
                    let value = tile[ty][tx];
                    let color = tilePixelValueToColor(value);
                    let o = y * 32 + x + ty * 8 + tx;
                    if x > 160 || y > 144 {
                        continue;
                    }
                    self.frame[o] = color[0];
                    self.frame[o + 1] = color[1];
                    self.frame[o + 2] = color[2];
                }
            }
        }
    }
}
