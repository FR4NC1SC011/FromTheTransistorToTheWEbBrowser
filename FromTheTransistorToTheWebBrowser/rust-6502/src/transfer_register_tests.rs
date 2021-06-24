#[cfg(test)]
mod transfer_register_tests {

    use crate::mos6502::*;
    use bit_field::BitField;
    use std::os::raw::*;

    type Byte = c_uchar;
    type Word = c_ushort;

    #[test]
    fn tax_of_can_transfer_non_negative_non_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.A = 0x42;
        cpu.X = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAX;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.X, 0x42);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tax_of_can_transfer_non_negative_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, false); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.A = 0x0;
        cpu.X = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAX;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0x0);
        assert_eq!(cpu.X, 0x0);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tax_of_can_transfer_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, false); // Negative Flag
        cpu.A = 0b10000000;
        cpu.X = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAX;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0b10000000);
        assert_eq!(cpu.X, 0b10000000);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tay_of_can_transfer_non_negative_non_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.A = 0x42;
        cpu.Y = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAY;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.Y, 0x42);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tay_of_can_transfer_non_negative_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, false); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.A = 0x0;
        cpu.Y = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAY;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0x0);
        assert_eq!(cpu.Y, 0x0);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tay_of_can_transfer_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, false); // Negative Flag
        cpu.A = 0b10000000;
        cpu.Y = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TAY;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.A, 0b10000000);
        assert_eq!(cpu.Y, 0b10000000);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn txa_of_can_transfer_non_negative_non_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.X = 0x42;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TXA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x42);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn txa_of_can_transfer_non_negative_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, false); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.X = 0x0;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TXA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x0);
        assert_eq!(cpu.A, 0x0);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn txa_of_can_transfer_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, false); // Negative Flag
        cpu.X = 0b10000000;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TXA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0b10000000);
        assert_eq!(cpu.A, 0b10000000);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tya_of_can_transfer_non_negative_non_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.Y = 0x42;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TYA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0x42);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tya_of_can_transfer_non_negative_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, false); // Zero Flag
        cpu.PS.set_bit(7, true); // Negative Flag
        cpu.Y = 0x0;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TYA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0x0);
        assert_eq!(cpu.A, 0x0);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn tya_of_can_transfer_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Zero Flag
        cpu.PS.set_bit(7, false); // Negative Flag
        cpu.Y = 0b10000000;
        cpu.A = 0x32;

        mem.Data[0xFFFC] = cpu.INS_TYA;

        let mut expected_cycles = 2;

        // when
        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        //then
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0b10000000);
        assert_eq!(cpu.A, 0b10000000);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    fn verify_unmodified_flags(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0));
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2));
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3));
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4));
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6));
    }
}
