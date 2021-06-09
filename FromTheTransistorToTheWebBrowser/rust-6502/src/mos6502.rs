use num_traits::WrappingShl;
use std::num::Wrapping;
use std::os::raw::*;

//TODO: alias declaration?
// Byte = c_uchar
// Word = c_ushort

#[derive(Debug)]
pub struct Mem {
    pub MAX_MEM: u32,
    pub Data: Vec<c_uchar>,
}

#[derive(Debug)]
pub struct CPU {
    pub PC: c_ushort, // program counter
    pub SP: c_ushort, // stack pointer

    // Registers
    pub A: c_uchar, // Accumulator
    pub X: c_uchar,
    pub Y: c_uchar,

    // Status flags
    pub C: c_uchar,
    pub Z: c_uchar,
    pub I: c_uchar,
    pub D: c_uchar,
    pub B: c_uchar,
    pub V: c_uchar,
    pub N: c_uchar,

    // Opcodes
    pub INS_LDA_IM: c_uchar,
    pub INS_LDA_ZP: c_uchar,
    pub INS_LDA_ZPX: c_uchar,
    pub INS_LDA_ABS: c_uchar,
    pub INS_LDA_ABSX: c_uchar,
    pub INS_LDA_ABSY: c_uchar,
    pub INS_LDA_INDX: c_uchar,
    pub INS_LDA_INDY: c_uchar,
    pub INS_JSR: c_uchar, // TODO: Fix overflow
}

impl Mem {
    fn initialize(&mut self) {
        for d in 0..self.MAX_MEM {
            self.Data.push(0);
        }
    }

    pub fn new() -> Self {
        Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        }
    }

    // write 2 bytes
    fn write_word(&mut self, value: c_ushort, address: u32, cycles: &mut usize) {
        self.Data[address as usize] = (value & 0xFF) as u8;
        self.Data[(address + 1) as usize] = (value >> 8) as u8;
        *cycles -= 2;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            PC: 0,
            SP: 0,

            A: 0,
            X: 0,
            Y: 0,

            C: 1,
            Z: 1,
            I: 1,
            D: 1,
            B: 1,
            V: 1,
            N: 1,

            // Opcodes
            INS_LDA_IM: 0xA9,
            INS_LDA_ZP: 0xA5,
            INS_LDA_ZPX: 0xB5,
            INS_LDA_ABS: 0xAD,
            INS_LDA_ABSX: 0xBD,
            INS_LDA_ABSY: 0xB9,
            INS_LDA_INDX: 0xA1,
            INS_LDA_INDY: 0xB1,
            INS_JSR: 0x20,
        }
    }

    pub fn reset(&mut self, memory: &mut Mem) {
        self.PC = 0xFFFC;
        self.SP = 0x0100;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.C = 0;
        self.Z = 0;
        self.I = 0;
        self.D = 0;
        self.B = 0;
        self.V = 0;
        self.N = 0;

        memory.initialize();
    }

    fn fetch_word(&mut self, cycles: &mut usize, memory: &mut Mem) -> c_ushort {
        // 6502 is little endian
        let mut data: c_ushort = memory.Data[self.PC as usize] as u16;
        self.PC += 1;

        // data |= (memory.Data[self.PC as usize] << 8 as u32) as u16;
        // data = data.wrapping_shl(memory.Data[self.PC as usize] as u32);
        data = data + WrappingShl::wrapping_shl(&data, 8);

        self.PC += 1;

        *cycles -= 2;
        data
    }

    fn fetch_byte(&mut self, cycles: &mut usize, memory: &mut Mem) -> c_uchar {
        let data: c_uchar = memory.Data[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;
        data
    }

    fn read_byte(&mut self, cycles: &mut usize, address: c_uchar, memory: &mut Mem) -> c_uchar {
        let data: c_uchar = memory.Data[address as usize];
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) -> usize {
        let cycles_requested = *cycles;
        while cycles > &mut 0 {
            let ins: c_uchar = self.fetch_byte(cycles, memory);

            match ins {
                0xA9 => {
                    println!("Instruction Load A");
                    let value: c_uchar = self.fetch_byte(cycles, memory);
                    self.A = value;
                    self.lda_set_status();
                }

                0xA5 => {
                    println!("Instruction Load ZP");
                    let zero_page_address: c_uchar = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                0xB5 => {
                    println!("Instruction Load ZPX");
                    let mut zero_page_address: c_uchar = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                0x20 => {
                    println!("Instruction Load JSR");
                    let mut sub_addr: c_ushort = self.fetch_word(cycles, memory);
                    memory.write_word(self.PC - 1, self.SP as u32, cycles);
                    self.SP += 2;
                    self.PC = sub_addr;
                    *cycles -= 1;
                }
                _ => eprintln!("Instruction not handled {}", ins),
            }
        }
         cycles_requested - *cycles
    }

    fn lda_set_status(&mut self) {
        self.Z = match self.A == 0 {
            false => 0,
            true => 1,
        };

        self.N = match (self.A & 0b10000000) > 0 {
            false => 0,
            true => 1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldainmediate_can_load_value_into_a_register() {
        // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_IM;
        mem.Data[0xFFFD] = 0x84;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.A, 0x84);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldainmediate_can_affect_the_zero_flag() {
        // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.A = 0x29;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_IM;
        mem.Data[0xFFFD] = 0x0;

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);

        // then:
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazp_can_load_value_into_a_register() {
        // LDAZeroPageCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);
        assert_eq!(cycles_used, 3);

        // then:
        assert_eq!(cpu.A, 0x37);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazpx_can_load_value_into_a_register() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        cpu.X = 5;

        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);
        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn test_ldazpx_can_load_value_into_a_register_when_it_wraps() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        cpu.X = 0xFF;

        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x007F] = 0x37;

        // when:
        let mut cpu_copy = CPU::new();
        cpu_copy.reset(&mut mem);

        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);

        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
    fn the_cpu_does_nothing_when_we_execute_cero_cycles() {
        // given: 
        let mut cpu = CPU::new();
        let mut mem = Mem::new();

        // when:
        let cycles_used = cpu.execute(&mut 0, &mut mem);

        // then: 
        assert_eq!(cycles_used, 0);
    }

//      #[test]


//     fn cpu_can_execute_more_cycles_than_requested_if_required_by_the_instruction() {
//         // LDAInmediateCanLoadAValueIntoTheAReg
//         let mut mem = Mem::new();
//         let mut cpu = CPU::new();
//         let mut cpu_copy = CPU::new();
// 
//         // given:
//         cpu.reset(&mut mem);
//         cpu_copy.reset(&mut mem);
//         mem.Data[0xFFFC] = cpu.INS_LDA_IM;
//         mem.Data[0xFFFD] = 0x84;
// 
//         // when:
//         let cycles_used = cpu.execute(&mut 1, &mut mem);
// 
//         // then:
//         assert_eq!(cycles_used, 2);
//     }
// 

      #[test]
     fn executing_a_bad_inst_does_not_put_us_into_an_infinite_loop() {
         let mut mem = Mem::new();
         let mut cpu = CPU::new();
         let mut cpu_copy = CPU::new();
 
         // given:
         cpu.reset(&mut mem);
         cpu_copy.reset(&mut mem);
         mem.Data[0xFFFC] = 0x0;
         mem.Data[0xFFFD] = 0x0;
 
         // when:
         let cycles_used = cpu.execute(&mut 3, &mut mem);
 
         // then:
         assert_eq!(cycles_used, 3);
     }

    #[test]
    fn test_ldaabs_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();

        // given:
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABS;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;  // 0x4480
        mem.Data[0x4480] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
    }

     #[test]
     fn test_ldaabsx_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.X = 1;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;  // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }
    
     #[test]
     fn test_ldaabsx_can_load_value_into_a_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.X = 0xFF;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSX;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44;  // 0x4402
        mem.Data[0x4501] = 0x37;  // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
     fn test_ldaabsy_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.Y = 1;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSY;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0xFFFE] = 0x44;  // 0x4480
        mem.Data[0x4481] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }
    
     #[test]
     fn test_ldaabsy_can_load_value_into_a_register_when_it_crosses_page_boundary() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.Y = 0xFF;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ABSY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0xFFFE] = 0x44;  // 0x4402
        mem.Data[0x4501] = 0x37;  // 0x4402 + 0xFF crosses page boundary!

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
     fn test_ldaindx_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.X = 0x04;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_INDX;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0006] = 0x00;  // 0x02 + 0x04
        mem.Data[0x0007] = 0x80;
        mem.Data[0x8000] = 0x37;

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

     #[test]
     fn test_ldaindy_can_load_value_into_a_register() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.Y = 0x04;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_INDY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x00;  
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8004] = 0x37;  //0x8000 + 0x4

        // when:
        let cycles_used = cpu.execute(&mut 5, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 5);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }

    #[test]
     fn test_ldaindy_can_load_value_into_a_register_when_it_crosses_a_page() {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        let mut cpu_copy = CPU::new();

        // given:
        cpu.Y = 0xFF;
        cpu.reset(&mut mem);
        cpu_copy.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_INDY;
        mem.Data[0xFFFD] = 0x02;
        mem.Data[0x0002] = 0x02;  
        mem.Data[0x0003] = 0x80;
        mem.Data[0x8101] = 0x37;  //0x8002 + 0xFF

        // when:
        let cycles_used = cpu.execute(&mut 6, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 6);
        assert_eq!(cpu.Z, 1);
        assert_eq!(cpu.N, 0);
        verify_unmodified_flags_from_lda(cpu, cpu_copy);
    }






    fn verify_unmodified_flags_from_lda(cpu: CPU, cpu_copy: CPU) {
        assert_eq!(cpu.C, cpu_copy.C);
        assert_eq!(cpu.I, cpu_copy.I);
        assert_eq!(cpu.D, cpu_copy.D);
        assert_eq!(cpu.B, cpu_copy.B);
        assert_eq!(cpu.V, cpu_copy.V);
    }
}
