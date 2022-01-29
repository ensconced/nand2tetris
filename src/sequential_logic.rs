use crate::boolean_arithmetic::Add16;
use crate::boolean_logic::{DMux4Way, DMux8Way, Mux, Mux16, Mux4Way16, Mux8Way16, TwoInOneOutGate};
use crate::ordering::{get_all_connected_pins, sort_and_compute};
use crate::pin::{Pin, PinArray16};
use crate::test_utils::i16_to_bools;
use std::rc::Rc;

#[derive(Default)]
struct FlipFlop {
    input: Rc<Pin>,
    output: Rc<Pin>,
}

impl FlipFlop {
    fn new() -> Self {
        let input = Pin::new();
        let output = Pin::new();
        output.flipflop_connect(input.clone());
        Self { input, output }
    }

    fn tick(&self) {
        self.output.value.set(self.input.value.get());
    }
}

#[test]
fn test_flipflop() {
    let flipflop = FlipFlop::new();
    assert_eq!(flipflop.input.value.get(), false);
    assert_eq!(flipflop.output.value.get(), false);
    flipflop.input.value.set(true);
    assert_eq!(flipflop.output.value.get(), false);
    flipflop.tick();
    assert_eq!(flipflop.output.value.get(), true);
    flipflop.input.value.set(false);
    assert_eq!(flipflop.output.value.get(), true);
    flipflop.tick();
    assert_eq!(flipflop.output.value.get(), false);
}

#[test]
fn test_flip_flop_pair() {
    let flipflop_a = FlipFlop::new();
    let flipflop_b = FlipFlop::new();
    flipflop_b.input.feed_from(flipflop_a.output.clone());

    let all_pins = get_all_connected_pins(vec![flipflop_b.output.clone()]);

    flipflop_a.input.value.set(true);
    assert_eq!(flipflop_a.output.value.get(), false);
    assert_eq!(flipflop_b.output.value.get(), false);
    flipflop_a.tick();
    flipflop_b.tick();
    sort_and_compute(&[flipflop_b.output.clone()], &all_pins);
    assert_eq!(flipflop_a.output.value.get(), true);
    assert_eq!(flipflop_b.output.value.get(), false);
    flipflop_a.tick();
    flipflop_b.tick();
    sort_and_compute(&[flipflop_b.output.clone()], &all_pins);
    assert_eq!(flipflop_a.output.value.get(), true);
    assert_eq!(flipflop_b.output.value.get(), true);
}

#[test]
fn test_flipflop_chain() {
    let mut flip_flops: Vec<FlipFlop> = vec![];
    for i in 0..10 {
        let flip_flop = FlipFlop::new();
        if i > 0 {
            flip_flop.input.feed_from(flip_flops[i - 1].output.clone());
        }
        flip_flops.push(flip_flop);
    }

    let all_pins = get_all_connected_pins(vec![flip_flops[9].output.clone()]);

    flip_flops[0].input.value.set(true);

    let compute = || {
        for _ in 0..10 {
            for flip_flop in flip_flops.iter() {
                flip_flop.tick();
            }
        }
        sort_and_compute(&[flip_flops[9].output.clone()], &all_pins);
    };

    for step in 0..10 {
        for (flipflop_idx, flipflop) in flip_flops.iter().enumerate() {
            assert_eq!(flipflop.output.value.get(), flipflop_idx < step);
        }
        compute();
    }
}

#[derive(Default)]
struct BitRegister {
    input: Rc<Pin>,
    output: Rc<Pin>,
    load: Rc<Pin>,
    flipflop: FlipFlop,
}

impl BitRegister {
    fn new() -> Self {
        let result = Self {
            input: Pin::new(),
            output: Pin::new(),
            load: Pin::new(),
            flipflop: FlipFlop::new(),
        };

        result.output.feed_from(result.flipflop.output.clone());
        let mux = Mux::new();
        mux.input_a.feed_from(result.flipflop.output.clone());
        result.flipflop.input.feed_from(mux.output);
        mux.sel.feed_from(result.load.clone());
        mux.input_b.feed_from(result.input.clone());

        result
    }

    fn tick(&self) {
        self.flipflop.tick();
    }
}

#[test]
fn test_bit_register() {
    let bit = BitRegister::new();
    let all_pins = get_all_connected_pins(vec![bit.output.clone()]);

    // is properly initialised
    assert_eq!(bit.input.value.get(), false);
    assert_eq!(bit.output.value.get(), false);
    assert_eq!(bit.load.value.get(), false);

    // setting the input and ticking without setting the load bit shouldn't
    // change the output
    bit.input.value.set(true);
    bit.tick();
    let output_val = sort_and_compute(&[bit.output.clone()], &all_pins)[0];
    assert_eq!(output_val, false);

    // setting the load bit doesn't change the output value until you tick
    bit.load.value.set(true);
    let output_val = sort_and_compute(&[bit.output.clone()], &all_pins)[0];
    assert_eq!(output_val, false);

    // when you do tick, the output value does change...
    bit.tick();
    let output_val = sort_and_compute(&[bit.output], &all_pins)[0];
    assert_eq!(output_val, true);
}

#[derive(Default)]
struct Register {
    input: PinArray16,
    output: PinArray16,
    load: Rc<Pin>,
    bits: [BitRegister; 16],
}

impl Register {
    fn new() -> Self {
        let mut result = Self {
            input: PinArray16::new(),
            output: PinArray16::new(),
            load: Pin::new(),
            bits: Default::default(),
        };
        for i in 0..16 {
            let bit = BitRegister::new();
            bit.load.feed_from(result.load.clone());
            bit.input.feed_from(result.input.pins[i].clone());
            result.output.pins[i].feed_from(bit.output.clone());
            result.bits[i] = bit;
        }
        result
    }

    fn tick(&self) {
        for bit in self.bits.iter() {
            bit.tick();
        }
    }
}

#[test]
fn test_register() {
    let test_nums = [0, 1, 1234, i16::MIN, i16::MAX / 2, i16::MAX];
    let register = Register::new();
    let all_pins = get_all_connected_pins(register.output.pins.to_vec());

    for test_num in test_nums {
        let num_as_bools = i16_to_bools(test_num);
        register.load.value.set(true);
        register.input.set_values(num_as_bools);
        sort_and_compute(&register.output.pins, &all_pins);
        register.tick();
        let result = sort_and_compute(&register.output.pins, &all_pins);
        assert_eq!(result, num_as_bools);

        sort_and_compute(&register.output.pins, &all_pins);
        register.tick();
        let result = sort_and_compute(&register.output.pins, &all_pins);
        assert_eq!(result, num_as_bools);

        register.load.value.set(false);
        register.input.set_values([false; 16]);
        sort_and_compute(&register.output.pins, &all_pins);
        register.tick();
        let result = sort_and_compute(&register.output.pins, &all_pins);
        assert_eq!(result, num_as_bools);

        register.load.value.set(true);
        let result = sort_and_compute(&register.output.pins, &all_pins);
        assert_eq!(result, num_as_bools);

        sort_and_compute(&register.output.pins, &all_pins);
        register.tick();
        let result = sort_and_compute(&register.output.pins, &all_pins);
        assert_eq!(result, [false; 16]);
    }
}

#[derive(Default)]
struct Ram8 {
    // TODO - would be nice to be able to remove these
    registers: [Register; 8],
    input: PinArray16,
    output: PinArray16,
    address: [Rc<Pin>; 3],
    load: Rc<Pin>,
}

impl Ram8 {
    fn new() -> Self {
        let mut result = Self {
            registers: Default::default(),
            address: [Pin::new(), Pin::new(), Pin::new()],
            input: PinArray16::new(),
            output: PinArray16::new(),
            load: Pin::new(),
        };

        let mux = Mux8Way16::new();
        result.output.feed_from(mux.output);

        let dmux = DMux8Way::new();
        dmux.input.feed_from(result.load.clone());

        for i in 0..3 {
            let sel_pin = result.address[i].clone();
            mux.sel[i].feed_from(sel_pin.clone());
            dmux.sel[i].feed_from(sel_pin)
        }

        for i in 0..8 {
            let reg = Register::new();
            mux.inputs[i].feed_from(reg.output.clone());
            reg.load.feed_from(dmux.outputs[i].clone());
            reg.input.feed_from(result.input.clone());
            result.registers[i] = reg;
        }

        result
    }

    fn tick(&self) {
        for register in self.registers.iter() {
            register.tick();
        }
    }
}

#[test]
fn test_ram8() {
    let ram = Ram8::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());

    let val_a = i16_to_bools(1234);
    let addr_a = [false, false, true];
    // store value at address
    ram.load.value.set(true);
    ram.input.set_values(val_a);
    for i in 0..3 {
        ram.address[i].value.set(addr_a[i]);
    }
    sort_and_compute(&ram.output.pins, &all_pins);
    ram.tick();

    // retrieve value from output
    let output = sort_and_compute(&ram.output.pins, &all_pins);
    assert_eq!(output, val_a);

    // store another value at a different address
    let val_b = i16_to_bools(4567);
    let addr_b = [true, false, false];
    ram.input.set_values(val_b);
    for i in 0..3 {
        ram.address[i].value.set(addr_b[i]);
    }
    sort_and_compute(&ram.output.pins, &all_pins);
    ram.tick();
    let result = sort_and_compute(&ram.output.pins, &all_pins);
    assert_eq!(result, val_b);

    // check that original value is still present
    ram.load.value.set(false);
    for i in 0..3 {
        ram.address[i].value.set(addr_a[i]);
    }
    sort_and_compute(&ram.output.pins, &all_pins);
    ram.tick();
    let result = sort_and_compute(&ram.output.pins, &all_pins);
    assert_eq!(result, val_a);
}

#[derive(Default)]
struct Ram64 {
    // TODO - would be nice to be able to remove these
    ram8s: [Ram8; 8],
    input: PinArray16,
    output: PinArray16,
    address: [Rc<Pin>; 6],
    load: Rc<Pin>,
}

impl Ram64 {
    fn new() -> Self {
        let mut result = Self {
            ram8s: Default::default(),
            input: PinArray16::new(),
            output: PinArray16::new(),
            address: [
                Pin::new(),
                Pin::new(),
                Pin::new(),
                Pin::new(),
                Pin::new(),
                Pin::new(),
            ],
            load: Pin::new(),
        };

        let dmux = DMux8Way::new();
        dmux.input.feed_from(result.load.clone());

        let mux = Mux8Way16::new();
        result.output.feed_from(mux.output);

        for i in 0..3 {
            let sel_pin = result.address[i].clone();
            dmux.sel[i].feed_from(sel_pin.clone());
            mux.sel[i].feed_from(sel_pin)
        }

        for i in 0..8 {
            let ram = Ram8::new();
            ram.input.feed_from(result.input.clone());
            mux.inputs[i].feed_from(ram.output.clone());
            ram.load.feed_from(dmux.outputs[i].clone());
            for j in 0..3 {
                ram.address[j].feed_from(result.address[3 + j].clone());
            }
            result.ram8s[i] = ram;
        }

        result
    }

    fn tick(&self) {
        for ram8 in self.ram8s.iter() {
            ram8.tick();
        }
    }
}

#[test]
fn test_ram_64() {
    let ram = Ram64::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());

    let nums = [1234, 5678, -1234];
    // NB the first two addrs will be within the same Ram8
    let addrs = [
        [false, true, false, false, false, true],
        [false, true, false, false, true, false],
        [true, false, false, false, true, true],
    ];

    ram.load.value.set(true);
    // load in the three different values at different addresses
    for (num_idx, num) in nums.iter().enumerate() {
        ram.input.set_values(i16_to_bools(*num));
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
    }

    // now check all values are as expected
    ram.load.value.set(false);
    for (num_idx, num) in nums.into_iter().enumerate() {
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
        let result = sort_and_compute(&ram.output.pins, &all_pins);
        assert_eq!(result, i16_to_bools(num));
    }
}
#[derive(Default)]
pub struct Ram512 {
    // TODO - would be nice to be able to remove these
    ram64s: [Ram64; 8],
    pub input: PinArray16,
    pub output: PinArray16,
    pub address: [Rc<Pin>; 9],
    pub load: Rc<Pin>,
}

impl Ram512 {
    pub fn new() -> Self {
        let mut result = Self {
            ram64s: Default::default(),
            input: PinArray16::new(),
            output: PinArray16::new(),
            address: Default::default(),
            load: Pin::new(),
        };

        for i in 0..9 {
            result.address[i] = Pin::new();
        }

        let dmux = DMux8Way::new();
        dmux.input.feed_from(result.load.clone());

        let mux = Mux8Way16::new();
        result.output.feed_from(mux.output);

        for i in 0..3 {
            let sel_pin = result.address[i].clone();
            dmux.sel[i].feed_from(sel_pin.clone());
            mux.sel[i].feed_from(sel_pin)
        }

        for i in 0..8 {
            let ram = Ram64::new();
            ram.input.feed_from(result.input.clone());
            mux.inputs[i].feed_from(ram.output.clone());
            ram.load.feed_from(dmux.outputs[i].clone());
            for j in 0..6 {
                ram.address[j].feed_from(result.address[3 + j].clone());
            }
            result.ram64s[i] = ram;
        }

        result
    }

    pub fn tick(&self) {
        for ram64 in self.ram64s.iter() {
            ram64.tick();
        }
    }
}

#[test]
fn test_ram_512() {
    let ram = Ram512::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());

    let nums = [1234, 5678, -1234];
    // NB the first two addrs will be within the same Ram64
    let addrs = [
        [false, true, false, false, false, true, false, false, false],
        [false, true, false, false, false, true, false, false, true],
        [true, false, false, false, true, true, false, true, false],
    ];

    ram.load.value.set(true);
    // load in the three different values at different addresses
    for (num_idx, num) in nums.iter().enumerate() {
        ram.input.set_values(i16_to_bools(*num));
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
    }

    // now check all values are as expected
    ram.load.value.set(false);
    for (num_idx, num) in nums.into_iter().enumerate() {
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
        let result = sort_and_compute(&ram.output.pins, &all_pins);
        assert_eq!(result, i16_to_bools(num));
    }
}

#[derive(Default)]
pub struct Ram4k {
    // TODO - would be nice to be able to remove these
    ram512s: [Box<Ram512>; 8],
    pub input: PinArray16,
    pub output: PinArray16,
    pub address: [Rc<Pin>; 12],
    pub load: Rc<Pin>,
}

impl Ram4k {
    pub fn new() -> Self {
        let mut result = Self {
            ram512s: Default::default(),
            input: PinArray16::new(),
            output: PinArray16::new(),
            address: Default::default(),
            load: Pin::new(),
        };

        for i in 0..12 {
            result.address[i] = Pin::new();
        }

        let dmux = DMux8Way::new();
        dmux.input.feed_from(result.load.clone());

        let mux = Mux8Way16::new();
        result.output.feed_from(mux.output);

        for i in 0..3 {
            let sel_pin = result.address[i].clone();
            dmux.sel[i].feed_from(sel_pin.clone());
            mux.sel[i].feed_from(sel_pin)
        }

        for i in 0..8 {
            let ram = Ram512::new();
            ram.input.feed_from(result.input.clone());
            mux.inputs[i].feed_from(ram.output.clone());
            ram.load.feed_from(dmux.outputs[i].clone());
            for j in 0..9 {
                ram.address[j].feed_from(result.address[3 + j].clone());
            }
            result.ram512s[i] = Box::new(ram);
        }

        result
    }

    pub fn tick(&self) {
        for ram512 in self.ram512s.iter() {
            ram512.tick();
        }
    }
}

#[test]
fn test_ram_4k() {
    let ram = Ram4k::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());

    let nums = [1234, 5678, -1234];
    // NB the first two addrs will be within the same Ram512
    let addrs = [
        [
            false, true, false, false, false, true, false, false, false, false, false, true,
        ],
        [
            false, true, false, false, false, true, false, false, false, false, true, false,
        ],
        [
            true, false, false, false, true, true, false, true, false, true, false, false,
        ],
    ];

    ram.load.value.set(true);
    // load in the three different values at different addresses
    for (num_idx, num) in nums.iter().enumerate() {
        ram.input.set_values(i16_to_bools(*num));
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
    }

    // now check all values are as expected
    ram.load.value.set(false);
    for (num_idx, num) in nums.into_iter().enumerate() {
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
        let result = sort_and_compute(&ram.output.pins, &all_pins);
        assert_eq!(result, i16_to_bools(num));
    }
}

pub struct Ram16k {
    // TODO - would be nice to be able to remove these
    ram4ks: [Ram4k; 4],
    pub input: PinArray16,
    pub output: PinArray16,
    pub address: [Rc<Pin>; 14],
    pub load: Rc<Pin>,
}

impl Ram16k {
    pub fn new() -> Self {
        let mut result = Self {
            ram4ks: Default::default(),
            input: PinArray16::new(),
            output: PinArray16::new(),
            address: Default::default(),
            load: Pin::new(),
        };

        for i in 0..14 {
            result.address[i] = Pin::new();
        }

        let dmux = DMux4Way::new();
        dmux.input.feed_from(result.load.clone());

        let mux = Mux4Way16::new();
        result.output.feed_from(mux.output);

        for i in 0..2 {
            let sel_pin = result.address[i].clone();
            dmux.sel[i].feed_from(sel_pin.clone());
            mux.sel[i].feed_from(sel_pin)
        }

        for i in 0..4 {
            let ram = Ram4k::new();
            ram.input.feed_from(result.input.clone());
            mux.inputs[i].feed_from(ram.output.clone());
            ram.load.feed_from(dmux.outputs[i].clone());
            for j in 0..12 {
                ram.address[j].feed_from(result.address[2 + j].clone());
            }
            result.ram4ks[i] = ram;
        }

        result
    }

    pub fn tick(&self) {
        for ram4k in self.ram4ks.iter() {
            ram4k.tick();
        }
    }
}

#[test]
fn test_ram_16k() {
    let ram = Ram16k::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());
    let nums = [1234, 5678, -1234];
    // NB the first two addrs will be within the same Ram4k
    let addrs = [
        [
            false, true, false, false, false, true, false, false, false, false, false, true, false,
            false,
        ],
        [
            false, true, false, false, false, true, false, false, false, false, false, true, false,
            true,
        ],
        [
            true, false, false, false, true, true, false, true, false, true, false, false, true,
            false,
        ],
    ];

    ram.load.value.set(true);
    // load in the three different values at different addresses
    for (num_idx, num) in nums.iter().enumerate() {
        ram.input.set_values(i16_to_bools(*num));
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
    }

    // now check all values are as expected
    ram.load.value.set(false);
    for (num_idx, num) in nums.into_iter().enumerate() {
        let addr = addrs[num_idx];
        for (pin_idx, pin_val) in addr.iter().enumerate() {
            ram.address[pin_idx].value.set(*pin_val);
        }
        sort_and_compute(&ram.output.pins, &all_pins);
        ram.tick();
        let result = sort_and_compute(&ram.output.pins, &all_pins);
        assert_eq!(result, i16_to_bools(num));
    }
}

struct Counter {
    input: PinArray16,
    output: PinArray16,
    inc: Rc<Pin>,
    load: Rc<Pin>,
    reset: Rc<Pin>,
    register: Register,
}

impl Counter {
    fn new() -> Self {
        let result = Self {
            input: PinArray16::new(),
            output: PinArray16::new(),
            inc: Pin::new(),
            load: Pin::new(),
            reset: Pin::new(),
            register: Register::new(),
        };

        let zero = PinArray16::new();
        let one = PinArray16::new();
        one.pins[15].value.set(true);

        let adder = Add16::new();

        let or_a = TwoInOneOutGate::or();
        let or_b = TwoInOneOutGate::or();
        result.register.load.feed_from(or_a.output);
        or_a.inputs[0].feed_from(result.inc.clone());
        or_a.inputs[1].feed_from(or_b.output);
        or_b.inputs[0].feed_from(result.load.clone());
        or_b.inputs[1].feed_from(result.reset.clone());

        let mux_a = Mux16::new();
        let mux_b = Mux16::new();
        let mux_c = Mux16::new();
        result.register.input.feed_from(mux_a.output);
        mux_a.sel.feed_from(result.reset.clone());
        mux_a.inputs[1].feed_from(zero.clone());
        mux_a.inputs[0].feed_from(mux_b.output);
        mux_b.sel.feed_from(result.load.clone());
        mux_b.inputs[1].feed_from(result.input.clone());
        mux_b.inputs[0].feed_from(mux_c.output);
        mux_c.inputs[0].feed_from(zero);
        mux_c.inputs[1].feed_from(adder.output);
        mux_c.sel.feed_from(result.inc.clone());

        adder.inputs[0].feed_from(result.register.output.clone());
        adder.inputs[1].feed_from(one);

        result.output.feed_from(result.register.output.clone());

        result
    }

    fn tick(&self) {
        self.register.tick();
    }
}

#[test]
fn test_counter() {
    let counter = Counter::new();
    let all_pins = get_all_connected_pins(counter.output.pins.to_vec());

    counter.load.value.set(true);
    counter.input.set_values(i16_to_bools(47));
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(47)
    );
    counter.load.value.set(false);
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(47)
    );
    counter.reset.value.set(true);
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(0)
    );
    counter.reset.value.set(false);
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(0)
    );
    counter.inc.value.set(true);
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(1)
    );
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(2)
    );
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(3)
    );
    counter.inc.value.set(false);
    sort_and_compute(&counter.output.pins, &all_pins);
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(3)
    );
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(3)
    );
    counter.tick();
    assert_eq!(
        sort_and_compute(&counter.output.pins, &all_pins),
        i16_to_bools(3)
    );
}
