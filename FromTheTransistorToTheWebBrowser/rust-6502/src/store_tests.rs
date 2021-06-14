#[cfg(test)]
mod store_tests {

use crate::mos6502::*;
use std::os::raw::*;

type Byte = c_uchar;
type Word = c_ushort;


    fn test_sta_zp(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.A = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x0080] = 0x00;

        let mut expected_cycles = 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    fn test_stx_zp(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.X = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x0080] = 0x00;

        let mut expected_cycles = 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    fn test_sty_zp(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.Y = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x0080] = 0x00;

        let mut expected_cycles = 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }


    fn test_sta_abs(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.A = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0x00;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    fn test_stx_abs(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.X = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0x00;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    fn test_sty_abs(opcode_to_test: Byte) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.Y = 0x2F;
        mem.Data[0xFFFC] = opcode_to_test;
        mem.Data[0xFFFD] = 0x00;
        mem.Data[0xFFFE] = 0x80;
        mem.Data[0x8000] = 0x00;

        let mut expected_cycles = 4;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, expected_cycles);
        assert_eq!(mem.Data[0x0080], 0x2F);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn test_sta_zp_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_sta_zp(cpu.INS_STA_ZP);
    }

    #[test]
    fn test_stx_zp_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_stx_zp(cpu.INS_STX_ZP);
    }

    #[test]
    fn test_sty_zp_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_sty_zp(cpu.INS_STY_ZP);
    }

    #[test]
    fn test_sta_abs_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_sta_abs(cpu.INS_STA_ABS);
    }

    #[test]
    fn test_stx_abs_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_stx_abs(cpu.INS_STX_ABS);
    }

    #[test]
    fn test_sty_abs_can_store_the_a_register_into_memory() {
        let cpu = CPU::new();
        test_sty_abs(cpu.INS_STY_ABS);
    }



    fn verify_unmodified_flags_from_store(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.C, cpu_copy.C);
        assert_eq!(cpu.Z, cpu_copy.Z);
        assert_eq!(cpu.I, cpu_copy.I);
        assert_eq!(cpu.D, cpu_copy.D);
        assert_eq!(cpu.B, cpu_copy.B);
        assert_eq!(cpu.V, cpu_copy.V);
        assert_eq!(cpu.N, cpu_copy.N);
    }
}


