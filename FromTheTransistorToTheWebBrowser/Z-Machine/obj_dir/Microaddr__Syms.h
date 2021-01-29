// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table internal header
//
// Internal details; most calling programs do not need this header,
// unless using verilator public meta comments.

#ifndef _MICROADDR__SYMS_H_
#define _MICROADDR__SYMS_H_  // guard

#include "verilated.h"

// INCLUDE MODULE CLASSES
#include "Microaddr.h"
#include "Microaddr_microaddr.h"

// SYMS CLASS
class Microaddr__Syms : public VerilatedSyms {
  public:
    
    // LOCAL STATE
    const char* __Vm_namep;
    bool __Vm_didInit;
    
    // SUBCELL STATE
    Microaddr*                     TOPp;
    
    // CREATORS
    Microaddr__Syms(Microaddr* topp, const char* namep);
    ~Microaddr__Syms() = default;
    
    // METHODS
    inline const char* name() { return __Vm_namep; }
    
} VL_ATTR_ALIGNED(VL_CACHE_LINE_BYTES);

#endif  // guard
