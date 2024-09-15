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

pub struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
    scanline_counter: u16,
    frame: [u8; 160 * 3 * 144],
}

impl GPU {
    pub fn new() -> Self{
        GPU{
            vram: [0; VRAM_SIZE],
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

        if index >= 0x1800 { return }

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
        self.scanline_counter += cycles;

        if (self.scanline_counter >= 456){
            self.scanline_counter -= 456;
            //0xFF44++
            let currentline = 0;
            if currentline == 144 {
                // VBlank
            } else if currentline > 153 {
                //0xFF44 = 0
            } else if currentline < 144 {
                self.draw_scan_line(currentline);
            }
        }
    }

    fn draw_scan_line(&mut self, line: u16) {
        //1ライン描画
        //self.frame
    }
}