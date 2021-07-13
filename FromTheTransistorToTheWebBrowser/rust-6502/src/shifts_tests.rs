#[cfg(test)]
mod shifts_tests {

    use bit_field::BitField;

    use crate::Mem;
    use crate::CPU;

    // type Byte = c_uchar;
    // type Word = c_ushort;

    #[test]
    fn test_asl_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 1;

        mem.Data[0xFF00] = cpu.INS_ASL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 2);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_asl_can_shift_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.A = 0b11000010;

        mem.Data[0xFF00] = cpu.INS_ASL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0b10000100);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_asl_zp_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_ASL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 2);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_asl_zp_can_shift_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ASL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0b11000010;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0b10000100);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_asl_zpx_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_ASL_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 2);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_asl_zpx_can_shift_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_ASL_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 0b11000010;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 0b10000100);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_asl_abs_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_ASL_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x8000], 2);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_asl_abs_can_shift_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ASL_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 0b11000010;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x8000], 0b10000100);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_asl_absx_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_ASL_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);

        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(mem.Data[0x8000 + 0x10], 2);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_asl_absx_can_shift_a_negative_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_ASL_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 0b11000010;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);

        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(mem.Data[0x8000 + 0x10], 0b10000100);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    // ---------- LSR ---------

    #[test]
    fn test_lsr_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 1;

        mem.Data[0xFF00] = cpu.INS_LSR_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_can_shift_a_zero_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 8;

        mem.Data[0xFF00] = cpu.INS_LSR_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 4);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_zp_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_LSR_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_zp_can_shift_a_zero_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_LSR_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 8;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 4);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_zpx_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_LSR_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_zpx_can_shift_a_zero_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_LSR_ZPX;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042 + 0x10] = 8;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x0042 + 0x10], 4);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_abs_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_LSR_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x8000], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_abs_can_shift_a_zero_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_LSR_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 8;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cycles_used, 6);
        assert_eq!(mem.Data[0x8000], 4);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_absx_can_shift_the_value_of_one() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_LSR_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);

        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(mem.Data[0x8000 + 0x10], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_lsr_absx_can_shift_a_zero_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.X = 0x10;

        mem.Data[0xFF00] = cpu.INS_LSR_ABSX;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000 + 0x10] = 8;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);

        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(mem.Data[0x8000 + 0x10], 4);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    // ---------- ROL ---------

    #[test]
    fn test_rol_can_shift_a_bit_out_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 0;

        mem.Data[0xFF00] = cpu.INS_ROL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 1);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_can_shift_a_bit_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 0b10000000;

        mem.Data[0xFF00] = cpu.INS_ROL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_can_shift_zero_with_no_carry() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        cpu.A = 0;

        mem.Data[0xFF00] = cpu.INS_ROL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_can_shift_a_value_that_result_in_neg_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.A = 0b01110011;

        mem.Data[0xFF00] = cpu.INS_ROL_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0b11100111);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    // -------- Zero Page ---------

    #[test]
    fn test_rol_zp_can_shift_a_bit_out_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_ROL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 1);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_zp_can_shift_a_bit_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_ROL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0b10000000;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_zp_can_shift_zero_with_no_carry() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, true); // N

        mem.Data[0xFF00] = cpu.INS_ROL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_rol_zp_can_shift_a_value_that_result_in_neg_value() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ROL_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0b01110011;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0b11100111);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    // ---------- ROR ---------

    #[test]
    fn test_ror_can_shift_the_carry_flag_into_the_operand() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.A = 0;

        mem.Data[0xFF00] = cpu.INS_ROR_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0b10000000);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_ror_can_shift_a_value_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.A = 1;

        mem.Data[0xFF00] = cpu.INS_ROR_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_ror_can_rotate_a_number() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        cpu.A = 0b01101101;

        mem.Data[0xFF00] = cpu.INS_ROR_ACC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.A, 0b10110110);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    // -------- Zero Page ---------

    #[test]
    fn test_ror_zp_can_shift_the_carry_flag_into_the_operand() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ROR_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0b10000000);
        assert_eq!(cpu.PS.get_bit(0), false); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }

    #[test]
    fn test_ror_zp_can_shift_a_value_into_the_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, false); // C
        cpu.PS.set_bit(1, false); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ROR_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 1;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), true); // Z
        assert_eq!(cpu.PS.get_bit(7), false); // N
    }

    #[test]
    fn test_ror_zp_can_rotate_a_number() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(0, true); // C
        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, false); // N

        mem.Data[0xFF00] = cpu.INS_ROR_ZP;
        mem.Data[0xFF01] = 0x42;
        mem.Data[0x0042] = 0b01101101;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cycles_used, 5);
        assert_eq!(mem.Data[0x0042], 0b10110110);
        assert_eq!(cpu.PS.get_bit(0), true); // C
        assert_eq!(cpu.PS.get_bit(1), false); // Z
        assert_eq!(cpu.PS.get_bit(7), true); // N
    }
}
