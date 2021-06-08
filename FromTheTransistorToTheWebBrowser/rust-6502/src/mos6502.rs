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
    pub INS_JSR: c_uchar, // TODO: Fix overflow
}

impl Mem {
    fn initialize(&mut self) {
        for d in 0..self.MAX_MEM {
            self.Data.push(0);
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

        data |= (memory.Data[self.PC as usize] << 8 as u32) as u16;

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
                    zero_page_address += self.X;
                    // zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                // TODO: Fix overflow
                // 0x20 => {
                //     println!("Instruction Load JSR");
                //     let mut sub_addr: c_ushort = self.fetch_word(cycles, memory);
                //     memory.write_word(self.PC - 1, self.SP as u32, cycles);
                //     self.PC = sub_addr;
                //     *cycles -= 1;
                // }
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
    fn test_LDAInmValueintoARegister() {
        // LDAInmediateCanLoadAValueIntoTheAReg
        let mut mem = Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        };

        let mut cpu = CPU {
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
            INS_JSR: 0x20,
        };

        // given:
        // start - inline a little program
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_IM;
        mem.Data[0xFFFD] = 0x84;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 2, &mut mem);
        assert_eq!(cycles_used, 2);

        // then:
        assert_eq!(cpu.A, 0x84);
    }

    #[test]
    fn test_LDAZPValueintoARegister() {
        // LDAZeroPageCanLoadAValueIntoTheAReg
        let mut mem = Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        };

        let mut cpu = CPU {
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
            INS_JSR: 0x20,
        };

        // given:
        // start - inline a little program
        cpu.reset(&mut mem);
        mem.Data[0xFFFC] = cpu.INS_LDA_ZP;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0042] = 0x37;
        // end - inline a little program

        // when:
        let cycles_used = cpu.execute(&mut 3, &mut mem);
        assert_eq!(cycles_used, 3);

        // then:
        assert_eq!(cpu.A, 0x37);
    }

    #[test]
    fn test_LDAZPXValueintoARegister() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        };

        let mut cpu = CPU {
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
            INS_JSR: 0x20,
        };

        // given:
        cpu.reset(&mut mem);
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
    }

  #[test]
    fn test_LDAZPXValueintoARegisterWhenItWraps() {
        // LDAZeroPageXCanLoadAValueIntoTheAReg
        let mut mem = Mem {
            MAX_MEM: 1024 * 64,
            Data: Vec::new(),
        };

        let mut cpu = CPU {
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
            INS_JSR: 0x20,
        };

        

        // given:
        cpu.reset(&mut mem);
        cpu.X = 0xFF;

        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x80;
        mem.Data[0x007F] = 0x37;
        // end - inline a little program


        // when:
        let mut cpu_copy = CPU {
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
            INS_JSR: 0x20,
        };
        let cycles_used = cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        assert_eq!(cycles_used, 4);


        assert_eq!(cpu.C, cpu_copy.C);
        assert_eq!(cpu.I, cpu_copy.I);
        assert_eq!(cpu.D, cpu_copy.D);
        assert_eq!(cpu.B, cpu_copy.B);
        assert_eq!(cpu.V, cpu_copy.V);
    }

}
