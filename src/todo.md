debugging add16

- conditionally include debug output
- write util to parse debug output into regraph playground - with each component in combo...
- ...or just to generate a human-readable report without regraph?
- and highlight any cycles...

performance ideas

- profiling!
- push instead of pull by doing toposort then reversing
- when pushing, use transactions when setting full set of inputs to avoid multiplying work
- pare down long eq chains

refactoring ideas

- use macros for generating similar gates with diff number of inputs etc

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
