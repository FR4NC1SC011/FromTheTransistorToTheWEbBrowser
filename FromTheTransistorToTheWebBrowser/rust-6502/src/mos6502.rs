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

    // LDA
    pub INS_LDA_IM: Byte,
    pub INS_LDA_ZP: Byte,
    pub INS_LDA_ZPX: Byte,
    pub INS_LDA_ABS: Byte,
    pub INS_LDA_ABSX: Byte,
    pub INS_LDA_ABSY: Byte,
    pub INS_LDA_INDX: Byte,
    pub INS_LDA_INDY: Byte,

    // LDX
    pub INS_LDX_IM: Byte,
    pub INS_LDX_ZP: Byte,
    pub INS_LDX_ZPY: Byte,
    pub INS_LDX_ABS: Byte,
    pub INS_LDX_ABSY: Byte,
    
    // LDY
    pub INS_LDY_IM: Byte,
    pub INS_LDY_ZP: Byte,
    pub INS_LDY_ZPX: Byte,
    pub INS_LDY_ABS: Byte,
    pub INS_LDY_ABSX: Byte,

    // STA
    pub INS_STA_ZP: Byte,
    pub INS_STA_ZPX: Byte,
    pub INS_STA_ABS: Byte,
    pub INS_STA_ABSX: Byte,
    pub INS_STA_ABSY: Byte,
    pub INS_STA_INDX: Byte,
    pub INS_STA_INDY: Byte,

    // STX
    pub INS_STX_ZP: Byte,
    pub INS_STX_ZPY: Byte,
    pub INS_STX_ABS: Byte,

    // STY
    pub INS_STY_ZP: Byte,
    pub INS_STY_ZPX: Byte,
    pub INS_STY_ABS: Byte,



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
            
            // LDA
            INS_LDA_IM: 0xA9,
            INS_LDA_ZP: 0xA5,
            INS_LDA_ZPX: 0xB5,
            INS_LDA_ABS: 0xAD,
            INS_LDA_ABSX: 0xBD,
            INS_LDA_ABSY: 0xB9,
            INS_LDA_INDX: 0xA1,
            INS_LDA_INDY: 0xB1,
            
            // LDX
            INS_LDX_IM: 0xA2,
            INS_LDX_ZP: 0xA6,
            INS_LDX_ZPY: 0xB6,
            INS_LDX_ABS: 0xAE,
            INS_LDX_ABSY: 0xBE,

            // LDY
            INS_LDY_IM: 0xA0,
            INS_LDY_ZP: 0xA4,
            INS_LDY_ZPX: 0xB4,
            INS_LDY_ABS: 0xAC,
            INS_LDY_ABSX: 0xBC,

            // STA
            INS_STA_ZP: 0x85,
            INS_STA_ZPX: 0x95,
            INS_STA_ABS: 0x8D,
            INS_STA_ABSX: 0x9D,
            INS_STA_ABSY: 0x99,
            INS_STA_INDX: 0x81,
            INS_STA_INDY: 0x91,


            // STX
            INS_STX_ZP: 0x86,
            INS_STX_ZPY: 0x96,
            INS_STX_ABS: 0x8E,

            // STY
            INS_STY_ZP: 0x84,
            INS_STY_ZPX: 0x94,
            INS_STY_ABS: 0x8C,

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

    fn write_byte(&mut self, value: Byte, cycles: &mut usize, address: Word, memory: &mut Mem) {
        memory.Data[address as usize] = value;
        *cycles = cycles.wrapping_sub(1);
    }


    pub fn execute(&mut self, cycles: &mut usize, memory: &mut Mem) -> usize {
        let cycles_requested = *cycles;
        while cycles > &mut 0 {
            let ins: Byte = self.fetch_byte(cycles, memory);

            match ins {
                0xA9 => {
                    println!("Instruction LDA Inmediate");
                    self.A = self.fetch_byte(cycles, memory);
                    self.lda_register_set_status();
                }

                0xA2 => {
                    println!("Instruction LDX Inmediate");
                    self.X = self.fetch_byte(cycles, memory);
                    self.ldx_register_set_status();
                }

                0xA0 => {
                    println!("Instruction LDY Inmediate");
                    self.Y = self.fetch_byte(cycles, memory);
                    self.ldy_register_set_status();
                }

                0xA5 => {
                    println!("Instruction LDA ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0xA6 => {
                    println!("Instruction LDX ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.X = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldx_register_set_status();
                }

                0xA4 => {
                    println!("Instruction LDY ZP");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.Y = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldy_register_set_status();
                }

                0xB4 => {
                    println!("Instruction LDY ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.Y = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldy_register_set_status();
                }

                0xB5 => {
                    println!("Instruction LDA ZPX");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.A = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.lda_register_set_status();
                }

                0xB6 => {
                    println!("Instruction LDX ZPY");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.Y);
                    *cycles -= 1;
                    self.X = self.read_byte(cycles, zero_page_address as u16, memory);
                    self.ldx_register_set_status();
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
                    self.lda_register_set_status();
                } 

               0xAE => {
                    println!("Instruction LDX Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.X = self.read_byte(cycles, abs_addrress as u16, memory);
                    self.ldx_register_set_status();
                } 

                0xAC => {
                    println!("Instruction LDY Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.Y = self.read_byte(cycles, abs_addrress as u16, memory);
                    self.ldy_register_set_status();
                }

                0xBC => {
                    println!("Instruction LDY Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.Y = self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.ldy_register_set_status();
                }

                0xBD => {
                    println!("Instruction LDA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0xBE => {
                    println!("Instruction LDX Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.X = self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.ldx_register_set_status();
                }

                0xB9 => {
                    println!("Instruction LDA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_y: Word = abs_addrress + self.Y as u16;
                    self.A = self.read_byte(cycles, abs_address_plus_y, memory);
                    if abs_address_plus_y - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    self.lda_register_set_status();
                }

                0xA1 => {
                    println!("Instruction LDA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word = self.read_word(cycles, zero_page_address as u16, memory);
                    self.A = self.read_byte(cycles, effective_address, memory);
                    self.lda_register_set_status();
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
                    self.lda_register_set_status();
                }






                0x85 => {
                    println!("Instruction STA Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.A, cycles, zero_page_address as u16, memory);
                }

                0x95 => {
                    println!("Instruction STA Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.A, cycles, zero_page_address as u16, memory);
                }

                0x86 => {
                    println!("Instruction STX Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.X, cycles, zero_page_address as u16, memory);
                }

                0x84 => {
                    println!("Instruction STY Zero Page");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.write_byte(self.Y, cycles, zero_page_address as u16, memory);
                }

                0x94 => {
                    println!("Instruction STY Zero Page X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address = zero_page_address.wrapping_add(self.X);
                    *cycles -= 1;
                    self.write_byte(self.Y, cycles, zero_page_address as u16, memory);
                }

                0x8D => {
                    println!("Instriction STA Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.A, cycles, abs_addrress as u16, memory);
                }

                0x8E => {
                    println!("Instriction STX Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.X, cycles, abs_addrress as u16, memory);
                }

                0x8C => {
                    println!("Instruction STY Absolute");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    self.write_byte(self.Y, cycles, abs_addrress as u16, memory);
                }

                0x9D => {
                    println!("Instruction STA Absolute X");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.X as u16;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    *cycles -= 1;       // TODO: check is this is correct
                }

                0x99 => {
                    println!("Instruction STA Absolute Y");
                    let abs_addrress: Word = self.fetch_word(cycles, memory);
                    let abs_address_plus_x: Word = abs_addrress + self.Y as u16;
                    self.write_byte(self.A, cycles, abs_address_plus_x, memory);
                    if abs_address_plus_x - abs_addrress >= 0xFF {
                        *cycles -= 1;
                    }
                    *cycles -= 1;       // TODO: check is this is correct
                }

                0x81 => {
                    println!("Instruction STA Indirect X");
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    let effective_address: Word = self.read_word(cycles, zero_page_address as u16, memory);
                    self.write_byte(self.A, cycles, effective_address, memory);
                }

                0x91 => {
                    println!("Instruction STA Indirect Y");
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word = self.read_word(cycles, zero_page_address as u16, memory);
                    let effective_address_y: Word = effective_address + self.Y as u16;
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.write_byte(self.A, cycles, effective_address_y, memory);
                    *cycles -= 1;
   
                }




                _ => {
                    unimplemented!("Instruction not handled {}", ins);
                }
            }
        }

         cycles_requested - *cycles
    }

    fn lda_register_set_status(&mut self) {
        self.Z = match self.A == 0 {
            false => 0,
            true => 1,
        };

        self.N = match (self.A & 0b10000000) > 0 {
            false => 0,
            true => 1,
        };
    }

    fn ldx_register_set_status(&mut self) {
        self.Z = match self.X == 0 {
            false => 0,
            true => 1,
        };

        self.N = match (self.X & 0b10000000) > 0 {
            false => 0,
            true => 1,
        };
    }

    fn ldy_register_set_status(&mut self) {
        self.Z = match self.Y == 0 {
            false => 0,
            true => 1,
        };

        self.N = match (self.Y & 0b10000000) > 0 {
            false => 0,
            true => 1,
        };
    }


}


