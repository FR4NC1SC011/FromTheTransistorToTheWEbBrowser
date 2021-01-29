// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See Microaddr.h for the primary calling header

#ifndef _MICROADDR_MICROADDR_H_
#define _MICROADDR_MICROADDR_H_  // guard

#include "verilated.h"

//==========

class Microaddr__Syms;

//----------

VL_MODULE(Microaddr_microaddr) {
  public:
    
    // TYPEDEFS
    // That were declared public
    enum cmd {
        NONE = 0U,
        INC = 1U,
        LOAD = 2U
    };
    
    // INTERNAL VARIABLES
  private:
    Microaddr__Syms* __VlSymsp;  // Symbol table
  public:
    
    // CONSTRUCTORS
  private:
    VL_UNCOPYABLE(Microaddr_microaddr);  ///< Copying not allowed
  public:
    Microaddr_microaddr(const char* name = "TOP");
    ~Microaddr_microaddr();
    
    // INTERNAL METHODS
    void __Vconfigure(Microaddr__Syms* symsp, bool first);
  private:
    void _ctor_var_reset() VL_ATTR_COLD;
} VL_ATTR_ALIGNED(VL_CACHE_LINE_BYTES);

//----------


#endif  // guard
