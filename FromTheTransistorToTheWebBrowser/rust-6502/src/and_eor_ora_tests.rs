#[cfg(test)]
mod and_eor_ora_tests {

use crate::mos6502::*;
use std::os::raw::*;
use bit_field::BitField;

type Byte = c_uchar;
type Word = c_ushort;

    enum ELogicalOp {
        And,
        Eor,
        Or,
    }

    fn do_logical_op(x: Byte, y: Byte, logical_op: ELogicalOp) -> Byte {
        match logical_op {
            ELogicalOp::And => {
                return x & y
            },
            ELogicalOp::Eor => {
                return x ^ y
            },
            ELogicalOp::Or => {
                return x | y
            },
        }
    }

    fn verify_unmodified_flags_from_logical_op_ins(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0));
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2));
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3));
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4));
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6));
    }


    fn test_logical_op_inmediate(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_IM;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_IM;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_IM;
            },
        }

        mem.Data[0xFFFD] = 0x84;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x84, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), expected_negative);
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_zeropage(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ZP;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ZP;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ZP;
            },
        }

        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 3);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), expected_negative);
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

     fn test_logical_op_zeropage_x(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        cpu.X = 5;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ZPX;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ZPX;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ZPX;
            },
        }

        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(7), expected_negative);
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

     fn test_logical_op_absolute(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ABS;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ABS;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ABS;
            },
        }

        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;
        mem.Data[0x4480] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

     fn test_logical_op_absolute_x(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        cpu.X = 1;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ABSX;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ABSX;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ABSX;
            },
        }

        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_absolute_y(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        cpu.Y = 1;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ABSY;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ABSY;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ABSY;
            },
        }

        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

     fn test_load_register_absolute_y_when_crossing_page(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        cpu.Y = 0xFF;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ABSY;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ABSY;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ABSY;
            },
        }

        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44;
        mem.Data[0x4501] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_load_register_absolute_x_when_crossing_page(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        cpu.X = 0xFF;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ABSX;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ABSX;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ABSX;
            },
        }

        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44;
        mem.Data[0x4501] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_indirect_x(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        cpu.X = 0x04;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_INDX;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_INDX;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_INDX;
            },
        }

        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0006] = 0x00;
        mem.Data[0x0007] = 0x80;
        mem.Data[0x8000] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_indirect_y(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        cpu.Y = 0x04;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_INDY;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_INDY;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_INDY;
            },
        }

        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x00;
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8004] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_indirect_y_when_it_crosses_a_page(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(1, true); // Z
        cpu.PS.set_bit(7, true); // N
        cpu.A = 0xCC;
        cpu.Y = 0xFF;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_INDY;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_INDY;
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_INDY;
            },
        }

        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x02;
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8101] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }

    fn test_logical_op_zeropage_x_when_it_wraps(logical_op: ELogicalOp) {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        cpu.X = 0xFF;
        match logical_op {
            ELogicalOp::And => {
            mem.Data[0xFFFC] = cpu.INS_AND_ZPX;
            },
            ELogicalOp::Eor => {
            mem.Data[0xFFFC] = cpu.INS_EOR_ZPX
            },
            ELogicalOp::Or => {
            mem.Data[0xFFFC] = cpu.INS_ORA_ZPX;
            },
        }

        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x007F] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        let expected_result: Byte = do_logical_op(0xCC, 0x37, logical_op);
        let expected_negative: bool = (expected_result & 0b10000000) > 0;
        assert_eq!(cpu.A, expected_result);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);              // Z
        assert_eq!(cpu.PS.get_bit(7), expected_negative);  // N
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);
    }



    #[test]
    fn test_logicalop_and_on_a_register_inmediate() {
        test_logical_op_inmediate(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_on_a_register_inmediate() {
        test_logical_op_inmediate(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_on_a_register_inmediate() {
        test_logical_op_inmediate(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_eor_inmediate_can_affect_zero_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        mem.Data[0xFFFC] = cpu.INS_EOR_IM;
        mem.Data[0xFFFD] = cpu.A;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(7), false);
        verify_unmodified_flags_from_logical_op_ins(cpu, cpu_copy);

    }

    #[test]
    fn test_logicalop_and_on_a_register_zeropage() {
        test_logical_op_zeropage(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_on_a_register_zeropage() {
        test_logical_op_zeropage(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_on_a_register_zeropage() {
        test_logical_op_zeropage(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_on_a_register_zeropagex() {
        test_logical_op_zeropage_x(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_on_a_register_zeropagex() {
        test_logical_op_zeropage_x(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_on_a_register_zeropagex() {
        test_logical_op_zeropage_x(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_can_load_a_value_into_the_a_register_when_it_wraps_zpx() {
        test_logical_op_zeropage_x_when_it_wraps(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_can_load_a_value_into_the_a_register_when_it_wraps_zpx() {
        test_logical_op_zeropage_x_when_it_wraps(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_can_load_a_value_into_the_a_register_when_it_wraps_zpx() {
        test_logical_op_zeropage_x_when_it_wraps(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_on_a_register_absolute() {
        test_logical_op_absolute(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_on_a_register_absolute() {
        test_logical_op_absolute(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_on_a_register_absolute() {
        test_logical_op_absolute(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_on_a_register_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_on_a_register_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_on_a_register_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_when_it_crosses_a_page_boundary_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_when_it_crosses_a_page_boundary_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_when_it_crosses_a_page_boundary_absolutex() {
        test_logical_op_absolute_x(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_when_it_crosses_a_page_boundary_absolutey() {
        test_logical_op_absolute_y(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_when_it_crosses_a_page_boundary_absolutey() {
        test_logical_op_absolute_y(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_when_it_crosses_a_page_boundary_absolutey() {
        test_logical_op_absolute_y(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_indirectx() {
        test_logical_op_indirect_x(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_indirectx() {
        test_logical_op_indirect_x(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_indirectx() {
        test_logical_op_indirect_x(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_indirecty() {
        test_logical_op_indirect_y(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_indirecty() {
        test_logical_op_indirect_y(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_indirecty() {
        test_logical_op_indirect_y(ELogicalOp::Eor);
    }

    #[test]
    fn test_logicalop_and_when_it_crosses_a_page_indirecty() {
        test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::And);
    }

    #[test]
    fn test_logicalop_or_when_it_crosses_a_page_indirecty() {
        test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::Or);
    }

    #[test]
    fn test_logicalop_eor_when_it_crosses_a_page_indirecty() {
        test_logical_op_indirect_y_when_it_crosses_a_page(ELogicalOp::Eor);
    }


    #[test]
    fn test_bit_zero_page() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(6, false);
        cpu.PS.set_bit(7, false);
        cpu.A = 0xCC;
        mem.Data[0xFFFC] = cpu.INS_BIT_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0xCC;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);

        // then:
        assert_eq!(cpu.A, 0xCC);
        assert_eq!(cycles_used, 3);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(6), true);
        assert_eq!(cpu.PS.get_bit(7), true);
    }

    #[test]
    fn test_bit_zero_page_zero_result_zero() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(6, true);
        cpu.PS.set_bit(7, true);
        cpu.A = 0xCC;
        mem.Data[0xFFFC] = cpu.INS_BIT_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x33;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);

        // then:
        assert_eq!(cpu.A, 0xCC);
        assert_eq!(cycles_used, 3);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(6), false);
        assert_eq!(cpu.PS.get_bit(7), false);
    }

    #[test]
    fn test_bit_zero_page_zero_result_zero_bits_6_and_7() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.PS.set_bit(6, false);
        cpu.PS.set_bit(7, false);
        cpu.A = 0x33;
        mem.Data[0xFFFC] = cpu.INS_BIT_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0xCC;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x33);
        assert_eq!(cycles_used, 3);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(6), true);
        assert_eq!(cpu.PS.get_bit(7), true);
    }


 #[test]
    fn test_bit_absolute() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        mem.Data[0xFFFC] = cpu.INS_BIT_ABS;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0xCC;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0xCC);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), false);
        assert_eq!(cpu.PS.get_bit(6), true);
        assert_eq!(cpu.PS.get_bit(7), true);
    }

 #[test]
    fn test_bit_absolute_result_zero() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0xCC;
        mem.Data[0xFFFC] = cpu.INS_BIT_ABS;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0x33;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0xCC);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(6), true);
        assert_eq!(cpu.PS.get_bit(7), true);
    }

    #[test]
    fn test_bit_absolute_result_zero_bits_6_and_7() {
         // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        cpu.A = 0x33;
        mem.Data[0xFFFC] = cpu.INS_BIT_ABS;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0xCC;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x33);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(6), false);
        assert_eq!(cpu.PS.get_bit(7), false);
    }
}

