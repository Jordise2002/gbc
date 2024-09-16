use crate::cartridge;

#[cfg(test)]

#[test]
fn test_adding_op_hl()
{
    let cart = crate::cartridge::Cartidge::new_from_bytes(vec![crate::code::Opcode::ADD_HL_BC as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.set_hl(0x0FFF);
    cpu.set_bc(100);

    cpu.run();

    assert_eq!(cpu.get_hl(), 4195);
    
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}

#[test]
fn test_adding_op_a()
{
    let cart = crate::cartridge::Cartidge::new_from_bytes(vec![crate::code::Opcode::ADD_A_B as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 0x0F;
    cpu.b = 0x01;

    cpu.run();

    assert_eq!(cpu.a, 0x10);
    
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}

#[test]
fn test_sub_op_a_b()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::SUB_A_C as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 200;
    cpu.c = 30;

    cpu.run();

    assert_eq!(cpu.a, 170);

    assert_eq!(cpu.get_substraction_flag(), true);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false)
}

#[test]
fn test_dec_op_de()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::DEC_DE as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.set_de(500);

    cpu.run();

    assert_eq!(cpu.get_de(), 499);
    assert_eq!(cpu.get_substraction_flag(), true);
    assert_eq!(cpu.get_zero_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), false);
}

#[test]
fn test_inc_op_c()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::INC_C as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.c = 0x0F;

    cpu.run();

    assert_eq!(cpu.c, 0x10);
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_zero_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
}

#[test]
fn test_and_op_d()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::AND_A_C as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 0x8;
    cpu.c = 0x0;

    cpu.run();

    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_zero_flag(), true);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}

#[test]
fn test_ld_a_iHL()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::LD_A_iHL as u8, 20]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.set_hl(0x1);

    cpu.run();

    assert_eq!(cpu.a, 20);
}

#[test]
fn test_cp_op_a_b()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::SUB_A_C as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 200;
    cpu.c = 30;

    cpu.run();

    assert_eq!(cpu.get_substraction_flag(), true);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false)
}

#[test]
fn test_sbc_a_ihl()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::SBC_A_N8 as u8, 199]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 200;
    cpu.set_carry_flag(true);
    cpu.set_hl(1);

    cpu.run();

    assert_eq!(cpu.get_zero_flag(),  true);
    assert_eq!(cpu.get_substraction_flag(), true);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}

#[test]
fn test_xor_a_b()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::XOR_A_B as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 7;
    cpu.b = 3;

    cpu.run();

    assert_eq!(cpu.a, 4);

    assert_eq!(cpu.get_zero_flag(), false);
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), false);
    assert_eq!(cpu.get_carry_flag(), false)
}

#[test]
fn test_or_a_b()
{
    let cart = crate::Cartidge::new_from_bytes(vec![crate::code::Opcode::OR_A_B as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 4;
    cpu.b = 3;

    cpu.run();

    assert_eq!(cpu.a, 7);

    assert_eq!(cpu.get_zero_flag(), false);
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), false);
    assert_eq!(cpu.get_carry_flag(), false)
}