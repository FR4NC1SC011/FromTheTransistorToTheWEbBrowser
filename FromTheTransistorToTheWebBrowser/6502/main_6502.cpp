#include <stdio.h>
#include <stdlib.h>


using Byte = unsigned char;
using Word = unsigned short;

using u32 = unsigned int;
using s32 = signed int;

struct Mem {
  
  static constexpr u32 MAX_MEM = 1024 * 64;
  Byte Data[MAX_MEM];

  void Initialise() {
    for (u32 i = 0; i < MAX_MEM; i++) {
      Data[i] = 0;
    }
  }
  
  // read 1 byte
  Byte operator[] (u32 Address) const {
    // Assert here Address is < MAX_MEM
    return Data[Address];
  }


  // write 1 byte
  Byte& operator[] (u32 Address) {
    // Assert here Address is < MAX_MEM
    return Data[Address];
  }

  void WriteWord(Word Value, u32 Address, s32& Cycles) {
    Data[Address] = Value & 0xFF;
    Data[Address + 1] = (Value >> 8);
    Cycles -= 2;
  }
};

struct CPU {
  Word PC;  // program counter 
  Word SP;  // stack pointer

  Byte A, X, Y; // registers

  Byte C : 1;   // Status flag
  Byte Z : 1;
  Byte I : 1;
  Byte D : 1;
  Byte B : 1;
  Byte V : 1;
  Byte N : 1;


  void Reset(Mem& memory) {
    PC = 0xFFFC;
    SP = 0x0100;
    C = Z = I = D = B = V = N = 0;
    A = X = Y = 0;
    memory.Initialise();
  }

  Byte FetchByte(s32& Cycles, Mem& memory) {
    Byte Data = memory[PC];
    PC++;
    Cycles--;
    return Data;
  }

  Word FetchWord(s32& Cycles, Mem& memory) {
    // 6502 is little endian
    Word Data = memory[PC];
    PC++;

    Data |= (memory[PC] << 8);
    PC++;

    Cycles += 2;

    return Data;
  }

  Byte ReadByte(s32& Cycles, Word Address, Mem& memory) {
    Byte Data = memory[Address];
    Cycles--;
    return Data;
  }

  static constexpr Byte
    INS_LDA_IM = 0xA9,
    INS_LDA_ZP = 0xA5,
    INS_LDA_ZPX = 0xB5,
    INS_LDA_ABS = 0xAD,
    INS_LDA_ABSX = 0xBD,
    INS_LDA_ABSY = 0xB9,
    INS_LDA_INDX = 0xA1,
    INS_LDA_INDY = 0xB1,
    INS_JSR = 0x20;



  void LDASetStatus() {
    Z = (A == 0);
    N = (A & 0b10000000) > 0;
  }

  // Return the number of cycles that were used
  s32 Execute(s32 Cycles, Mem& memory) {

    const s32 CyclesRequested = Cycles;
    while (Cycles > 0) {
      Byte Ins = FetchByte(Cycles, memory);
      switch(Ins) {
        case INS_LDA_IM:
          {
            Byte Value = FetchByte(Cycles, memory);
            A = Value;
            LDASetStatus();
          } break;

        case INS_LDA_ZP:
          {
            Byte ZPAddress = FetchByte(Cycles, memory);
            A = ReadByte(Cycles, ZPAddress, memory);
            LDASetStatus();
          } break;

        case INS_LDA_ZPX:
          {
            Byte ZPXAddress = FetchByte(Cycles, memory);
            ZPXAddress += X;
            Cycles--;
            A = ReadByte(Cycles, ZPXAddress, memory);
            LDASetStatus();
          } break;

        case INS_LDA_ABS: 
          {
            Word AbsAddr = FetchWord(Cycles, memory);
            ReadByte(Cycles, AbsAddr, memory);
          } break;

        case INS_LDA_ABSX:
          {
            Word AbsAddr = FetchWord(Cycles, memory);
            Word AbsAddrX =  AbsAddr + X;
            ReadByte(Cycles, AbsAddrX, memory);

            if (AbsAddrX - AbsAddr >= 0xFF) {
              Cycles--;
            }
          } break;

        case INS_JSR: 
          {
            Word SubAddr = FetchWord(Cycles, memory);
            memory.WriteWord(PC - 1, SP, Cycles);
            SP += 2;
            PC = SubAddr;
            Cycles--;
          } break;

        default:
          {
            printf("Instruction not handled %d", Ins);
            throw -1;
          } break;
      }
    }
  
    const s32 NumCyclesUsed = CyclesRequested - Cycles;
    return NumCyclesUsed;
  }
};


int main() {
  Mem mem;
  CPU cpu;
  cpu.Reset(mem);
  mem[0xFFFC] = CPU::INS_LDA_ZP;
  mem[0xFFFD] = 0x42;
  mem[0x0042] = 0x84;
  cpu.Execute(3, mem);
  return 0;

}
