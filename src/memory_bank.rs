pub struct MemoryBank {
    bank_size: usize,
    bank_ammount: usize,
    current_bank: usize,
    mem: Vec<u8>,
}

impl MemoryBank {
    pub fn new(bank_size: usize, bank_ammount: usize, current_bank: usize) -> Self {
        MemoryBank {
            bank_size,
            bank_ammount,
            current_bank,
            mem: vec![0; bank_ammount * bank_size],
        }
    }

    pub fn read(&self, address: u16) -> Option<u8> {
        self.mem
            .get(address as usize + self.current_bank * self.bank_size)
            .copied()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let actual_address = self.current_bank * self.bank_size;
        assert!(
            actual_address < self.mem.len(),
            "Writing to memory out of bounds"
        );
        self.mem[actual_address] = value;
    }

    pub fn read_bankless(&self, address: u16) -> Option<u8> {
        self.mem.get(address as usize).copied()
    }

    pub fn write_bankless(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}
