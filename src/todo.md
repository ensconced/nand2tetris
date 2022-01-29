do we need a new connection enum variant for flipflops, to allow properly including everything in graph?

- conditionally include debug output
- write util to parse debug output into regraph playground - with each component in combo...
- ...or just to generate a human-readable report without regraph?
- and highlight any cycles...

performance ideas

- profiling!
- only do sorting once - not every time in sort_and_compute (although a function
  that does it all will probably we handy for making tests less verbose)
- on tick, only compute downstream from changed pins?
- remove useless pins (i.e. compress long chains)
- threading
- convert graph from linked-list style to more data-oriented format before
  computation - vec of raw Pins without Rc? - connections are only necessary for
  flipflops and can use indices instead of refs

refactoring ideas

- DRY up ram code
- use macros for generating similar gates with diff number of inputs etc
- cargo clippy and address issues

extra ideas

implement bitregisters from basic logic gates rather than taking as primitive?
create visualizer for this VM and others - use wasm
write compiler for a simple HDL and just use that...

VMs which could be included in visualizer...

chip-8
hack
crafting-interpreters lox vm
llvm
wasm bytecode vm
AVR / arduino
other real well known but simple ISAs
8086
6502
