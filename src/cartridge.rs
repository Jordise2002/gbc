use std::fs;

pub struct Cartidge {
    pub content: Vec<u8>,
}

impl Cartidge {
    pub fn new(path: String) -> Self {
        let content =
            fs::read(path.clone()).expect(format!("Couldn't find rom at {}", path).as_str());
        Cartidge { content }
    }

    #[cfg(test)]
    pub fn new_from_bytes(content: Vec<u8>) -> Self
    {
        Cartidge {content}
    }

    pub fn get_rom_banks(&self) -> usize {
        match self.content[0x148] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => {
                panic!("Not supported Rom Size|");
            }
        }
    }

    pub fn get_ram_banks(&self) -> usize {
        match self.content[0x149] {
            0x0 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => {
                panic!("Not supported Ram Size!");
            }
        }
    }

    pub fn get_cgb(&self) -> bool {
        match self.content[0x143] {
            0x80 => true,
            0xC0 => true,
            _ => false,
        }
    }
}
