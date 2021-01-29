// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Microaddr.h for the primary calling header

#include "Microaddr.h"
#include "Microaddr__Syms.h"

//==========

VL_CTOR_IMP(Microaddr) {
    Microaddr__Syms* __restrict vlSymsp = __VlSymsp = new Microaddr__Syms(this, name());
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    VL_CELL(__PVT__microaddr, Microaddr_microaddr);
    // Reset internal values
    
    // Reset structure values
    _ctor_var_reset();
}

void Microaddr::__Vconfigure(Microaddr__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
    Verilated::timeunit(-12);
    Verilated::timeprecision(-12);
}

Microaddr::~Microaddr() {
    VL_DO_CLEAR(delete __VlSymsp, __VlSymsp = nullptr);
}

void Microaddr::_eval_initial(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_eval_initial\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

void Microaddr::final() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::final\n"); );
    // Variables
    Microaddr__Syms* __restrict vlSymsp = this->__VlSymsp;
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Microaddr::_eval_settle(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_eval_settle\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_settle__TOP__2(vlSymsp);
}

void Microaddr::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_ctor_var_reset\n"); );
    // Body
    clk = VL_RAND_RESET_I(1);
    reset = VL_RAND_RESET_I(1);
    cmd = VL_RAND_RESET_I(2);
    load_addr = VL_RAND_RESET_I(11);
    addr = VL_RAND_RESET_I(11);
    microaddr_counter__DOT__next_addr = VL_RAND_RESET_I(11);
}
