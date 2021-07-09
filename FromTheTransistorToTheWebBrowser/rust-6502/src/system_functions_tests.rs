#[cfg(test)]
mod system_functions_tests {

    use bit_field::BitField;
    // use std::os::raw::*;

    use crate::Mem;
    use crate::CPU;
    use crate::Flags;


    #[test]
    fn test_nop_will_do_nothing_but_consume_a_cycle() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_NOP;
        let cpu_copy = cpu;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);


        // then:
        assert_eq!(cycles_used, 2);
        assert_eq!(cpu.PS, cpu_copy.PS);
        assert_eq!(cpu.PC, 0xFF01);
        assert_eq!(cpu.A, cpu_copy.A);
        assert_eq!(cpu.X, cpu_copy.X);
        assert_eq!(cpu.Y, cpu_copy.Y);
        assert_eq!(cpu.SP, cpu_copy.SP);
    }

    #[test]
    fn test_brk_will_load_the_program_counter_from_the_interrupt_vector() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_BRK;
        mem.Data[0xFFFE] = 0x00;
        mem.Data[0xFFFF] = 0x80;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);


        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(cpu.PC, 0x8000);
    }

    #[test]
    fn test_brk_will_load_the_program_counter_from_the_interrupt_vector_2() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        mem.Data[0xFF00] = cpu.INS_BRK;
        mem.Data[0xFFFE] = 0x00;
        mem.Data[0xFFFF] = 0x90;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);


        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(cpu.PC, 0x9000);
    }

    #[test]
    fn test_brk_will_set_the_break_flag() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        
        cpu.PS.set_bit(4, false);    // B

        mem.Data[0xFF00] = cpu.INS_BRK;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);


        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(cpu.PS.get_bit(4), true);
    }

    #[test]
    fn test_brk_will_push_3_bytes_onto_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);
        
        cpu.PS.set_bit(4, false);    // B

        mem.Data[0xFF00] = cpu.INS_BRK;
        let cpu_copy = cpu;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);


        // then:
        assert_eq!(cycles_used, 7);
        assert_eq!(cpu.SP, cpu_copy.SP - 3);
    }

    #[test]
    fn test_brk_will_push_pc_and_ps_onto_the_stack() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        mem.Data[0xFF00] = cpu.INS_BRK;
        let cpu_copy = cpu;
        let old_sp: u16 = cpu_copy.SP as u16;
        let x = 0x100 | old_sp;

        // when:
        let cycles_used = cpu.execute(&mut 7, &mut mem);


        // then:
        assert_eq!(cycles_used, 7);
        dbg!(mem.Data[((x | old_sp ) - 0) as usize]);
        assert_eq!(mem.Data[((x | old_sp ) - 0) as usize], 0xFF);
        // https://www.c64-wiki.com/wiki/BRK
        // Note that since BRK increments the program counter by 
        // 2 instead of 1, it is advisable to use a NOP after it 
        // to avoid issues
        assert_eq!(mem.Data[((x | old_sp ) - 1) as usize], 0x02);
        assert_eq!(mem.Data[((x | old_sp ) - 2) as usize], cpu_copy.PS 
                                                    | Flags::UnusedFlagBit as u8 
                                                    | Flags::BreakFlagBit as u8);
        // https://wiki.nesdev.com/w/index.php/Status_flags
        // Instruction	|Bits 5 and 4	| Side effects after pushing 
        // BRK			|	11			| I is set to 1 
        assert_eq!(cpu.PS.get_bit(2), true);

    }

    #[test]
    fn test_rti_can_return_from_an_interrupt_leaving_the_cpu_in_the_state_when_it_entered() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset_vector(&mut mem, 0xFF00);

        mem.Data[0xFF00] = cpu.INS_BRK;
        mem.Data[0xFFFE] = 0x00;
        mem.Data[0xFFFF] = 0x80;
        mem.Data[0x8000] = cpu.INS_RTI;
        let cpu_copy = cpu;

        // when:
        let cycles_used_brk = cpu.execute(&mut 7, &mut mem);
        let cycles_used_rti = cpu.execute(&mut 6, &mut mem);


        // then:
        assert_eq!(cycles_used_brk, 7);
        assert_eq!(cycles_used_rti, 6);
        assert_eq!(cpu.SP, cpu_copy.SP);
        assert_eq!(0xFF02, cpu.PC);
        assert_eq!(cpu.PS, cpu_copy.PS);
    }
}
