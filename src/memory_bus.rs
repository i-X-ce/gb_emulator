use crate::{cartridge::{self, Cartridge}, gpu::{self, LcdControlregisters, LcdStatusregisters, GPU, VRAM_BEGIN, VRAM_END}};

pub struct  MemoryBus{
    memory: [u8; 0x10000],
    pub gpu: GPU,
    catridge: Cartridge,
}

impl MemoryBus{
    pub fn new(cartridge: Cartridge) -> Self{
        MemoryBus {
            memory: [0; 0x10000],
            gpu: GPU::new(),
            catridge: cartridge,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.catridge.read_byte(address as u16),
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            },
            0xFF40 => u8::from(self.gpu.control),
            0xFF41 => u8::from(self.gpu.status),
            0xFF44 => self.gpu.ly,
            0xFF45 => self.gpu.lyc,
            _ => panic!("TODO: support other areas of memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8){
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.catridge.write_byte(address as u16, value),
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            },
            0xFF40 => self.gpu.control = LcdControlregisters::from(value),
            0xFF41 => self.gpu.status = LcdStatusregisters::from(value),
            0xFF44 => { /* read only */ },
            0xFF45 => self.gpu.lyc = value,
            _ => panic!("TODO: support other areas of memory")
        }
    }
}
