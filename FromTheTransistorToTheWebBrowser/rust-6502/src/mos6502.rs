use num_traits::WrappingShl;
use std::os::raw::*;

type Byte = c_uchar;
type Word = c_ushort;

#[derive(Debug)]
pub struct Mem {
    pub MAX_MEM: u32,
    pub Data: Vec<Byte>,
}

#[derive(Debug)]
pub struct CPU {
    pub PC: Word, // program counter
    pub SP: Word, // stack pointer

    // Registers
    pub A: Byte, // Accumulator
    pub X: Byte,
    pub Y: Byte,

    // Status flags
    pub C: Byte,
    pub Z: Byte,
    pub I: Byte,
    pub D: Byte,
    pub B: Byte,
    pub V: Byte,
    pub N: Byte,

    // Opcodes
    pub INS_LDA_IM: Byte,
    pub INS_LDA_ZP: Byte,
    pub INS_LDA_ZPX: Byte,
    pub INS_LDA_ABS: Byte,
    pub INS_LDA_ABSX: Byte,
    pub INS_LDA_ABSY: Byte,
    pub INS_LDA_INDX: Byte,
    pub INS_LDA_INDY: Byte,
    pub INS_JSR: Byte, // TODO: Fix overflow
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
    fn write_word(&mut self, value: Word, address: u32, cycles: &mut usize) {
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

    fn fetch_word(&mut self, cycles: &mut usize, memory: &mut Mem) -> Word {
        // 6502 is little endian
        let mut data: Word = memory.Data[self.PC as usize] as Word;
        self.PC += 1;

        data |= WrappingShl::wrapping_shl(&(memory.Data[self.PC as usize] as Word), 8);
        self.PC += 1;

        *cycles -= 2;
        data
    }

    fn fetch_byte(&mut self, cycles: &mut usize, memory: &mut Mem) -> Byte {
        let data: Byte = memory.Data[self.PC as usize];
        // self.PC += 1;
        self.PC = self.PC.wrapping_add(1);
        *cycles -= 1;
        data
    }

    fn read_byte(&mut self, cycles: &mut usize, address: Word, memory: &mut Mem) -> Byte {
        let data: Byte = memory.Data[address as usize];
        // *cycles -= 1;
        *cycles = cycles.wrapping_sub(1);
        data
    }

    fn read_word(&mut self, cycles: &mut usize, address: Word, memory: &mut Mem) -> Word {

        let lo_byte: Byte = self.read_byte(cycles, address, memory);
        let hi_byte: Byte = self.read_byte(cycles, address + 1, memory);

        let mut data: Word = lo_byte as Word;
        data |= WrappingShl::wrapping_shl(&(hi_byte as Word), 8);

        data        
    }


    pub fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) -> usize {
        let cycles_requested = *cycles;
        while cycles > &mut 0 {
            let ins: Byte = self.fetch_byte(cycles, memory);

            match ins {
                0xA9 => {
                    println!("Instruction LDA Inmediate");
                    let value: Byte = self.fetch_byte(cycles, memory);
                    self.A = value;
                    self.lda_set_status();
                }

                0xA5 => {
                    println!("Instruction Load ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_set_status();
                }

                0xB5 => {
                    println!("Instruction Load ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_set_status();
                }

                0x20 => {
                    println!("Instruction Load JSR");
                    let sub_addr: Word = self.fetch_word(cycles, memory);
                    memory.write_word(self.PC - 1, self.SP as u32, cycles);
                    self.SP += 2;
                    self.PC = sub_addr;
                    *cycles -= 1;
                }

                0xAD => {
                    println!("Instruction LDA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.A = self.read_byte(cycles, abs_addrress as u16, memory);
                } 

                0xBD => {
                    println!("Instruction LDA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                }

                0xB9 => {
                    println!("Instruction LDA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                }

                0xA1 => {
                    println!("Instruction LDA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word = self.read_word(cycles, zero_page_address as u16, memory);
                    self.A = self.read_byte(cycles, effective_address, memory);
                }

                0xB1 => {
                    println!("Instruction LDA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word = self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    self.A = self.read_byte(cycles, effective_address_y, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }

                }

                _ => {
                    unimplemented!("Instruction not handled {}", ins);
                }
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


