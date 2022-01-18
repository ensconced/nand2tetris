ownership scheme:

- gate owns its own pins
- pin has vec of refs to other pins that it feeds
- does this work? why not?

end up with a struct owning some pins, and references to those pins -> rust does not make this easy to construct...

idea:
each pin has a single input...each pin needs a single owner...
so can we make that input the owner???

need to use a more general abstraction to account for sequential logic...
might have to be involving pins as the fundamental entities somehow?

use combinational logic in implementation of registers, ram, rather than rust-level for loops etc...
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
8086?
