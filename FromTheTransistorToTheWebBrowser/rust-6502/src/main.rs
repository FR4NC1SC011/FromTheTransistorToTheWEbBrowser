mod mos6502;

use mos6502::*;

fn main() {
    let mut mem = mos6502::Mem {
        MAX_MEM: 1024 * 64,
        Data: Vec::new(),
    };

    let mut cpu = mos6502::CPU {
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

    println!("6502 Emulator with rust");

        println!("{}", cpu.A);
        cpu.reset(&mut mem);
        cpu.X = 5;
        // start - inline a little program
        mem.Data[0xFFFC] = cpu.INS_LDA_ZPX;
        mem.Data[0xFFFD] = 0x42;
        mem.Data[0x0047] = 0x37;
        // end - inline a little program

        println!("{}", cpu.A);

        // when:
        cpu.execute(&mut 4, &mut mem);

        // then:
        assert_eq!(cpu.A, 0x37);
        println!("{}", cpu.A);

    // println!("{:x?}, {:x?}", cpu, mem);
    // cpu.reset(&mut mem);
    // println!("{:x?}, {:x?}", cpu, mem);
    // mem.Data[0xFFFC] = cpu.INS_JSR;
    // mem.Data[0xFFFD] = 0x42;
    // mem.Data[0xFFFE] = 0x42;
    // mem.Data[0x4242] = cpu.INS_LDA_IM;
    // mem.Data[0x4242] = 0x84;
    // cpu.execute(&mut 9, &mut mem);
    // println!("{:x?}, {:x?}", cpu, mem);
}




