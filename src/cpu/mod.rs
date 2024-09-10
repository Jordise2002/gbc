mod tests;

use byteorder::{ByteOrder, LittleEndian};

use crate::{
    bus::Bus,
    cartridge::{self, Cartidge},
    code,
};

pub struct Cpu {
    memory: Bus,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    cycles: u64
}

impl Cpu {
    pub fn new(cart: Cartidge) -> Self {
        Cpu {
            memory: Bus::new(cart),
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cycles: 0
        }
    }

    /**
     * Creates a cpu with a default memory bus independent of the cartridge header, used for testing
     */
    #[cfg(test)]
    pub fn new_test(cart:Cartidge) -> Self
    {
        Cpu {
            memory: Bus::new_test(cart),
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cycles: 0
        }
    }

    fn get_af(&self) -> u16 {
        LittleEndian::read_u16(&[self.f, self.a])
    }

    fn get_bc(&self) -> u16 {
        LittleEndian::read_u16(&[self.c, self.b])
    }

    fn get_de(&self) -> u16 {
        LittleEndian::read_u16(&[self.e, self.d])
    }

    fn get_hl(&self) -> u16 {
        LittleEndian::read_u16(&[self.l, self.h])
    }

    fn set_af(&mut self, value: u16) {
        let aux = value.to_be_bytes();
        self.a = aux[0];
        self.f = aux[1];
    }

    fn set_bc(&mut self, value: u16) {
        let aux = value.to_be_bytes();
        self.b = aux[0];
        self.c = aux[1];
    }

    fn set_de(&mut self, value: u16) {
        let aux = value.to_be_bytes();
        self.d = aux[0];
        self.e = aux[1];
    }

    fn set_hl(&mut self, value: u16) {
        let aux = value.to_be_bytes();
        self.h = aux[0];
        self.l = aux[1];
    }

    fn get_zero_flag(&self) -> bool {
        (self.f & 0b10000000) != 0
    }

    fn get_substraction_flag(&self) -> bool {
        (self.f & 0b01000000) != 0
    }

    fn get_half_carry_flag(&self) -> bool {
        (self.f & 0b00100000) != 0
    }

    fn get_carry_flag(&self) -> bool {
        (self.f & 0b00010000) != 0
    }

    fn set_zero_flag(&mut self, state:bool) {
        self.f = (self.f & 0b10000000)| (state as u8) << 7;
    }

    fn set_substraction_flag(&mut self, state: bool) {
        self.f = (self.f & 0b01000000) | (state as u8) << 6;
    }

    fn set_half_carry_flag(&mut self, state: bool) {
        self.f = (self.f & 0b00100000) | (state as u8) << 5;
    }

    fn set_carry_flag(&mut self, state:bool ) {
        self.f = (self.f & 0b00010000) | (state as u8) << 4;
    }

    fn fetch(&mut self) -> Option<u8> {
        let result = self
            .memory
            .read(self.pc);
        self.pc = self.pc + 1;
        result
    }

    fn fetch_16(&mut self) -> Option<u16> {
        let first_byte = self.fetch()?;
        let second_byte = self.fetch()?;
        Some(LittleEndian::read_u16(&[second_byte, first_byte]))
    }

    //TODO: Gestionar correctamente el valor de fetch aquí
    fn fetch_operand_value(&mut self, op_type: code::Operand) -> i32
    {
        match op_type
        {
            code::Operand::A => self.a as i32,
            code::Operand::B => self.b as i32,
            code::Operand::C => self.c as i32,
            code::Operand::D => self.d as i32,
            code::Operand::E => self.e as i32,
            code::Operand::H => self.h as i32,
            code::Operand::L => self.l as i32,
            code::Operand::AF => self.get_af() as i32,
            code::Operand::SP => self.sp as i32,
            code::Operand::PC => self.pc as i32,
            code::Operand::BC => self.get_bc() as i32,
            code::Operand::DE => self.get_de() as i32,
            code::Operand::HL => self.get_hl() as i32,

            code::Operand::N16 =>self.fetch_16().unwrap() as i32,
            code::Operand::N8 => self.fetch().unwrap() as i32,

            code::Operand::E8 => (self.fetch().unwrap() as i8) as i32,
            code::Operand::SP_PLUS_E8 => (self.sp as i32) + (self.fetch().unwrap() as i8) as i32,
 
            code::Operand::A16 => {
                let address = self.fetch_16();
             self.memory.read(address.unwrap()).expect("Error accessing memory") as i32
            },
            code::Operand::A8 => {
                let address = self.fetch().unwrap() as u16 + 0xFF00;
                self.memory.read(address).expect("Error accessing memory") as i32
            },

            code::Operand::iC => self.memory.read(0xFF00 + self.c as u16).expect("Error accessing to memory") as i32,
            code::Operand::iBC => self.memory.read(self.get_bc()).expect("Error accessing memory") as i32,
            code::Operand::iDE => self.memory.read(self.get_bc()).expect("Error accessing memory") as i32,
            code::Operand::iHL => self.memory.read(self.get_hl()).expect("Error accessing memory") as i32,
            code::Operand::iHLPLUS => {
                let result = self.fetch_operand_value(code::Operand::iHL);
                self.set_hl(self.get_hl() + 1);
                result
            },
            code::Operand::iHLMINUS => {
                let result = self.fetch_operand_value(code::Operand::iHL);
                self.set_hl(self.get_hl() - 1);
                result
            }
            
        }
    }

    fn handle_add_op(&mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size(), "Wrong operand size");

        self.set_substraction_flag(false);

        match op1_type
        {
            code::Operand::A => {
                let unceiled_value = self.a as i32 + self.fetch_operand_value(op2_type);
                self.a = unceiled_value as u8;
                self.set_zero_flag(self.a == 0);
                self.set_carry_flag(unceiled_value > 0xFF);
                self.set_half_carry_flag(self.a > 0xF);
            },
            code::Operand::HL => {
                let unceiled_value = self.get_hl() as i32 + self.fetch_operand_value(op2_type);
                self.set_hl(unceiled_value as u16);
                self.set_carry_flag(unceiled_value > 0xFFFF);
                self.set_half_carry_flag(self.get_hl() > 0xF);
            },
            code::Operand::SP => 
            {
                let unceiled_value = (self.sp as i32 + self.fetch_operand_value(op2_type));
                self.sp = unceiled_value as u16;
                self.set_zero_flag(false);
                self.set_carry_flag(unceiled_value > 0xFFFF);
                self.set_half_carry_flag(self.get_hl() > 0xF);
            }
            _=> panic!("ADD NOT SUPPORTED FOR {:?} {:?}", op1_type, op2_type),
        }
    }
 
    pub fn run(&mut self) {
        while let Some(c) = self.fetch() {
            let (instruction, cycles) = code::get_instruction_specs_from_code(c).expect(format!("Non Valid Opcode: {}", c).as_str());

            match instruction
            {
                code::Instruction::ADD(op1_type, op2_type) => 
                {
                    self.handle_add_op(op1_type, op2_type);
                }
                _ => 
                {
                    panic!("Not supported instruction type Opcode: {}", c);
                }
            }

            self.cycles += cycles; 
        }
    }
}
