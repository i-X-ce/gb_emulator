use std::{fs::{self, File}, io::Read};

use mapper::MBC1;

use crate::mapper;

enum RomSize {
    Bank2,
    Bank4,
}

enum RamSize{
    No,
    Unused,
    Bank1,
}

pub struct Cartridge{
    raw: Vec<u8>,
    mapper: MBC1,
    rom_size: RomSize,
    ram_size: RamSize,
}

impl Cartridge {
    pub fn new (filename: &str) -> Self {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut raw = vec![0; metadata.len() as usize];
        f.read(&mut raw).expect("buffer overflow");

        println!("{:02X?}", &raw[0x0104..=0x133]);

        Cartridge {
            raw, 
            mapper: match raw[0x0147] {
                // 0x00 => CartridgeType::RomOnly,
                0x01 => MBC1::new(),
                _ => panic!("unsupported crtridge type."),
            },
            rom_size: match raw[0x0148] {
                0x00 => RomSize::Bank2,
                0x01 => RomSize::Bank4,
                _ => panic!("unsupported rom size."),
            },
            ram_size: match raw[0x0149] {
                0x00 => RamSize::No,
                0x01 => RamSize::Unused,
                0x02 => RamSize::Bank1,
                _ => panic!("unsupported ram size."),
            }
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mapper.read_byte(&self.raw, addr)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.mapper.write_byte(&self.raw, addr, value);
    }
}