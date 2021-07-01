#[cfg(test)]

mod add_with_carry_tests {

    use std::os::raw::*;
    use bit_field::BitField;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    #[test]
    fn adc_can_add_zero_to_zero_and_get_zero() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);

        cpu.PS.set_bit(0, false);  // Carry Flag
        cpu.A = 0;
        cpu.PS.set_bit(1, true);  // Zero Flag
        cpu.PS.set_bit(7, true);  // Zero Flag


        mem.Data[0xFF00] = cpu.INS_ADC_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 0x00;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 4);
        assert_eq!(cpu.A, 0x0);
        assert_eq!(cpu.SP, cpu_copy.SP);
        assert_eq!(cpu.PS.get_bit(0), false);
        assert_eq!(cpu.PS.get_bit(1), true);
        assert_eq!(cpu.PS.get_bit(6), false);
        assert_eq!(cpu.PS.get_bit(6), false);
        // verify_unmodified_flags(cpu, cpu_copy);
    }

    // fn verify_unmodified_flags(cpu: CPU, cpu_copy: CPU) {
    //     assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0));
    //     assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2));
    //     assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3));
    //     assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4));
    //     assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6));
    // }
}
