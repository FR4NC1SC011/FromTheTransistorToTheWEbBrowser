#[cfg(test)]
mod load_tests {

    use bit_field::BitField;
    use std::os::raw::*;

    use crate::Mem;
    use crate::CPU;

    type Byte = c_uchar;
    type Word = c_ushort;

    #[test]
    fn test_ldainmediate_can_load_value_into_a_register() {
        // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_IM;
        mem.Data[0xFFFD] = 0x84;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.A, 0x84);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldxinmediate_can_load_value_into_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDX_IM;
        mem.Data[0xFFFD] = 0x84;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.X, 0x84);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldyinmediate_can_load_value_into_y_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDY_IM;
        mem.Data[0xFFFD] = 0x84;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.Y, 0x84);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldainmediate_can_affect_the_zero_flag() {
        // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.A = 0x29;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_IM;
        mem.Data[0xFFFD] = 0x0;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazp_can_load_value_into_a_register() {
        // LDAZeroPageCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);
        assert_eq!(cycles_used, 3);

        // then:
        assert_eq!(cpu.A, 0x37);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldxzp_can_load_value_into_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDX_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);
        assert_eq!(cycles_used, 3);

        // then:
        assert_eq!(cpu.X, 0x37);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldyzp_can_load_value_into_y_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDY_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);
        assert_eq!(cycles_used, 3);

        // then:
        assert_eq!(cpu.Y, 0x37);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazpx_can_load_value_into_a_register() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.X = 5;

        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);
        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldxzpy_can_load_value_into_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.Y = 5;

        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDX_ZPY;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);
        // then:
        assert_eq!(cpu.X, 0x37);
        assert_eq!(cycles_used, 4);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldyzpx_can_load_value_into_y_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.X = 5;

        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDY_ZPX;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);
        // then:
        assert_eq!(cpu.Y, 0x37);
        assert_eq!(cycles_used, 4);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazpx_can_load_value_into_a_register_when_it_wraps() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu.X = 0xFF;

        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x007F] = 0x37;

        // when:
        let mut cpu_copy = CPU::new();
        cpu_copy.reset(&mut mem);

        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn the_cpu_does_nothing_when_we_execute_cero_cycles() {
        // given:
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        // when:
        let cycles_used = cpu.execute(&mut 0, &mut mem);

        // then:
        assert_eq!(cycles_used, 0);
    }

    //      #[test]
    //     fn cpu_can_execute_more_cycles_than_requested_if_required_by_the_instruction() {
    //         // LDAInmediateCanLoadAValueIntoTheAReg
    //         let mut mem = Mem::new();
    //         let mut cpu = CPU::new();
    //         let mut cpu_copy = CPU::new();
    //
    //         // given:
    //         cpu.reset(&mut mem);
    //         cpu_copy.reset(&mut mem);
    //         mem.Data[0xFFFC] = cpu.INS_LDA_IM;
    //         mem.Data[0xFFFD] = 0x84;
    //
    //         // when:
    //         let cycles_used = cpu.execute(&mut 1, &mut mem);
    //
    //         // then:
    //         assert_eq!(cycles_used, 2);
    //     }
    //

    //      #[test]
    //     fn executing_a_bad_inst_does_not_put_us_into_an_infinite_loop() {
    //         let mut mem = Mem::new();
    //         let mut cpu = CPU::new();
    //         let mut cpu_copy = CPU::new();
    //
    //         // given:
    //         cpu.reset(&mut mem);
    //         cpu_copy.reset(&mut mem);
    //         mem.Data[0xFFFC] = 0x0;
    //         mem.Data[0xFFFD] = 0x0;
    //
    //         // when:
    //         let cycles_used = cpu.execute(&mut 3, &mut mem);
    //
    //         // then:
    //         assert_eq!(cycles_used, 3);
    //     }

    #[test]
    fn test_ldaabs_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABS;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4480] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
    }

    #[test]
    fn test_ldxabs_can_load_value_into_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDX_ABS;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4480] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.X, 0x37);
        assert_eq!(cycles_used, 4);
    }

    #[test]
    fn test_ldyabs_can_load_value_into_y_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDY_ABS;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4480] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.Y, 0x37);
        assert_eq!(cycles_used, 4);
    }

    #[test]
    fn test_ldaabsx_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.X = 1;
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldxabsy_can_load_value_into_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 1;
        mem.Data[0xFFFC] = cpu.INS_LDX_ABSY;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.X, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldxabsy_can_load_value_into_x_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 0xFF;
        mem.Data[0xFFFC] = cpu.INS_LDX_ABSY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44; // 0x4402
        mem.Data[0x4501] = 0x37; // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.X, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldyabsx_can_load_value_into_y_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.X = 1;
        mem.Data[0xFFFC] = cpu.INS_LDY_ABSX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.Y, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldyabsx_can_load_value_into_y_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.X = 0xFF;
        cpu.PS.set_bit(1, true);
        cpu.PS.set_bit(7, true);
        mem.Data[0xFFFC] = cpu.INS_LDY_ABSX;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44; // 0x4402
        mem.Data[0x4501] = 0x37; // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.Y, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaabsx_can_load_value_into_a_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.X = 0xFF;
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSX;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44; // 0x4402
        mem.Data[0x4501] = 0x37; // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaabsy_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 1;
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSY;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44; // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaabsy_can_load_value_into_a_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 0xFF;
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44; // 0x4402
        mem.Data[0x4501] = 0x37; // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaindx_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.X = 0x04;
        mem.Data[0xFFFC] = cpu.INS_LDA_INDX;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0006] = 0x00; // 0x02 + 0x04
        mem.Data[0x0007] = 0x80;
        mem.Data[0x8000] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaindy_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 0x04;
        mem.Data[0xFFFC] = cpu.INS_LDA_INDY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x00;
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8004] = 0x37; //0x8000 + 0x4

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldaindy_can_load_value_into_a_register_when_it_crosses_a_page() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.Y = 0xFF;
        mem.Data[0xFFFC] = cpu.INS_LDA_INDY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x02;
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8101] = 0x37; //0x8002 + 0xFF

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    // TODO: fix this function
    //     fn test_load_register_inmediate(opcode: Byte, CPU:: *mut Byte ) {
    //         let mut mem = Mem::new();
    //         let mut cpu = CPU::new();
    //         let mut cpu_copy = CPU::new();
    //
    //         // given:
    //         cpu.reset(&mut mem);
    //         cpu_copy.reset(&mut mem);
    //         mem.Data[0xFFFC] = cpu.INS_LDA_IM;
    //         mem.Data[0xFFFD] = 0x84;
    //
    //         // when:
    //         let cycles_used = cpu.execute(&mut 2, &mut mem);
    //         assert_eq!(cycles_used, 2);
    //
    //         // then:
    //         assert_eq!(cpu.*register, 0x84);
    //         verify_unmodified_flags_from_lda(cpu, cpu_copy);
    //     }

    fn verify_unmodified_flags_from_lda(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0));
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2));
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3));
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4));
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6));
    }
}
