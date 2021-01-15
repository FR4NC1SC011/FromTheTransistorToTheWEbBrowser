// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vthruwire.h for the primary calling header

#include "Vthruwire.h"
#include "Vthruwire__Syms.h"

//==========

VL_CTOR_IMP(Vthruwire) {
    Vthruwire__Syms* __restrict vlSymsp = __VlSymsp = new Vthruwire__Syms(this, name());
    Vthruwire* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Reset internal values
    
    // Reset structure values
    _ctor_var_reset();
}

void Vthruwire::__Vconfigure(Vthruwire__Syms* vlSymsp, bool first) {
    if (false && first) {}  // Prevent unused
    this->__VlSymsp = vlSymsp;
    if (false && this->__VlSymsp) {}  // Prevent unused
    Verilated::timeunit(-12);
    Verilated::timeprecision(-12);
}

Vthruwire::~Vthruwire() {
    VL_DO_CLEAR(delete __VlSymsp, __VlSymsp = nullptr);
}

void Vthruwire::_eval_initial(Vthruwire__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vthruwire::_eval_initial\n"); );
    Vthruwire* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Vthruwire::final() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vthruwire::final\n"); );
    // Variables
    Vthruwire__Syms* __restrict vlSymsp = this->__VlSymsp;
    Vthruwire* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
}

void Vthruwire::_eval_settle(Vthruwire__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vthruwire::_eval_settle\n"); );
    Vthruwire* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->_combo__TOP__1(vlSymsp);
}

void Vthruwire::_ctor_var_reset() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vthruwire::_ctor_var_reset\n"); );
    // Body
    i_sw = VL_RAND_RESET_I(1);
    o_led = VL_RAND_RESET_I(1);
}
