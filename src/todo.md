TODO

- tidy up
- break project into better parts - separate crates? - HDL part, optimization part...

I think getting good enough performance out of this with gate-level emulation is going to
be very difficult, if not impossible...

well maybe I'm not quite ready yet to give up. It would be good to know what clock speed will be necessary to say, get tetris working...

so the new goal is to do it dynamically i.e. have different levels of simulation, which we switch
between in the visualisation as you focus on different elements.

I guess that means each element should just have a high-level simulation...but you can drill down as far as you like.

do we need a new connection enum variant for flipflops, to allow properly including everything in graph?

- conditionally include debug output
- write util to parse debug output into regraph playground - with each component in combo...
- ...or just to generate a human-readable report without regraph?
- and highlight any cycles...

performance ideas

- when you remove sequential logic (i.e. flipflops) you should be left with some combinational units which can be memoized...

- https://en.wikipedia.org/wiki/Logic_optimization

- uplifting logic by spotting and compressing motifs in netlist into "smart nodes"? e.g. dedicated mux nodes, dedicated register nodes...etc etc

- multiple passes of uplifting...

- this would mean you basically implement multiple levels of abstraction at once - a Ram16k would be implemented directly in rust, but at the same time you can add whatever gate-level logic you want, it just might not be so optimized...

- break up

- can we have each ram512, or ram4k in a separate thread?

- use one representation for pins/connections for building the graph, but a
  different, more optimized one, for actually running

  // could also try using u32s here instead...not sure how that will affect things...
  enum OptimizedConnection {
  Eq(usize),
  Nand(usize, usize),
  }

  OptimizedPin {
  connection: Option<OptimizedConnection>
  value: Cell<bool>
  }

  type OptimizedFlipFlop = [usize, usize]

  let flipflops: Vec<OptimizedFlipFlop> = ...

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
- use macros to use rust as a nice HDL language?

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

https://gitlab.com/x653/nand2tetris-fpga/-/tree/master/
