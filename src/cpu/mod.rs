mod tests;

    use core::{panic};

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
        self.f = (self.f & !0b10000000)| (state as u8) << 7;
    }

    fn set_substraction_flag(&mut self, state: bool) {
        self.f = (self.f & !0b01000000) | (state as u8) << 6;
    }

    fn set_half_carry_flag(&mut self, state: bool) {
        self.f = (self.f & !0b00100000) | (state as u8) << 5;
    }

    fn set_carry_flag(&mut self, state:bool ) {
        self.f = (self.f & !0b00010000) | (state as u8) << 4;
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
                let other_value = self.fetch_operand_value(op2_type);
                self.set_half_carry_flag(((self.a & 0xF) + (other_value as u8 & 0xF)) & 0x10 == 0x10);
                let unceiled_value = self.a as i32 + other_value;
                self.a = unceiled_value as u8;
                self.set_zero_flag(self.a == 0);
                self.set_carry_flag(unceiled_value > 0xFF);
            },
            code::Operand::HL => {
                let hl_value = self.get_hl();
                let other_value = self.fetch_operand_value(op2_type);
                self.set_half_carry_flag(((hl_value & 0xFFF) + (other_value as u16 & 0xFFF)) & 0x1000 == 0x1000);
                let unceiled_value:i32 = hl_value as i32 + other_value as i32;
                self.set_hl(unceiled_value as u16);
                self.set_carry_flag(unceiled_value > 0xFFFF);
            },
            code::Operand::SP => 
            {
                let other_value = self.fetch_operand_value(op2_type);
                let unceiled_value = (self.sp as i32 + other_value);
                self.set_half_carry_flag(((self.sp & 0xFFF) + (other_value as u16 & 0xFFF)) & 0x1000 == 0x1000);
                self.sp = unceiled_value as u16;
                self.set_zero_flag(false);
                self.set_carry_flag(unceiled_value > 0xFFFF);
            }
            _=> panic!("ADD NOT SUPPORTED FOR {:?} {:?}", op1_type, op2_type),
        }
    }

    fn handle_dec_op(&mut self, op1_type: code::Operand)
    {
        self.set_substraction_flag(true);
        let value = self.fetch_operand_value(op1_type.clone());
        match op1_type.get_operand_size()
        {
            1 => {
                self.set_half_carry_flag(value as u8 & 0x0F < 1);
            }   
            2 => {
                self.set_half_carry_flag(value as u16 & 0x0FFF < 1);
            }
            _ => {
                panic!("Operand size {} not suported for DEC", op1_type.get_operand_size())
            }
        }
        match op1_type
        {
            code::Operand::A => 
            {
                self.a = self.a.wrapping_sub(1);
                self.set_zero_flag(self.a == 0);
            }
            code::Operand::B => {
                self.b = self.b.wrapping_sub(1);
                self.set_zero_flag(self.b == 0);
            }
            code::Operand::L => {
                self.l = self.b.wrapping_sub(1);
                self.set_zero_flag(self.l == 0);
            }
            code::Operand::E => {
                self.e = self.e.wrapping_sub(1);
                self.set_zero_flag(self.e == 0);
            }
            code::Operand::C => {
                self.c = self.c.wrapping_sub(1);
                self.set_zero_flag(self.c == 0);
            }
            code::Operand::D => {
                self.d = self.d.wrapping_sub(1);
                self.set_zero_flag(self.d == 0);
            }
            code::Operand::H => {
                self.h = self.h.wrapping_sub(1);
                self.set_zero_flag(self.h == 0);
            }
            code::Operand::iHL => {
                self.memory.write(self.get_hl(), self.memory.read(self.get_hl()).expect("Wrong memory access!").wrapping_sub(1));
                self.set_zero_flag(self.memory.read(self.get_hl()).expect("Wrong memory access!") == 0);
            }
            code::Operand::BC => {
                self.set_bc(self.get_bc().wrapping_sub(1));
                self.set_zero_flag(self.get_bc() == 0);
            }
            code::Operand::DE => {
                self.set_de(self.get_de().wrapping_sub(1));
                self.set_zero_flag(self.get_de() == 0);
            }
            code::Operand::HL => {
                self.set_hl(self.get_hl().wrapping_sub(1));
                self.set_zero_flag(self.get_hl() == 0);
            }
            code::Operand::SP => {
                self.sp = self.sp.wrapping_sub(1);
                self.set_zero_flag(self.sp == 0);
            }
            _ => {
                panic!("DEC not supported for Operand {:?}", op1_type);
            }
        }
    }

    fn handle_inc_op(& mut self, op1_type:code::Operand)
    {
        match op1_type
        {
            code::Operand::BC => {
                self.set_bc(self.get_bc().wrapping_add(1));
            }
            code::Operand::DE => {
                self.set_de(self.get_de().wrapping_add(1));
            }
            code::Operand::HL => {
                self.set_hl(self.get_hl().wrapping_add(1));
            }
            code::Operand::SP => {
                self.sp = self.sp.wrapping_add(1);
            }
            code::Operand::B => {
                self.set_half_carry_flag(((self.b & 0x0F) + 1) & 0x10 == 0x10);
                self.b = self.b.wrapping_add(1);
                self.set_zero_flag(self.b == 0);
            }
            code::Operand::D => {
                self.set_half_carry_flag(((self.d & 0x0F) + 1) & 0x10 == 0x10);
                self.d = self.d.wrapping_add(1);
                self.set_zero_flag(self.d == 0);
            }
            code::Operand::H => {
                self.set_half_carry_flag(((self.h & 0x0F) + 1) & 0x10 == 0x10);
                self.h = self.h.wrapping_add(1);
                self.set_zero_flag(self.h == 0);
            }
            code::Operand::iHL => {
                self.set_half_carry_flag((self.memory.read(self.get_hl()).expect("Wrong access memory!") & 0x0F + 1) & 0x10 == 0x10);
                self.memory.write(self.get_hl(), self.memory.read(self.get_hl()).expect("wrong access memory!").wrapping_add(1));
                self.set_zero_flag(self.memory.read(self.get_hl()).expect("Wrong memory access!") == 0);
            }
            code::Operand::C => {
                self.set_half_carry_flag(((self.c & 0x0F) + 1) & 0x10 == 0x10);
                self.c = self.c.wrapping_add(1);
                self.set_zero_flag(self.c == 0);
            }
            code::Operand::E => {
                self.set_half_carry_flag(((self.e & 0x0F) + 1) & 0x10 == 0x10);
                self.e = self.e.wrapping_add(1);
                self.set_zero_flag(self.e == 0);
            }
            code::Operand::L => {
                self.set_half_carry_flag(((self.l & 0x0F) + 1) & 0x10 == 0x10);
                self.l = self.l.wrapping_add(1);
                self.set_zero_flag(self.l == 0);
            }
            code::Operand::A => {
                self.set_half_carry_flag(((self.a & 0x0F) + 1) & 0x10 == 0x10);
                self.a = self.a.wrapping_add(1);
                self.set_zero_flag(self.a == 0);
            }
            _ => {
                panic!("INC not supported for operand {:?}", op1_type);
            }
        }

        if op1_type.get_operand_size() == 1
        {
            self.set_substraction_flag(false);
        }
    }
 
    fn handle_sub_op(&mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        if let code::Operand::A = op1_type.clone()
        {
            self.a = self.handle_cp_op(op1_type, op2_type);
        }
        else {
            panic!("CP NOT SUPPORTED FOR {:?} {:?}", op1_type, op2_type);
        }
    }
    fn handle_cp_op(&mut self, op1_type: code::Operand, op2_type: code::Operand) -> u8
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());
        
        if let code::Operand::A = op1_type
        {
            let other_operand = self.fetch_operand_value(op2_type);
            self.set_half_carry_flag(other_operand as u8 & 0xF > self.a & 0xF); //Comparamos los nibbles menores de los operadores
            let unceiled_value = self.a as i32 - other_operand;
            let a = unceiled_value as u8;

            self.set_substraction_flag(true);
            self.set_zero_flag(self.a == 0);
            a
        }
        else {
            panic!("SUB NOT SUPPOTED FOR {:?} {:?}", op1_type, op2_type)
        }
    }

    fn handle_and_op(&mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        self.set_half_carry_flag(true);
        self.set_substraction_flag(false);
        self.set_carry_flag(false);

        if let code::Operand::A = op1_type
        {
            let other_value = self.fetch_operand_value(op2_type);
            self.a = self.a & other_value as u8;
            self.set_zero_flag(self.a == 0);
        }
        else {
            panic!("AND NOT SUPPORTED FOR {:?} and {:?}", op1_type, op2_type);
        }
    }

    fn handle_ld_op(& mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        let other_value = self.fetch_operand_value(op2_type.clone());
        match op1_type
        {
            code::Operand::BC => {
                self.set_bc(other_value as u16);
            }
            code::Operand::DE => {
                self.set_de(other_value as u16);
            }
            code::Operand::HL => {
                self.set_hl(other_value as u16);
            }
            code::Operand::SP => {
                self.sp = other_value as u16;
            }
            code::Operand::A => {
                self.a = other_value as u8;
            }
            code::Operand::D => {
                self.d = other_value as u8;
            }
            code::Operand::H => {
                self.h = other_value as u8;
            }
            code::Operand::C => {
                self.c = other_value as u8;
            }
            code::Operand::E => {
                self.e = other_value as u8;
            }
            code::Operand::B => {
                self.b = other_value as u8;
            }
            code::Operand::L => {
                self.l = other_value as u8;
            }
            code::Operand::iDE => {
                self.memory.write(self.get_de(), other_value as u8);
            }
            code::Operand::iHL => {
                self.memory.write(self.get_hl(), other_value as u8);
            }
            code::Operand::iBC => {
                self.memory.write(self.get_bc(), other_value as u8);
            }
            code::Operand::iHLMINUS => {
                self.memory.write(self.get_hl(), other_value as u8);
                self.set_hl(self.get_hl() - 1);
            }
            code::Operand::iHLPLUS => {
                self.memory.write(self.get_hl(), other_value as u8);
                self.set_hl(self.get_hl() + 1);
            }
            _ => {
                panic!("LD NOT SUPPORTED FOR {:?} and {:?}", op1_type, op2_type);
            }
        }
    }

    fn handle_adc_op(& mut self, op1_type: code::Operand, op2_type: code::Operand) 
    {

        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        if let code::Operand::A = op1_type
        {
            let mut other_operand = self.fetch_operand_value(op2_type);
            if self.get_carry_flag()
            {
                other_operand += 1;
            }

            let unceiled_value = self.a as i32 + other_operand;
            self.a = unceiled_value as u8;
            self.set_zero_flag(self.a == 0);
            self.set_substraction_flag(false);
            self.set_carry_flag(unceiled_value > 0xFF);
            self.set_half_carry_flag(((self.a & 0xF) + (other_operand as u8 & 0xF)) & 0x10 == 0x10);
        }
        else {
            panic!("ADC NOT SUPPORTED FOR {:?}", op1_type)
        }
    }

    fn handle_sbc_op(&mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        if let code::Operand::A = op1_type
        {
            let mut other_operand = self.fetch_operand_value(op2_type);

            if self.get_carry_flag()
            {
                other_operand += 1;
            }

            let unceiled_value = self.a as i32 - other_operand;

            self.a = unceiled_value as u8;

            self.set_zero_flag(self.a == 0);
            self.set_substraction_flag(true);
            self.set_carry_flag(unceiled_value > 0xFF);
            self.set_half_carry_flag(other_operand as u8 & 0xF > self.a & 0xF);
        }
        else {
            panic!("SBC NOT SUPPORTED FOR {:?} {:?}", op1_type, op2_type);
        }
    }


    fn handle_xor_op(& mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        if let code::Operand::A = op1_type
        {
            let other_operand = self.fetch_operand_value(op2_type);

            self.a = self.a ^ other_operand as u8;

            self.set_zero_flag(self.a == 0);
            self.set_substraction_flag(false);
            self.set_half_carry_flag(false);
            self.set_carry_flag(false);
        }
        else {
            panic!("XOR NOT SUPPORTED FOR {:?} {:?}", op1_type, op2_type);
        }
    }

    fn handle_or_op(& mut self, op1_type: code::Operand, op2_type: code::Operand)
    {
        assert_eq!(op1_type.get_operand_size(), op2_type.get_operand_size());

        if let code::Operand::A = op1_type
        {
            let other_operand = self.fetch_operand_value(op2_type);
            self.a = self.a | other_operand as u8;

            self.set_zero_flag(self.a == 0);
            self.set_substraction_flag(false);
            self.set_carry_flag(false);
            self.set_half_carry_flag(false);
        }
        else {
            panic!("")
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
                code::Instruction::SUB(op1_type, op2_type) =>
                {
                    self.handle_sub_op(op1_type, op2_type);
                }
                code::Instruction::DEC(op1_type) => {
                    self.handle_dec_op(op1_type);
                }
                code::Instruction::INC(op1_type) => {
                    self.handle_inc_op(op1_type);
                }
                code::Instruction::AND(op1_type, op2_type) => {
                    self.handle_and_op(op1_type, op2_type);
                }
                code::Instruction::NOP => {

                }
                code::Instruction::LD(op1_type, op2_type) => 
                {
                    self.handle_ld_op(op1_type, op2_type);
                }
                code::Instruction::CP(op1_type, op2_type) => 
                {
                    self.handle_cp_op(op1_type, op2_type);
                }
                code::Instruction::ADC(op1_type, op2_type) => 
                {
                    self.handle_adc_op(op1_type, op2_type);
                }
                code::Instruction::SBC(op1_type, op2_type) => 
                {
                    self.handle_sbc_op(op1_type, op2_type);
                }
                code::Instruction::XOR(op1_type, op2_type) => 
                {
                    self.handle_xor_op(op1_type, op2_type);
                }
                code::Instruction::OR(op1_type, op2_type) => 
                {
                    self.handle_or_op(op1_type, op2_type);
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
