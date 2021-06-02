use std::os::raw::*;

//TODO: alias declaration?
// Byte = c_uchar
// Word = c_ushort


#[derive(Debug)]
struct CPU {
    PC: c_ushort,      // program counter
    SP: c_ushort,      // stack pointer

    // Registers
    A: c_uchar,        // Accumulator
    X: c_uchar,
    Y: c_uchar,

    // Status flags
    C: c_uchar,
    Z: c_uchar,
    I: c_uchar,
    D: c_uchar,
    B: c_uchar,
    V: c_uchar,
    N: c_uchar,
}

impl CPU {
    fn reset( &mut self) {
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
    }
}



fn main() {
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
    };

    println!("6502 Emulator with rust");

    println!("{:?}", cpu);

    cpu.reset();

    println!("{:?}", cpu);

}
