use crate::gpu::{self, GPU, VRAM_BEGIN, VRAM_END};

pub struct  MemoryBus{
    memory: [u8; 0x10000],
    gpu: GPU,
}

impl MemoryBus{
    pub fn new() -> Self{
        MemoryBus {
            memory: [0; 0x10000],
            gpu: GPU::new(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            },
            0xFF44 => self.gpu.ly,
            _ => panic!("TODO: support other areas of memory")
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8){
        let address = address as usize;
        match address {
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            },
            0xFF44 => self.gpu.ly = value,
            _ => panic!("TODO: support other areas of memory")
        }
    }
}
