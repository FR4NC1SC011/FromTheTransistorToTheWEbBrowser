#[cfg(test)]
mod status_flags_tests {

    use bit_field::BitField;
    use std::os::raw::*;

    use crate::Mem;
    use crate::CPU;

    type Byte = c_uchar;
    type Word = c_ushort;

    #[test]
    fn clc_can_clear_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(0, true);

        mem.Data[0xFFFC] = cpu.INS_CLC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(0), false);

        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimnal Mode
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn cld_can_clear_decimal_mode() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(3, true);

        mem.Data[0xFFFC] = cpu.INS_CLD;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(3), false);

        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Carry Flag
        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn cli_can_clear_interrupt_disable() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(2, true);

        mem.Data[0xFFFC] = cpu.INS_CLI;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(2), false);

        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Carry Flag
        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimal Mode
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn clv_can_clear_overflow_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(6, true);

        mem.Data[0xFFFC] = cpu.INS_CLV;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(6), false);

        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Carry Flag
        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimal Mode
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn sec_can_set_carry_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(0, false);

        mem.Data[0xFFFC] = cpu.INS_SEC;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(0), true);

        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimal Mode
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn sed_can_set_decimal_mode_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(3, false);

        mem.Data[0xFFFC] = cpu.INS_SED;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(3), true);

        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }

    #[test]
    fn sei_can_set_interrupt_disable_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
    
        cpu.PS.set_bit(2, false);

        mem.Data[0xFFFC] = cpu.INS_SEI;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.PS.get_bit(2), true);

        assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Carry Flag
        assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
        assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimal Mode
        assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
        assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
        assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
        assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
    }



// 
//         assert_eq!(cpu.PS.get_bit(0), cpu_copy.PS.get_bit(0)); // Carry Flag
//         assert_eq!(cpu.PS.get_bit(1), cpu_copy.PS.get_bit(1)); // Zero Flag
//         assert_eq!(cpu.PS.get_bit(2), cpu_copy.PS.get_bit(2)); // Interrupt Disable
//         assert_eq!(cpu.PS.get_bit(3), cpu_copy.PS.get_bit(3)); // Decimnal Mode
//         assert_eq!(cpu.PS.get_bit(4), cpu_copy.PS.get_bit(4)); // Break Command
//         assert_eq!(cpu.PS.get_bit(5), cpu_copy.PS.get_bit(5)); // Unused
//         assert_eq!(cpu.PS.get_bit(6), cpu_copy.PS.get_bit(6)); // Overflow Flag
//         assert_eq!(cpu.PS.get_bit(7), cpu_copy.PS.get_bit(7)); // Negative Flag
}
