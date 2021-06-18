#[cfg(test)]

mod stack_and_operations_tests {

use crate::mos6502::*;
use std::os::raw::*;

type Byte = c_uchar;
type Word = c_ushort;

    #[test]
    fn tsx_can_transfer_the_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.1 = 1;
        cpu.PS.7 = 1;
        cpu.X = 0x00;
        cpu.SP = 0x01;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.PS.1, 0);
        assert_eq!(cpu.PS.7, 0);
    }

    #[test]
    fn tsx_can_transfer_the_0value_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.1 = 1;
        cpu.PS.7 = 1;
        cpu.X = 0x00;
        cpu.SP = 0x00;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.PS.1, 1);
        assert_eq!(cpu.PS.7, 0);
    }

    #[test]
    fn tsx_can_transfer_the_negative_stack_pointer_to_the_x_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.PS.1 = 0;
        cpu.PS.7 = 0;
        cpu.X = 0x00;
        cpu.SP = 0b10000000;
        mem.Data[0xFF00] = cpu.INS_TSX;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.X, 0b10000000);
        assert_eq!(cpu.PS.1, 0);
        assert_eq!(cpu.PS.7, 1);
    }

    #[test]
    fn txs_can_transfer_the_x_register_to_the_sp() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.X = 0xFF;
        cpu.SP = 0;
        mem.Data[0xFF00] = cpu.INS_TXS;

        let mut expected_cycles = 2;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 2);
        assert_eq!(cpu.SP, 0xFF);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

    #[test]
    fn pha_can_transfer_the_a_register_to_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given: 
        cpu.reset_vector(&mut mem, 0xFF00);
        cpu_copy.reset_vector(&mut mem, 0xFF00);
        cpu.A = 0x42;
        mem.Data[0xFF00] = cpu.INS_PHA;

        let mut expected_cycles = 3;

        let actual_cycles = cpu.execute(&mut expected_cycles, &mut mem);

        // then:
        assert_eq!(actual_cycles, 3);
        assert_eq!(mem.Data[cpu.sp_to_address() as usize + 1 as usize], cpu.A);
        // verify_unmodified_flags_from_store(cpu, cpu_copy);
    }

   
    // fn verify_unmodified_flags_from_store(cpu: CPU, cpu_copy: CPU) {
    //     assert_eq!(cpu.C, cpu_copy.C);
    //     assert_eq!(cpu.PS.1, cpu_copy.Z);
    //     assert_eq!(cpu.I, cpu_copy.I);
    //     assert_eq!(cpu.D, cpu_copy.D);
    //     assert_eq!(cpu.B, cpu_copy.B);
    //     assert_eq!(cpu.V, cpu_copy.V);
    //     assert_eq!(cpu.PS.7, cpu_copy.N);
    // }

}


