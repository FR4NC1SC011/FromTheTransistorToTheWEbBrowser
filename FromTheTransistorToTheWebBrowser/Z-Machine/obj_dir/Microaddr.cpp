// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Microaddr.h for the primary calling header

#include "Microaddr.h"
#include "Microaddr__Syms.h"

//==========

void Microaddr::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Microaddr::eval\n"); );
    Microaddr__Syms* __restrict vlSymsp = this->__VlSymsp;  // Setup global symbol table
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
#ifdef VL_DEBUG
    // Debug assertions
    _eval_debug_assertions();
#endif  // VL_DEBUG
    // Initialize
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) _eval_initial_loop(vlSymsp);
    // Evaluate till stable
    int __VclockLoop = 0;
    QData __Vchange = 1;
    do {
        VL_DEBUG_IF(VL_DBG_MSGF("+ Clock loop\n"););
        _eval(vlSymsp);
        if (VL_UNLIKELY(++__VclockLoop > 100)) {
            // About to fail, so enable debug to see what's not settling.
            // Note you must run make with OPT=-DVL_DEBUG for debug prints.
            int __Vsaved_debug = Verilated::debug();
            Verilated::debug(1);
            __Vchange = _change_request(vlSymsp);
            Verilated::debug(__Vsaved_debug);
            VL_FATAL_MT("microaddr_counter.v", 1, "",
                "Verilated model didn't converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

void Microaddr::_eval_initial_loop(Microaddr__Syms* __restrict vlSymsp) {
    vlSymsp->__Vm_didInit = true;
    _eval_initial(vlSymsp);
    // Evaluate till stable
    int __VclockLoop = 0;
    QData __Vchange = 1;
    do {
        _eval_settle(vlSymsp);
        _eval(vlSymsp);
        if (VL_UNLIKELY(++__VclockLoop > 100)) {
            // About to fail, so enable debug to see what's not settling.
            // Note you must run make with OPT=-DVL_DEBUG for debug prints.
            int __Vsaved_debug = Verilated::debug();
            Verilated::debug(1);
            __Vchange = _change_request(vlSymsp);
            Verilated::debug(__Vsaved_debug);
            VL_FATAL_MT("microaddr_counter.v", 1, "",
                "Verilated model didn't DC converge\n"
                "- See DIDNOTCONVERGE in the Verilator manual");
        } else {
            __Vchange = _change_request(vlSymsp);
        }
    } while (VL_UNLIKELY(__Vchange));
}

VL_INLINE_OPT void Microaddr::_sequent__TOP__1(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_sequent__TOP__1\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->addr = vlTOPp->microaddr_counter__DOT__next_addr;
}

VL_INLINE_OPT void Microaddr::_settle__TOP__2(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_settle__TOP__2\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    vlTOPp->microaddr_counter__DOT__next_addr = (0x7ffU 
                                                 & ((0U 
                                                     == (IData)(vlTOPp->cmd))
                                                     ? (IData)(vlTOPp->addr)
                                                     : 
                                                    ((1U 
                                                      == (IData)(vlTOPp->cmd))
                                                      ? 
                                                     ((IData)(1U) 
                                                      + (IData)(vlTOPp->addr))
                                                      : 
                                                     ((2U 
                                                       == (IData)(vlTOPp->cmd))
                                                       ? (IData)(vlTOPp->load_addr)
                                                       : 0U))));
    if (vlTOPp->reset) {
        vlTOPp->microaddr_counter__DOT__next_addr = 0U;
    }
}

void Microaddr::_eval(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_eval\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    if (((IData)(vlTOPp->clk) & (~ (IData)(vlTOPp->__Vclklast__TOP__clk)))) {
        vlTOPp->_sequent__TOP__1(vlSymsp);
    }
    vlTOPp->_settle__TOP__2(vlSymsp);
    // Final
    vlTOPp->__Vclklast__TOP__clk = vlTOPp->clk;
}

VL_INLINE_OPT QData Microaddr::_change_request(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_change_request\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    return (vlTOPp->_change_request_1(vlSymsp));
}

VL_INLINE_OPT QData Microaddr::_change_request_1(Microaddr__Syms* __restrict vlSymsp) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_change_request_1\n"); );
    Microaddr* const __restrict vlTOPp VL_ATTR_UNUSED = vlSymsp->TOPp;
    // Body
    // Change detection
    QData __req = false;  // Logically a bool
    return __req;
}

#ifdef VL_DEBUG
void Microaddr::_eval_debug_assertions() {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Microaddr::_eval_debug_assertions\n"); );
    // Body
    if (VL_UNLIKELY((clk & 0xfeU))) {
        Verilated::overWidthError("clk");}
    if (VL_UNLIKELY((reset & 0xfeU))) {
        Verilated::overWidthError("reset");}
    if (VL_UNLIKELY((cmd & 0xfcU))) {
        Verilated::overWidthError("cmd");}
    if (VL_UNLIKELY((load_addr & 0xf800U))) {
        Verilated::overWidthError("load_addr");}
}
#endif  // VL_DEBUG
