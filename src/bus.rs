use crate::{
    cartridge::{self, Cartidge},
    memory_bank::MemoryBank,
};
pub struct Bus {
    cartridge_rom: MemoryBank,
    cgb_switchable_ram: MemoryBank,
    cartridge_switchable_ram: MemoryBank,
    non_switchable_region: MemoryBank,
    second_cgb_switchable_ram: MemoryBank,
    rest_ram: MemoryBank,
}

impl Bus {
    pub fn new(cart: Cartidge) -> Self {
        let is_cgb = cart.get_cgb();
        let rom_banks = cart.get_rom_banks();
        let cartridge_ram = cart.get_ram_banks();

        let cartridge_rom = MemoryBank::new(16 * 1024, rom_banks, 1);

        let cartridge_switchable_ram = MemoryBank::new(8 * 1024, cartridge_ram, 0);

        let cgb_switchable_ram = if is_cgb {
            MemoryBank::new(8 * 1024, 2, 0)
        } else {
            MemoryBank::new(8 * 1024, 1, 0)
        };

        let second_cgb_switchable_ram = if is_cgb {
            MemoryBank::new(4 * 1024, 7, 0)
        } else {
            MemoryBank::new(4 * 1024, 1, 0)
        };

        let non_switchable_region = MemoryBank::new(4 * 1024, 1, 0);

        let rest_ram = MemoryBank::new(8 * 1024, 1, 0);

        Bus {
            cartridge_rom,
            cgb_switchable_ram,
            cartridge_switchable_ram,
            non_switchable_region,
            second_cgb_switchable_ram,
            rest_ram,
        }
    }

    #[cfg(test)]
    pub fn new_test(cart: Cartidge) -> Self
    {
        let is_cgb = true;
        let rom_banks = 2;
        let cartridge_ram = 1;

        let cartridge_rom = MemoryBank::new(16 * 1024, rom_banks, 1);

        let cartridge_switchable_ram = MemoryBank::new(8 * 1024, cartridge_ram, 0);

        let cgb_switchable_ram = if is_cgb {
            MemoryBank::new(8 * 1024, 2, 0)
        } else {
            MemoryBank::new(8 * 1024, 1, 0)
        };

        let second_cgb_switchable_ram = if is_cgb {
            MemoryBank::new(4 * 1024, 7, 0)
        } else {
            MemoryBank::new(4 * 1024, 1, 0)
        };

        let non_switchable_region = MemoryBank::new(4 * 1024, 1, 0);

        let rest_ram = MemoryBank::new(8 * 1024, 1, 0);

        Bus {
            cartridge_rom,
            cgb_switchable_ram,
            cartridge_switchable_ram,
            non_switchable_region,
            second_cgb_switchable_ram,
            rest_ram,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                //Primer banco del cartucho
                self.cartridge_rom.write_bankless(address, value);
            }
            0x4000..=0x7FFF => {
                //Banco switcheable del cartucho
                self.cartridge_rom.write(address - 0x4000, value);
            }
            0x8000..=0x9FFF => {
                //Banco switcheable
                self.cgb_switchable_ram.write(address - 0x8000, value);
            }
            0xA000..=0xBFFF => {
                //Banco de ram switcheable del cartucho
                self.cartridge_switchable_ram.write(address - 0xA000, value);
            }
            0xC000..=0xCFFF => {
                self.non_switchable_region.write(address - 0xC000, value);
            }
            0xD000..=0xDFFF => {
                //Banco Switcheable
                self.second_cgb_switchable_ram
                    .write(address - 0xDFFF, value)
            }
            0xE000.. => {
                //Resto de la ram
                self.rest_ram.write(address - 0xE000, value)
            }
        }
    }

    pub fn read(&self, address: u16) -> Option<u8> {
        match address {
            0x0000..=0x3FFF => {
                //Primer banco del cartucho
                self.cartridge_rom.read_bankless(address)
            }
            0x4000..=0x7FFF => {
                //Banco switcheable del cartucho
                self.cartridge_rom.read(address - 0x4000)
            }
            0x8000..=0x9FFF => {
                //Banco switcheable
                self.cartridge_switchable_ram.read(address - 0x8000)
            }
            0xA000..=0xBFFF => {
                //Banco de ram switcheable del cartucho
                self.cartridge_switchable_ram.read(address - 0xA000)
            }
            0xC000..=0xCFFF => self.non_switchable_region.read(address - 0xC000),
            0xD000..=0xDFFF => {
                //Banco Switcheable
                self.second_cgb_switchable_ram.read(address - 0xD000)
            }
            0xE000.. => {
                //Resto de la ram
                self.rest_ram.read(address - 0xE000)
            }
        }
    }
}
