// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Primary design header
//
// This header should be included by all source files instantiating the design.
// The class here is then constructed to instantiate the design.
// See the Verilator manual for examples.

#ifndef _MICROADDR_H_
#define _MICROADDR_H_  // guard

#include "verilated.h"

//==========

class Microaddr__Syms;
class Microaddr_microaddr;


//----------

VL_MODULE(Microaddr) {
  public:
    // CELLS
    // Public to allow access to /*verilator_public*/ items;
    // otherwise the application code can consider these internals.
    Microaddr_microaddr* __PVT__microaddr;
    
    // PORTS
    // The application code writes and reads these signals to
    // propagate new values into/out from the Verilated model.
    VL_IN8(clk,0,0);
    VL_IN8(reset,0,0);
    VL_IN8(cmd,1,0);
    VL_IN16(load_addr,10,0);
    VL_OUT16(addr,10,0);
    
    // LOCAL SIGNALS
    // Internals; generally not touched by application code
    SData/*10:0*/ microaddr_counter__DOT__next_addr;
    
    // LOCAL VARIABLES
    // Internals; generally not touched by application code
    CData/*0:0*/ __Vclklast__TOP__clk;
    
    // INTERNAL VARIABLES
    // Internals; generally not touched by application code
    Microaddr__Syms* __VlSymsp;  // Symbol table
    
    // CONSTRUCTORS
  private:
    VL_UNCOPYABLE(Microaddr);  ///< Copying not allowed
  public:
    /// Construct the model; called by application code
    /// The special name  may be used to make a wrapper with a
    /// single model invisible with respect to DPI scope names.
    Microaddr(const char* name = "TOP");
    /// Destroy the model; called (often implicitly) by application code
    ~Microaddr();
    
    // API METHODS
    /// Evaluate the model.  Application must call when inputs change.
    void eval() { eval_step(); }
    /// Evaluate when calling multiple units/models per time step.
    void eval_step();
    /// Evaluate at end of a timestep for tracing, when using eval_step().
    /// Application must call after all eval() and before time changes.
    void eval_end_step() {}
    /// Simulation complete, run final blocks.  Application must call on completion.
    void final();
    
    // INTERNAL METHODS
    static void _eval_initial_loop(Microaddr__Syms* __restrict vlSymsp);
    void __Vconfigure(Microaddr__Syms* symsp, bool first);
  private:
    static QData _change_request(Microaddr__Syms* __restrict vlSymsp);
    static QData _change_request_1(Microaddr__Syms* __restrict vlSymsp);
    void _ctor_var_reset() VL_ATTR_COLD;
  public:
    static void _eval(Microaddr__Syms* __restrict vlSymsp);
  private:
#ifdef VL_DEBUG
    void _eval_debug_assertions();
#endif  // VL_DEBUG
  public:
    static void _eval_initial(Microaddr__Syms* __restrict vlSymsp) VL_ATTR_COLD;
    static void _eval_settle(Microaddr__Syms* __restrict vlSymsp) VL_ATTR_COLD;
    static void _sequent__TOP__1(Microaddr__Syms* __restrict vlSymsp);
    static void _settle__TOP__2(Microaddr__Syms* __restrict vlSymsp);
} VL_ATTR_ALIGNED(VL_CACHE_LINE_BYTES);

//----------


#endif  // guard
