use crate::pin::{Pin, PinArray16};
use crate::utils::{binaryi16, binaryu8};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const TEST_NUMS: [i16; 6] = [0, 1, 1234, -1234, i16::MAX, i16::MIN];

// TODO - might be good to use macro for this to allow different number of
// inputs and automatically generating full set of inputs
fn exhaustively_test_two_in_one_out(gate: TwoInOneOutGate, f: fn(bool, bool) -> bool) {
    let all_inputs = [[false, false], [false, true], [true, false], [true, true]];
    for input in all_inputs {
        gate.input_a.value.set(input[0]);
        gate.input_b.value.set(input[1]);
        gate.output.compute();
        assert_eq!(gate.output.value.get(), f(input[0], input[1]));
    }
}

fn exhaustively_test_three_in_one_out(gate: Mux, f: fn(bool, bool, bool) -> bool) {
    let all_inputs = [
        [false, false, false],
        [false, false, true],
        [false, true, false],
        [false, true, true],
        [true, false, false],
        [true, false, true],
        [true, true, false],
        [true, true, true],
    ];
    for input in all_inputs {
        gate.input_a.value.set(input[0]);
        gate.input_b.value.set(input[1]);
        gate.sel.value.set(input[2]);
        gate.output.compute();
        assert_eq!(gate.output.value.get(), f(input[0], input[1], input[2]));
    }
}

fn exhaustively_test_two_in_two_out(gate: DMux, f: fn(bool, bool) -> [bool; 2]) {
    let all_inputs = [[false, false], [false, true], [true, false], [true, true]];
    for input in all_inputs {
        gate.input.value.set(input[0]);
        gate.sel.value.set(input[1]);
        gate.output_a.compute();
        gate.output_b.compute();
        let expected = f(input[0], input[1]);
        assert_eq!(gate.output_a.value.get(), expected[0]);
        assert_eq!(gate.output_b.value.get(), expected[1]);
    }
}

struct TwoInOneOutGate {
    input_a: Rc<Pin>,
    input_b: Rc<Pin>,
    output: Rc<Pin>,
}

impl TwoInOneOutGate {
    fn base() -> Self {
        let output = Pin::new();
        let input_a = Pin::new();
        let input_b = Pin::new();
        Self {
            input_a,
            input_b,
            output,
        }
    }

    fn nand() -> Self {
        let result = Self::base();
        result
            .output
            .nand_connect(result.input_a.clone(), result.input_b.clone());
        result
    }

    fn or() -> Self {
        let result = Self::base();

        let nand_a = TwoInOneOutGate::nand();
        let nand_b = TwoInOneOutGate::nand();
        let nand_c = TwoInOneOutGate::nand();

        result.output.feed_from(nand_c.output);
        nand_c.input_a.feed_from(nand_a.output);
        nand_c.input_b.feed_from(nand_b.output);

        nand_a.input_a.feed_from(result.input_a.clone());
        nand_a.input_b.feed_from(result.input_a.clone());
        nand_b.input_a.feed_from(result.input_b.clone());
        nand_b.input_b.feed_from(result.input_b.clone());

        result
    }
    fn and() -> Self {
        let result = Self::base();
        let nand_gate = TwoInOneOutGate::nand();
        let not_gate = NotGate::new();
        result.output.feed_from(not_gate.output);
        not_gate.input.feed_from(nand_gate.output);
        nand_gate.input_a.feed_from(result.input_a.clone());
        nand_gate.input_b.feed_from(result.input_b.clone());
        result
    }

    fn xor() -> Self {
        let result = Self::base();

        let nand_a = Self::nand();
        let nand_b = Self::nand();
        let nand_c = Self::nand();
        let nand_d = Self::nand();

        result.output.feed_from(nand_d.output);
        nand_d.input_a.feed_from(nand_b.output);
        nand_d.input_b.feed_from(nand_c.output);

        nand_b.input_a.feed_from(result.input_a.clone());
        nand_b.input_b.feed_from(nand_a.output.clone());

        nand_c.input_b.feed_from(result.input_b.clone());
        nand_c.input_a.feed_from(nand_a.output);

        nand_a.input_a.feed_from(result.input_a.clone());
        nand_a.input_b.feed_from(result.input_b.clone());

        result
    }
}

#[test]
fn test_nand_gate() {
    exhaustively_test_two_in_one_out(TwoInOneOutGate::nand(), |a, b| !(a && b));
}

#[test]
fn test_and_gate() {
    exhaustively_test_two_in_one_out(TwoInOneOutGate::and(), |a, b| a && b);
}

#[test]
fn test_or() {
    exhaustively_test_two_in_one_out(TwoInOneOutGate::or(), |a, b| a || b);
}

#[test]
fn test_xor() {
    exhaustively_test_two_in_one_out(TwoInOneOutGate::xor(), |a, b| a ^ b);
}

struct NotGate {
    input: Rc<Pin>,
    output: Rc<Pin>,
}

impl NotGate {
    fn new() -> Self {
        let input = Pin::new();
        let output = Pin::new();
        let nand_gate = TwoInOneOutGate::nand();
        let result = Self { input, output };
        result.output.feed_from(nand_gate.output);
        nand_gate.input_a.feed_from(result.input.clone());
        nand_gate.input_b.feed_from(result.input.clone());
        result
    }
}

#[test]
fn test_not_gate() {
    let not_gate = NotGate::new();
    not_gate.input.value.set(true);
    not_gate.output.compute();
    assert_eq!(not_gate.output.value.get(), false);
    not_gate.input.value.set(false);
    not_gate.output.compute();
    assert_eq!(not_gate.output.value.get(), true);
}

#[derive(Default)]
struct Mux {
    input_a: Rc<Pin>,
    input_b: Rc<Pin>,
    sel: Rc<Pin>,
    output: Rc<Pin>,
}

impl Mux {
    fn new() -> Self {
        let input_a = Pin::new();
        let input_b = Pin::new();
        let sel = Pin::new();
        let output = Pin::new();
        let result = Self {
            input_a,
            input_b,
            sel,
            output,
        };

        let and_a = TwoInOneOutGate::and();
        let and_b = TwoInOneOutGate::and();
        let or = TwoInOneOutGate::or();
        let not = NotGate::new();

        result.output.feed_from(or.output);
        or.input_a.feed_from(and_a.output);
        or.input_b.feed_from(and_b.output);

        and_a.input_a.feed_from(result.input_a.clone());
        and_a.input_b.feed_from(not.output);
        not.input.feed_from(result.sel.clone());

        and_b.input_a.feed_from(result.input_b.clone());
        and_b.input_b.feed_from(result.sel.clone());

        result
    }
}

#[test]
fn test_mux() {
    let mux = Mux::new();
    eprintln!("{:#?}", mux.output);
    exhaustively_test_three_in_one_out(mux, |a, b, sel| if sel { b } else { a })
}

struct DMux {
    output_a: Rc<Pin>,
    output_b: Rc<Pin>,
    sel: Rc<Pin>,
    input: Rc<Pin>,
}

impl DMux {
    fn new() -> Self {
        let output_a = Pin::new();
        let output_b = Pin::new();
        let input = Pin::new();
        let sel = Pin::new();

        let result = Self {
            output_a,
            output_b,
            input,
            sel,
        };

        let not = NotGate::new();
        let and_a = TwoInOneOutGate::and();
        let and_b = TwoInOneOutGate::and();

        result.output_a.feed_from(and_a.output);
        and_a.input_a.feed_from(not.output);
        and_a.input_b.feed_from(result.input.clone());
        not.input.feed_from(result.sel.clone());

        result.output_b.feed_from(and_b.output);
        and_b.input_a.feed_from(result.sel.clone());
        and_b.input_b.feed_from(result.input.clone());

        result
    }
}

#[test]
fn test_dmux() {
    let dmux = DMux::new();
    exhaustively_test_two_in_two_out(
        dmux,
        |input, sel| if sel { [false, input] } else { [input, false] },
    )
}

struct Not16 {
    input: PinArray16,
    output: PinArray16,
}

impl Not16 {
    fn new() -> Self {
        let input = PinArray16::new();
        let output = PinArray16::new();
        let result = Self { input, output };
        for i in 0..16 {
            let not = NotGate::new();
            result.output.pins[i].feed_from(not.output);
            not.input.feed_from(result.input.pins[i].clone());
        }
        result
    }
}

#[test]
fn test_not16() {
    for num in TEST_NUMS {
        let not16 = Not16::new();
        not16.input.set_values(binaryi16(num));
        not16.output.compute();
        assert_eq!(
            not16.output.pins.map(|pin| pin.value.get()),
            binaryi16(!num)
        );
    }
}

struct TwoInOneOut16 {
    inputs: [PinArray16; 2],
    output: PinArray16,
}

impl TwoInOneOut16 {
    fn base() -> Self {
        let inputs = [PinArray16::new(), PinArray16::new()];
        let output = PinArray16::new();
        Self { inputs, output }
    }
    fn and16() -> Self {
        let result = Self::base();
        for i in 0..16 {
            let and = TwoInOneOutGate::and();
            result.output.pins[i].feed_from(and.output);
            and.input_a.feed_from(result.inputs[0].pins[i].clone());
            and.input_b.feed_from(result.inputs[1].pins[i].clone());
        }
        result
    }
    fn or16() -> Self {
        let result = Self::base();
        for i in 0..16 {
            let or = TwoInOneOutGate::or();
            result.output.pins[i].feed_from(or.output);
            or.input_a.feed_from(result.inputs[0].pins[i].clone());
            or.input_b.feed_from(result.inputs[1].pins[i].clone());
        }
        result
    }
}

#[test]
fn test_and16() {
    for num_a in TEST_NUMS {
        for num_b in TEST_NUMS {
            let and16 = TwoInOneOut16::and16();
            let test_input_a = binaryi16(num_a);
            let test_input_b = binaryi16(num_b);
            and16.inputs[0].set_values(test_input_a);
            and16.inputs[1].set_values(test_input_b);
            and16.output.compute();
            let result = and16.output.pins.map(|pin| pin.value.get());
            let expected = binaryi16(num_a & num_b);
            assert_eq!(result, expected);
        }
    }
}

#[test]
fn test_or16() {
    for num_a in TEST_NUMS {
        for num_b in TEST_NUMS {
            let or16 = TwoInOneOut16::or16();
            let test_input_a = binaryi16(num_a);
            let test_input_b = binaryi16(num_b);
            or16.inputs[0].set_values(test_input_a);
            or16.inputs[1].set_values(test_input_b);
            or16.output.compute();
            let result = or16.output.pins.map(|pin| pin.value.get());
            let expected = binaryi16(num_a | num_b);
            assert_eq!(result, expected);
        }
    }
}

struct Mux16 {
    input_a: PinArray16,
    input_b: PinArray16,
    sel: Rc<Pin>,
    output: PinArray16,
}

impl Mux16 {
    fn new() -> Self {
        let input_a = PinArray16::new();
        let input_b = PinArray16::new();
        let output = PinArray16::new();
        let sel = Pin::new();

        let result = Self {
            input_a,
            input_b,
            sel,
            output,
        };

        for i in 0..16 {
            let mux = Mux::new();
            mux.sel.feed_from(result.sel.clone());
            mux.input_a.feed_from(result.input_a.pins[i].clone());
            mux.input_b.feed_from(result.input_b.pins[i].clone());
            result.output.pins[i].feed_from(mux.output);
        }

        result
    }
}

#[test]
fn test_mux16() {
    for num_a in TEST_NUMS {
        for num_b in TEST_NUMS {
            for sel in [true, false] {
                let mux16 = Mux16::new();
                let test_input_a = binaryi16(num_a);
                let test_input_b = binaryi16(num_b);
                mux16.input_a.set_values(test_input_a);
                mux16.input_b.set_values(test_input_b);
                mux16.sel.value.set(sel);
                mux16.output.compute();
                let result = mux16.output.pins.map(|pin| pin.value.get());
                let expected = if sel { test_input_b } else { test_input_a };
                assert_eq!(result, expected);
            }
        }
    }
}

struct Or8Way {
    input: [Rc<Pin>; 8],
    output: Rc<Pin>,
}

impl Or8Way {
    fn new() -> Self {
        let input: [Rc<Pin>; 8] = Default::default();
        let output = Pin::new();
        let result = Self { input, output };

        let or_a = TwoInOneOutGate::or();
        let or_b = TwoInOneOutGate::or();
        let or_c = TwoInOneOutGate::or();
        let or_d = TwoInOneOutGate::or();
        let or_e = TwoInOneOutGate::or();
        let or_f = TwoInOneOutGate::or();
        let or_g = TwoInOneOutGate::or();

        result.output.feed_from(or_a.output);

        or_a.input_a.feed_from(or_b.output);
        or_a.input_b.feed_from(or_e.output);

        or_b.input_a.feed_from(or_c.output);
        or_b.input_b.feed_from(or_d.output);
        or_e.input_a.feed_from(or_f.output);
        or_e.input_b.feed_from(or_g.output);

        or_c.input_a.feed_from(result.input[0].clone());
        or_c.input_b.feed_from(result.input[1].clone());
        or_d.input_a.feed_from(result.input[2].clone());
        or_d.input_b.feed_from(result.input[3].clone());
        or_f.input_a.feed_from(result.input[4].clone());
        or_f.input_b.feed_from(result.input[5].clone());
        or_g.input_a.feed_from(result.input[6].clone());
        or_g.input_b.feed_from(result.input[7].clone());

        result
    }
}

#[test]
fn test_or8way() {
    let or8way = Or8Way::new();
    let test_bytes = [0, 1, 2, 123, u8::MAX];
    for num in test_bytes {
        let test_input = binaryu8(num);
        for i in 0..8 {
            or8way.input[i].value.set(test_input[i]);
        }
        or8way.output.compute();
        assert_eq!(or8way.output.value.get(), num != 0);
    }
}

struct Or4Way16 {
    inputs: [PinArray16; 4],
    output: PinArray16,
}

impl Or4Way16 {
    fn new() -> Self {
        struct Layer {
            inputs: Vec<PinArray16>,
            outputs: Vec<PinArray16>,
        }

        let layers = vec![];
        let mut or16s_per_layer = 2;

        let add_layer = || {
            let layer = Layer {
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            for i in 0..or16s_per_layer {
                let or16 = TwoInOneOut16::or16();
                layer.outputs.push(or16.output);
                layer.inputs.extend(or16.inputs);
            }
            layers.push(layer);
        };

        while or16s_per_layer > 0 {
            add_layer();
        }

        let bottom_layer = vec![TwoInOneOut16::or16(), TwoInOneOut16::or16()];
        let inputs: [PinArray16; 4] = Default::default();
        let output = PinArray16::new();
        let result = Self { inputs, output };
        let mut layer_idx = 0;
        let layer_outputs = vec![];
        let layer_inputs = vec![];
        while or16s_per_layer > 0 {
            let layer = vec![];
            for i in 0..or16s_per_layer {
                let or16 = TwoInOneOut16::or16();
                layer_inputs.extend(or16.inputs);
                if layer_idx == 0 {
                    result.inputs[i] = or16.inputs[0];
                }
            }
            or_16s.push(layer);
            layer_idx += 1;
        }
        result
    }
}

struct Mux4Way16 {
    inputs: [PinArray16; 4],
    sel: [Rc<Pin>; 2],
    output: PinArray16,
}

impl Mux4Way16 {
    fn new() -> Self {
        let inputs: [PinArray16; 4] = Default::default();
        let sel: [Rc<Pin>; 2] = Default::default();
        let output = PinArray16::new();
        let result = Self {
            inputs,
            sel,
            output,
        };

        let constant_false = PinArray16::new();

        let muxes: Vec<Mux16> = (0..4)
            .map(|i| {
                let mux = Mux16::new();
                let and = TwoInOneOutGate::and();
                if i & 2 == 0 {
                    let not = NotGate::new();
                    not.input.feed_from(result.sel[0].clone());
                    and.input_a.feed_from(not.output);
                } else {
                    and.input_a.feed_from(result.sel[0].clone());
                }
                if i & 1 == 0 {
                    let not = NotGate::new();
                    not.input.feed_from(result.sel[1].clone());
                    and.input_b.feed_from(not.output);
                } else {
                    and.input_b.feed_from(result.sel[1].clone());
                }
                mux.input_a.feed_from(constant_false.clone());
                mux.input_b.feed_from(result.inputs[i].clone());
                mux.sel.feed_from(and.output);
                mux
            })
            .collect();

        let top_or16 = TwoInOneOut16::or16();
        let bottom_or16s = vec![TwoInOneOut16::or16(), TwoInOneOut16::or16()];
        for (idx, mux) in muxes.into_iter().enumerate() {
            let or_idx = idx / 2;
            bottom_or16s[or_idx].inputs[idx & 1].feed_from(mux.output);
        }
        for (idx, bottom_or16) in bottom_or16s.into_iter().enumerate() {
            top_or16.inputs[idx].feed_from(bottom_or16.output);
        }
        result.output.feed_from(top_or16.output);
        result
    }
}

#[test]
fn test_mux4way16() {
    let test_cases = [
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [1, 2, 3, 4],
        [i16::MIN, i16::MAX, 123, 456],
    ];
    for [num_a, num_b, num_c, num_d] in test_cases {
        for sel in [[false, false], [false, true], [true, false], [true, true]] {
            let mux = Mux4Way16::new();
            mux.inputs[0].set_values(binaryi16(num_a));
            mux.inputs[1].set_values(binaryi16(num_b));
            mux.inputs[2].set_values(binaryi16(num_c));
            mux.inputs[3].set_values(binaryi16(num_d));
            for i in 0..=1 {
                mux.sel[i].value.set(sel[i]);
            }
            mux.output.compute();
            let result = mux.output.pins.map(|pin| pin.value.get());
            let expected = if sel[0] {
                if sel[1] {
                    binaryi16(num_d)
                } else {
                    binaryi16(num_c)
                }
            } else {
                if sel[1] {
                    binaryi16(num_b)
                } else {
                    binaryi16(num_a)
                }
            };
            assert_eq!(expected, result);
        }
    }
}

struct Mux8Way16 {
    inputs: [PinArray16; 8],
    sel: [Rc<Pin>; 3],
    output: PinArray16,
}

impl Mux8Way16 {
    fn new() -> Self {
        let inputs: [PinArray16; 8] = Default::default();
        let sel: [Rc<Pin>; 3] = Default::default();
        let output = PinArray16::new();
        let result = Self {
            inputs,
            sel,
            output,
        };

        let constant_false = PinArray16::new();

        let muxes: Vec<Mux16> = (0..8)
            .map(|i| {
                let mux = Mux16::new();
                let and_a = TwoInOneOutGate::and();
                let and_b = TwoInOneOutGate::and();
                and_a.input_a.feed_from(and_b.output);

                if i & 4 == 0 {
                    let not = NotGate::new();
                    not.input.feed_from(result.sel[0].clone());
                    and_b.input_a.feed_from(not.output);
                } else {
                    and_b.input_a.feed_from(result.sel[0].clone());
                }
                if i & 2 == 0 {
                    let not = NotGate::new();
                    not.input.feed_from(result.sel[1].clone());
                    and_b.input_b.feed_from(not.output);
                } else {
                    and_b.input_b.feed_from(result.sel[1].clone());
                }
                if i & 1 == 0 {
                    let not = NotGate::new();
                    not.input.feed_from(result.sel[2].clone());
                    and_b.input_b.feed_from(not.output);
                } else {
                    and_b.input_b.feed_from(result.sel[2].clone());
                }
                mux.input_a.feed_from(constant_false.clone());
                mux.input_b.feed_from(result.inputs[i].clone());
                mux.sel.feed_from(and_a.output);
                mux
            })
            .collect();

        let mux_count = muxes.len();
        if mux_count.count_ones() != 1 {
            panic!("number of muxes must be a power of 2");
        }
        let mut components_per_layer = mux_count;
        while components_per_layer > 0 {
            let layer = vec![];
            for i in 0..components_per_layer {
                layer.push(TwoInOneOut16::or16());
            }
            or_16s.push(layer);
        }

        let top_or16 = TwoInOneOut16::or16();
        let middle_or16s = vec![];
        let bottom_or16s = vec![];
        for (idx, mux) in muxes.into_iter().enumerate() {
            let middle_or_idx = idx / 2;
            bottom_or16s[middle_or_idx].inputs[idx & 1].feed_from(mux.output);
        }
        for (idx, bottom_or16) in bottom_or16s.into_iter().enumerate() {
            top_or16.inputs[idx].feed_from(bottom_or16.output);
        }
        result.output.feed_from(top_or16.output);

        result
    }
}

// fn mux8way16(
//     input_a: [bool; 16],
//     input_b: [bool; 16],
//     input_c: [bool; 16],
//     input_d: [bool; 16],
//     input_e: [bool; 16],
//     input_f: [bool; 16],
//     input_g: [bool; 16],
//     input_h: [bool; 16],
//     sel: [bool; 3],
// ) -> [bool; 16] {
//     or16a(
//         or16b(
//             or16c(
//                 mux16a(
//                     [false; 16],
//                     input_a,
//                     and(not(sel[0]), and(not(sel[1]), not(sel[2]))),
//                 ),
//                 mux16b(
//                     [false; 16],
//                     input_b,
//                     and(not(sel[0]), and(not(sel[1]), sel[2])),
//                 ),
//             ),
//             or16d(
//                 mux16c(
//                     [false; 16],
//                     input_c,
//                     and(not(sel[0]), and(sel[1], not(sel[2]))),
//                 ),
//                 mux16d([false; 16], input_d, and(not(sel[0]), and(sel[1], sel[2]))),
//             ),
//         ),
//         or16e(
//             or16f(
//                 mux16e(
//                     [false; 16],
//                     input_e,
//                     and(sel[0], and(not(sel[1]), not(sel[2]))),
//                 ),
//                 mux16f([false; 16], input_f, and(sel[0], and(not(sel[1]), sel[2]))),
//             ),
//             or16g(
//                 mux16g([false; 16], input_g, and(sel[0], and(sel[1], not(sel[2])))),
//                 mux16h([false; 16], input_h, and(sel[0], and(sel[1], sel[2]))),
//             ),
//         ),
//     )
// }

// fn dmux4way(input: bool, sel: [bool; 2]) -> [bool; 4] {
//     [
//         and(input, and(not(sel[0]), not(sel[1]))),
//         and(input, and(not(sel[0]), sel[1])),
//         and(input, and(sel[0], not(sel[1]))),
//         and(input, and(sel[0], sel[1])),
//     ]
// }

// fn dmux8way(input: bool, sel: [bool; 3]) -> [bool; 8] {
//     [
//         and(input, and(not(sel[0]), and(not(sel[1]), not(sel[2])))),
//         and(input, and(not(sel[0]), and(not(sel[1]), sel[2]))),
//         and(input, and(not(sel[0]), and(sel[1], not(sel[2])))),
//         and(input, and(not(sel[0]), and(sel[1], sel[2]))),
//         and(input, and(sel[0], and(not(sel[1]), not(sel[2])))),
//         and(input, and(sel[0], and(not(sel[1]), sel[2]))),
//         and(input, and(sel[0], and(sel[1], not(sel[2])))),
//         and(input, and(sel[0], and(sel[1], sel[2]))),
//     ]
// }

// #[cfg(test)]
// mod tests {
//     fn test_mux4way16() {
//         let a = [
//             true, false, true, false, false, true, false, true, false, false, false, true, false,
//             true, true, false,
//         ];
//         let b = [
//             false, false, true, false, true, true, false, false, true, false, true, true, false,
//             false, false, true,
//         ];
//         let c = [
//             false, false, true, true, false, false, true, true, false, true, true, false, false,
//             true, false, true,
//         ];
//         let d = [
//             true, true, false, false, true, true, false, true, false, false, false, true, true,
//             false, false, true,
//         ];
//         assert_eq!(mux4way16(a, b, c, d, [false, false]), a);
//         assert_eq!(mux4way16(a, b, c, d, [false, true]), b);
//         assert_eq!(mux4way16(a, b, c, d, [true, false]), c);
//         assert_eq!(mux4way16(a, b, c, d, [true, true]), d);
//     }

//     #[test]
//     fn test_mux8way16() {
//         let a = [
//             true, false, true, false, false, true, false, true, false, false, false, true, false,
//             true, true, false,
//         ];
//         let b = [
//             false, false, true, false, true, true, false, false, true, false, true, true, false,
//             false, false, true,
//         ];
//         let c = [
//             false, false, true, true, false, false, true, true, false, true, true, false, false,
//             true, false, true,
//         ];
//         let d = [
//             true, true, false, false, true, true, false, true, false, false, false, true, true,
//             false, false, true,
//         ];
//         let e = [
//             false, false, true, true, false, true, false, false, true, false, true, false, false,
//             true, false, false,
//         ];
//         let f = [
//             true, true, true, true, false, false, false, true, false, true, false, false, false,
//             true, false, true,
//         ];
//         let g = [
//             false, true, false, true, false, false, true, true, false, true, false, false, true,
//             false, true, true,
//         ];
//         let h = [
//             false, false, false, true, true, false, true, false, true, false, false, false, true,
//             true, false, true,
//         ];
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, false]), a);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, true]), b);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, false]), c);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, true]), d);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, false]), e);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, true]), f);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, false]), g);
//         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, true]), h);
//     }

//     #[test]
//     fn test_dmux4way() {
//         assert_eq!(
//             dmux4way(false, [false, false]),
//             [false, false, false, false]
//         );
//         assert_eq!(dmux4way(false, [false, true]), [false, false, false, false]);
//         assert_eq!(dmux4way(false, [true, false]), [false, false, false, false]);
//         assert_eq!(dmux4way(false, [true, true]), [false, false, false, false]);
//         assert_eq!(dmux4way(true, [false, false]), [true, false, false, false]);
//         assert_eq!(dmux4way(true, [false, true]), [false, true, false, false]);
//         assert_eq!(dmux4way(true, [true, false]), [false, false, true, false]);
//         assert_eq!(dmux4way(true, [true, true]), [false, false, false, true]);
//     }

//     #[test]
//     fn test_dmux8way() {
//         assert_eq!(
//             dmux8way(false, [false, false, false]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [false, false, true]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [false, true, false]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [false, true, true]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [true, false, false]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [true, false, true]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [true, true, false]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(false, [true, true, true]),
//             [false, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [false, false, false]),
//             [true, false, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [false, false, true]),
//             [false, true, false, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [false, true, false]),
//             [false, false, true, false, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [false, true, true]),
//             [false, false, false, true, false, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [true, false, false]),
//             [false, false, false, false, true, false, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [true, false, true]),
//             [false, false, false, false, false, true, false, false]
//         );
//         assert_eq!(
//             dmux8way(true, [true, true, false]),
//             [false, false, false, false, false, false, true, false]
//         );
//         assert_eq!(
//             dmux8way(true, [true, true, true]),
//             [false, false, false, false, false, false, false, true]
//         );
//     }
// }
