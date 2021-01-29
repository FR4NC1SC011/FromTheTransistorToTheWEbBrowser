// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Microaddr.h for the primary calling header

#include "Microaddr_microaddr.h"
#include "Microaddr__Syms.h"

//==========

VL_CTOR_IMP(Microaddr_microaddr) {
    // Reset internal values
    // Reset structure values
    _ctor_var_reset();
}

void Microaddr_microaddr::__Vconfigure(Microaddr__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
}

Microaddr_microaddr::~Microaddr_microaddr() {
}

void Microaddr_microaddr::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+        Microaddr_microaddr::_ctor_var_reset\n"); );
}
