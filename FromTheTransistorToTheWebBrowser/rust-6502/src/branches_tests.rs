#[cfg(test)]

mod branches_tests {

    use std::os::raw::*;

    type Byte = c_uchar;
    // type SByte = c_schar;
    use bit_field::BitField;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    #[test]
    fn beq_can_branch_forward_when_zero_is_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, true);
        // cpu.Y = 0x42;
        // cpu.X = 0x42;
        mem.Data[0xFF00] = cpu.INS_BEQ;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 3; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.PC, 0xFF03);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn beq_can_branch_forward_when_zero_is_not_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        // cpu.Y = 0x42;
        // cpu.X = 0x42;
        mem.Data[0xFF00] = cpu.INS_BEQ;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 2; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.PC, 0xFF02);
        // assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn beq_can_branch_forward_into_a_new_page_when_zero_is_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFEFD);
        cpu_copy.reset_vector(&mut mem, 0xEFFD);
        cpu.PS.set_bit(1, true);
        // cpu.Y = 0x42;
        // cpu.X = 0x42;
        mem.Data[0xFEFD] = cpu.INS_BEQ;
        mem.Data[0xFEFE] = 0x1;

        let mut expected_cycles = 5; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(cpu.PC, 0xFF00);
        // assert_eq!(cpu.PS, cpu_copy.PS);
    }


    // TODO: check this test
    #[test]
    fn beq_can_branch_backwards_when_zero_is_set_from_assembly() {

        // loop
        // lda #0
        // beq loop

        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        // let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFFCC);
        // cpu_copy.reset_vector(&mut mem, 0xFFCC);

        cpu.PS.set_bit(1, true);
        // cpu_copy.PS.set_bit(1, true);

        mem.Data[0xFFCC] = 0xA9;
        mem.Data[0xFFCC + 1] = 0x00;
        mem.Data[0xFFCC + 2] = 0xF0;
        mem.Data[0xFFCC + 3] = 0xFC;

        let cpu_copy = cpu;

        let mut expected_cycles = 2 + 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2 + 3);
        assert_eq!(cpu.PC, 0xFFCC);
        // assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn bne_can_branch_forward_when_zero_is_not_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.set_bit(1, false);
        cpu_copy.PS.set_bit(1, false);
        mem.Data[0xFF00] = cpu.INS_BNE;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 3; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.PC, 0xFF03);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn bcs_can_branch_forward_when_carry_flag_is_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, true);
        cpu_copy.PS.set_bit(0, true);

        mem.Data[0xFF00] = cpu.INS_BCS;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 3; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.PC, 0xFF03);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn bcc_can_branch_forward_when_carry_flag_is_not_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, false);
        cpu_copy.PS.set_bit(0, false);

        mem.Data[0xFF00] = cpu.INS_BCC;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 3; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.PC, 0xFF03);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }

    #[test]
    fn bmi_can_branch_forward_when_carry_flag_is_set() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(7, true);
        cpu_copy.PS.set_bit(7, true);

        mem.Data[0xFF00] = cpu.INS_BMI;
        mem.Data[0xFF01] = 0x01;

        let mut expected_cycles = 3; // 2 or 3 or 5

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.PC, 0xFF03);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }
}
