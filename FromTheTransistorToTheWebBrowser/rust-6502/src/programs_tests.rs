#[cfg(test)]
mod programs_tests {

    use std::os::raw::*;

    type Byte = c_uchar;
    type Word = c_ushort;

    use crate::Mem;
    use crate::CPU;

    #[test]
    fn test_load_a_program_into_the_correct_area_of_memory() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        let prg: [Byte; 14] = [
            0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
        ];

        // when
        let start_address = cpu.load_prg(prg, 14, &mut mem);

        //then
        assert_eq!(mem.Data[0x0FFF], 0x0);
        assert_eq!(mem.Data[0x1000], 0xA9);
        assert_eq!(mem.Data[0x1001], 0xFF);
        assert_eq!(mem.Data[0x1002], 0x85);
        //...
        assert_eq!(mem.Data[0x1009], 0x4C);
        assert_eq!(mem.Data[0x100A], 0x02);
        assert_eq!(mem.Data[0x100B], 0x10);
        assert_eq!(mem.Data[0x100C], 0x0);
    }

    #[test]
    fn test_load_a_program_and_execute_it() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);

        let prg: [Byte; 14] = [
            0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
        ];

        // when
        let start_address = cpu.load_prg(prg, 14, &mut mem);
        cpu.PC = start_address;

        let mut clock: i32 = 10000;
        loop {
            if clock <= 0 {
                break;
            }

            clock -= cpu.execute(&mut 1, &mut mem) as i32;
        }
    }
}
