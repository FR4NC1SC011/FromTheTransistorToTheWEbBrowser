#[cfg(test)]
mod increments_and_decrements_tests {

    use bit_field::BitField;
    use std::os::raw::*;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    #[test]
    fn incx_can_increment_a_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.X = 0x0;
        mem.Data[0xFF00] = cpu.INS_INX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn incx_can_increment_255() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        cpu.PS.set_bit(7, true);
        cpu.X = 0xFF;
        mem.Data[0xFF00] = cpu.INS_INX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn incx_can_increment_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.X = 0b10000010;
        mem.Data[0xFF00] = cpu.INS_INX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0b10000011);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn incy_can_increment_a_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.Y = 0x0;
        mem.Data[0xFF00] = cpu.INS_INY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn incy_can_increment_255() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        cpu.PS.set_bit(7, true);
        cpu.Y = 0xFF;
        mem.Data[0xFF00] = cpu.INS_INY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn incy_can_increment_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.Y = 0b10000010;
        mem.Data[0xFF00] = cpu.INS_INY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0b10000011);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decx_can_decrement_a_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.X = 0x00;
        mem.Data[0xFF00] = cpu.INS_DEX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0xFF);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decx_can_decrement_255() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.X = 0xFF;
        mem.Data[0xFF00] = cpu.INS_DEX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0xFE);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decx_can_decrement_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.X = 0b10000011;
        mem.Data[0xFF00] = cpu.INS_DEX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0b10000010);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decy_can_decrement_a_zero_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.Y = 0x0;
        mem.Data[0xFF00] = cpu.INS_DEY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0xFF);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decy_can_decrement_255() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.Y = 0xFF;
        mem.Data[0xFF00] = cpu.INS_DEY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0xFE);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn decy_can_decrement_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, false);
        cpu.Y = 0b10000011;
        mem.Data[0xFF00] = cpu.INS_DEY;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.Y, 0b10000010);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), true);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn dec_can_decrement_a_value_in_the_zero_page() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        mem.Data[0xFF00] = cpu.INS_DEC_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0x57;

        let mut expected_cycles = 5;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(mem.Data[0x0042], 0x56);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn dec_can_decrement_a_value_in_the_zero_page_x() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x10;
        mem.Data[0xFF00] = cpu.INS_DEC_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 0x56);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn dec_can_decrement_a_value_absolute() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        mem.Data[0xFF00] = cpu.INS_DEC_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x8000], 0x56);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn dec_can_decrement_a_value_absolute_x() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x10;
        mem.Data[0xFF00] = cpu.INS_DEC_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x8000 + 0x10], 0x56);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn inc_can_increment_a_value_in_the_zero_page() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        mem.Data[0xFF00] = cpu.INS_INC_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0x57;

        let mut expected_cycles = 5;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(mem.Data[0x0042], 0x58);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn inc_can_increment_a_value_in_the_zero_page_x() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x10;
        mem.Data[0xFF00] = cpu.INS_INC_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 0x58);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn inc_can_increment_a_value_absolute() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        mem.Data[0xFF00] = cpu.INS_INC_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x8000], 0x58);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags(cpu, cpu_copy);
    }

    #[test]
    fn inc_can_increment_a_value_absolute_x() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        cpu.X = 0x10;
        mem.Data[0xFF00] = cpu.INS_INC_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 0x57;

        let mut expected_cycles = 6;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_eq!(mem.Data[0x8000 + 0x10], 0x58);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
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
