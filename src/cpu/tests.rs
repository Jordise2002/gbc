#[cfg(test)]

#[test]
fn test_adding_op()
{
    let cart = crate::cartridge::Cartidge::new_from_bytes(vec![crate::code::Opcode::ADD_HL_BC as u8]);
    let mut cpu = super::Cpu::new_test(cart);

    cpu.set_hl(100);
    cpu.set_bc(100);

    cpu.run();

    assert_eq!(cpu.get_hl(), 200);
    
    assert_eq!(cpu.get_substraction_flag(), false);
    assert_eq!(cpu.get_half_carry_flag(), true);
    assert_eq!(cpu.get_carry_flag(), false);
}