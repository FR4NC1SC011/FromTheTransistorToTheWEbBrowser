#[cfg(test)]

mod branches_tests {

    use std::os::raw::*;

    type Byte = c_uchar;
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
}
