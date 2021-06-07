
use std::os::raw::*;
use std::num::Wrapping;

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
    pub INS_JSR: c_uchar,       // TODO: Fix it

}

impl Mem {
    fn initialize(&mut self) {
        for d in 0..self.MAX_MEM {
            self.Data.push(0);
        }
    }

    // write 2 bytes 
    fn write_word(&mut self, value: c_ushort, address: u32, cycles: &mut usize) {
        self.Data[address as usize]     = (value & 0xFF) as u8;
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

    pub fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) {
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
                    println!("Instruction Load ZP");
                    let mut zero_page_address: c_uchar = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address, memory);
                    self.lda_set_status();
                }

                0x20 => {
                    println!("Instruction Load JSR");
                    let mut sub_addr: c_ushort = self.fetch_word(cycles, memory);
                    memory.write_word(self.PC - 1, self.SP as u32, cycles);
                    self.PC = sub_addr;
                    *cycles -= 1;
                }

                _ => eprintln!("Instruction not handled {}", ins),
            }
        }
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


