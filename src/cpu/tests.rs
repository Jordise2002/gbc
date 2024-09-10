#[cfg(test)]

#[test]
fn test_adding_op_hl()
{
    let cart = crate::cartridge::Cartidge::new_from_bytes(vec![crate::code::Opcode::ADD_HL_BC as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.set_hl(0xFFFE);
    cpu.set_bc(100);

    cpu.run();

    assert_eq!(cpu.get_hl(), 98);
    
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), true);
}

#[test]
fn test_adding_op_a()
{
    let cart = crate::cartridge::Cartidge::new_from_bytes(vec![crate::code::Opcode::ADD_A_B as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.a = 0xF0;
    cpu.b = 0x01;

    cpu.run();

    assert_eq!(cpu.a, 0xF1);
    
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}