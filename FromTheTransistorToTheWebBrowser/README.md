# From The Transistor To The Web Browser
Inspired by George Hotz -> [geohot/fromthetransistor](https://github.com/geohot/fromthetransistor).

# What did we learn from this?

* A program is just big array bytes
* Writing a CPU emulator is time consuming
* Some features are undocumented!
* Emulating a CPU is harder you than you think, because you have emulate things you got for free in a real CPU (like the clock)
* Writing Unit tests for the CPU was a good idea.
* Big switch state works for small instruction set processors, probably not for bigger ones, like the 68000.
