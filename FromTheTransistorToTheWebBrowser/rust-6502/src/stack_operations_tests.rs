#[cfg(test)]

mod stack_and_operations_tests {

    use bit_field::BitField;
    use std::os::raw::*;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Flags;
    use crate::Mem;
    use crate::CPU;

    #[test]
    fn tsx_can_transfer_the_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x00;
        cpu.SP = 0x01;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
    }

    #[test]
    fn tsx_can_transfer_the_0value_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x00;
        cpu.SP = 0x00;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
    }

    #[test]
    fn tsx_can_transfer_the_negative_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        cpu.PS.set_bit(7, false);
        cpu.X = 0x00;
        cpu.SP = 0b10000000;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0b10000000);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
    }

    #[test]
    fn txs_can_transfer_the_x_register_to_the_sp() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.X = 0xFF;
        cpu.SP = 0;
        mem.Data[0xFF00] = cpu.INS_TXS;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.SP, 0xFF);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn pha_can_transfer_the_a_register_to_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.A = 0x42;
        mem.Data[0xFF00] = cpu.INS_PHA;

        let mut expected_cycles = 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(mem.Data[cpu.sp_to_address() as usize + 1 as usize], cpu.A);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn pla_can_pull_a_value_from_the_stack_to_the_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.A = 0x00;
        cpu.SP = 0xFE;
        mem.Data[0x01FF] = 0x42;
        mem.Data[0xFF00] = cpu.INS_PLA;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.SP, 0xFF);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn pla_can_pull_zerovalue_from_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        cpu.PS.set_bit(7, true);
        cpu.A = 0x42;
        cpu.SP = 0xFE;
        mem.Data[0x01FF] = 0x00;
        mem.Data[0xFF00] = cpu.INS_PLA;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        assert_eq!(cpu.SP, 0xFF);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn pla_can_pull_negative_value_from_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(7, false);
        cpu.PS.set_bit(1, true);
        cpu.A = 0x42;
        cpu.SP = 0xFE;
        mem.Data[0x01FF] = 0b10000001;
        mem.Data[0xFF00] = cpu.INS_PLA;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, 0b10000001);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        assert_eq!(cpu.SP, 0xFF);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn php_can_push_ps_onto_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS = 0xCC;
        mem.Data[0xFF00] = cpu.INS_PHP;

        let mut expected_cycles = 3;
        cpu_copy = cpu;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(
            mem.Data[cpu.sp_to_address() as usize + 1 as usize],
            0xCC | Flags::UnusedFlagBit as u8 | Flags::BreakFlagBit as u8
        );
        assert_eq!(cpu.PS, cpu_copy.PS);
        assert_eq!(cpu.SP, 0xFE);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn php_always_set_bits_4_and_5_on_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS = 0x0;
        mem.Data[0xFF00] = cpu.INS_PHP;

        let mut expected_cycles = 3;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        let add_ps_on_stack: Word = cpu.sp_to_address() + 1;
        assert_eq!(actual_cycles, 3);
        // https://wiki.nesdev.com/w/index.php/Status_flags
        //Two interrupts (/IRQ and /NMI) and two instructions (PHP and BRK) push
        // the flags to the stack. In the byte pushed, bit 5 is always set to 1,
        //and bit 4 is 1 if from an instruction (PHP or BRK) or 0 if from an
        // interrupt line being pulled low (/IRQ or /NMI). This is the only time
        // and place where the B flag actually exists: not in the status register
        // itself, but in bit 4 of the copy that is written to the stack.
        let flags_on_stack: Byte = Flags::UnusedFlagBit as u8 | Flags::BreakFlagBit as u8;
        assert_eq!(mem.Data[add_ps_on_stack as usize], flags_on_stack);
    }

    #[test]
    fn plp_can_pull_a_value_from_the_stack_into_the_ps() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS = 0;
        cpu.SP = 0xFE;
        mem.Data[0x01FF] = 0x42 | Flags::BreakFlagBit as u8 | Flags::UnusedFlagBit as u8;
        mem.Data[0xFF00] = cpu.INS_PLP;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.PS, 0x42);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn plp_clear_bits_4_and_5_when_pulling_from_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS = 0;
        cpu.SP = 0xFE;
        mem.Data[0x01FF] = Flags::BreakFlagBit as u8 | Flags::UnusedFlagBit as u8;
        mem.Data[0xFF00] = cpu.INS_PLP;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.PS, 0);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    // fn verify_unmodified_flags_from_store(cpu: CPU, cpu_copy: CPU) {
    //     assert_eq!(cpu.C, cpu_copy.C);
    //     assert_eq!(cpu.PS.1, cpu_copy.Z);
    //     assert_eq!(cpu.I, cpu_copy.I);
    //     assert_eq!(cpu.D, cpu_copy.D);
    //     assert_eq!(cpu.B, cpu_copy.B);
    //     assert_eq!(cpu.V, cpu_copy.V);
    //     assert_eq!(cpu.PS.7, cpu_copy.N);
    // }
}
