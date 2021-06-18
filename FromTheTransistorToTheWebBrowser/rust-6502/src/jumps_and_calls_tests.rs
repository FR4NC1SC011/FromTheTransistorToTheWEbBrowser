#[cfg(test)]

mod jumps_and_calls_tests {

use crate::mos6502::*;
use std::os::raw::*;

type Byte = c_uchar;
type Word = c_ushort;

    #[test]
    fn can_jump_to_subroutine_and_jump_back_again() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_JSR;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = cpu.INS_RTS;
        mem.Data[0xFF03] = cpu.INS_LDA_IM;
        mem.Data[0xFF04] = 0x42;

        let mut expected_cycles = 6 + 6 + 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6 + 6 + 2);
        assert_eq!(cpu.A, 0x42);
        assert_eq!(cpu.SP, cpu_copy.SP);
    }

   #[test]
    fn jsr_does_not_affect_the_processor_status() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_JSR;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;

        let mut expected_cycles = 6; 

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6);
        assert_ne!(cpu.SP, cpu_copy.SP);
        assert_eq!(cpu.PC, 0x8000);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

   #[test]
    fn rts_does_not_affect_the_processor_status() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_JSR;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = cpu.INS_RTS;

        let mut expected_cycles = 6 + 6; 

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 6 + 6);
        assert_eq!(cpu.PC, 0xFF03);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }


   #[test]
    fn jmp_abs_can_jump_to_a_new_location() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_JMP_ABS;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;

        let mut expected_cycles = 3; 

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(cpu.SP, cpu_copy.SP);
        assert_eq!(cpu.PC, 0x8000);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

   #[test]
    fn jmp_ind_can_jump_to_a_new_location() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_JMP_IND;
        mem.Data[0xFF01] = 0x00;
        mem.Data[0xFF02] = 0x80;
        mem.Data[0x8000] = 0x00;
        mem.Data[0x8001] = 0x90;

        let mut expected_cycles = 5; 

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 5);
        assert_eq!(cpu.SP, cpu_copy.SP);
        assert_eq!(cpu.PC, 0x9000);
        verify_unmodified_flags_from_store(cpu, cpu_copy);
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
