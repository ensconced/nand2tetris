use crate::pin::{Pin, PinArray16};
use crate::utils::{bools_to_usize, i16_to_bools, u8_to_bools};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const TEST_NUMS: [i16; 6] = [0, 1, 1234, -1234, i16::MAX, i16::MIN];

// TODO - might be good to use macro for this to allow different number of
// inputs and automatically generating full set of inputs
fn exhaustively_test_two_in_one_out(gate: TwoInOneOutGate, f: fn(bool, bool) -> bool) {
    let all_inputs = [[false, false], [false, true], [true, false], [true, true]];
    for input in all_inputs {
        gate.inputs[0].value.set(input[0]);
        gate.inputs[1].value.set(input[1]);
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
    inputs: [Rc<Pin>; 2],
    output: Rc<Pin>,
}

impl TwoInOneOutGate {
    fn base() -> Self {
        let output = Pin::new();
        Self {
            inputs: [Pin::new(), Pin::new()],
            output,
        }
    }

    fn nand() -> Self {
        let result = Self::base();
        result
            .output
            .nand_connect(result.inputs[0].clone(), result.inputs[1].clone());
        result
    }

    fn or() -> Self {
        let result = Self::base();

        let nand_a = TwoInOneOutGate::nand();
        let nand_b = TwoInOneOutGate::nand();
        let nand_c = TwoInOneOutGate::nand();

        result.output.feed_from(nand_c.output);
        nand_c.inputs[0].feed_from(nand_a.output);
        nand_c.inputs[1].feed_from(nand_b.output);

        nand_a.inputs[0].feed_from(result.inputs[0].clone());
        nand_a.inputs[1].feed_from(result.inputs[0].clone());
        nand_b.inputs[0].feed_from(result.inputs[1].clone());
        nand_b.inputs[1].feed_from(result.inputs[1].clone());

        result
    }
    fn and() -> Self {
        let result = Self::base();
        let nand_gate = TwoInOneOutGate::nand();
        let not_gate = NotGate::new();
        result.output.feed_from(not_gate.output);
        not_gate.input.feed_from(nand_gate.output);
        nand_gate.inputs[0].feed_from(result.inputs[0].clone());
        nand_gate.inputs[1].feed_from(result.inputs[1].clone());
        result
    }

    fn xor() -> Self {
        let result = Self::base();

        let nand_a = Self::nand();
        let nand_b = Self::nand();
        let nand_c = Self::nand();
        let nand_d = Self::nand();

        result.output.feed_from(nand_d.output);
        nand_d.inputs[0].feed_from(nand_b.output);
        nand_d.inputs[1].feed_from(nand_c.output);

        nand_b.inputs[0].feed_from(result.inputs[0].clone());
        nand_b.inputs[1].feed_from(nand_a.output.clone());

        nand_c.inputs[1].feed_from(result.inputs[1].clone());
        nand_c.inputs[0].feed_from(nand_a.output);

        nand_a.inputs[0].feed_from(result.inputs[0].clone());
        nand_a.inputs[1].feed_from(result.inputs[1].clone());

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
        nand_gate.inputs[0].feed_from(result.input.clone());
        nand_gate.inputs[1].feed_from(result.input.clone());
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
        or.inputs[0].feed_from(and_a.output);
        or.inputs[1].feed_from(and_b.output);

        and_a.inputs[0].feed_from(result.input_a.clone());
        and_a.inputs[1].feed_from(not.output);
        not.input.feed_from(result.sel.clone());

        and_b.inputs[0].feed_from(result.input_b.clone());
        and_b.inputs[1].feed_from(result.sel.clone());

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
        and_a.inputs[0].feed_from(not.output);
        and_a.inputs[1].feed_from(result.input.clone());
        not.input.feed_from(result.sel.clone());

        result.output_b.feed_from(and_b.output);
        and_b.inputs[0].feed_from(result.sel.clone());
        and_b.inputs[1].feed_from(result.input.clone());

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
        not16.input.set_values(i16_to_bools(num));
        not16.output.compute();
        assert_eq!(
            not16.output.pins.map(|pin| pin.value.get()),
            i16_to_bools(!num)
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
            and.inputs[0].feed_from(result.inputs[0].pins[i].clone());
            and.inputs[1].feed_from(result.inputs[1].pins[i].clone());
        }
        result
    }
    fn or16() -> Self {
        let result = Self::base();
        for i in 0..16 {
            let or = TwoInOneOutGate::or();
            result.output.pins[i].feed_from(or.output);
            or.inputs[0].feed_from(result.inputs[0].pins[i].clone());
            or.inputs[1].feed_from(result.inputs[1].pins[i].clone());
        }
        result
    }
}

#[test]
fn test_and16() {
    for num_a in TEST_NUMS {
        for num_b in TEST_NUMS {
            let and16 = TwoInOneOut16::and16();
            let test_input_a = i16_to_bools(num_a);
            let test_input_b = i16_to_bools(num_b);
            and16.inputs[0].set_values(test_input_a);
            and16.inputs[1].set_values(test_input_b);
            and16.output.compute();
            let result = and16.output.pins.map(|pin| pin.value.get());
            let expected = i16_to_bools(num_a & num_b);
            assert_eq!(result, expected);
        }
    }
}

#[test]
fn test_or16() {
    for num_a in TEST_NUMS {
        for num_b in TEST_NUMS {
            let or16 = TwoInOneOut16::or16();
            let test_input_a = i16_to_bools(num_a);
            let test_input_b = i16_to_bools(num_b);
            or16.inputs[0].set_values(test_input_a);
            or16.inputs[1].set_values(test_input_b);
            or16.output.compute();
            let result = or16.output.pins.map(|pin| pin.value.get());
            let expected = i16_to_bools(num_a | num_b);
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
                let test_input_a = i16_to_bools(num_a);
                let test_input_b = i16_to_bools(num_b);
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

        or_a.inputs[0].feed_from(or_b.output);
        or_a.inputs[1].feed_from(or_e.output);

        or_b.inputs[0].feed_from(or_c.output);
        or_b.inputs[1].feed_from(or_d.output);
        or_e.inputs[0].feed_from(or_f.output);
        or_e.inputs[1].feed_from(or_g.output);

        or_c.inputs[0].feed_from(result.input[0].clone());
        or_c.inputs[1].feed_from(result.input[1].clone());
        or_d.inputs[0].feed_from(result.input[2].clone());
        or_d.inputs[1].feed_from(result.input[3].clone());
        or_f.inputs[0].feed_from(result.input[4].clone());
        or_f.inputs[1].feed_from(result.input[5].clone());
        or_g.inputs[0].feed_from(result.input[6].clone());
        or_g.inputs[1].feed_from(result.input[7].clone());

        result
    }
}

#[test]
fn test_or8way() {
    let or8way = Or8Way::new();
    let test_bytes = [0, 1, 2, 123, u8::MAX];
    for num in test_bytes {
        let test_input = u8_to_bools(num);
        for i in 0..8 {
            or8way.input[i].value.set(test_input[i]);
        }
        or8way.output.compute();
        assert_eq!(or8way.output.value.get(), num != 0);
    }
}

struct OrFunnel {
    inputs: Vec<PinArray16>,
    output: PinArray16,
}

impl OrFunnel {
    fn new(bottom_layer_or_count: usize) -> Self {
        if bottom_layer_or_count.count_ones() != 1 {
            panic!("bottom layer or count must be a power of 2");
        }
        let mut inputs = vec![];
        for _ in 0..bottom_layer_or_count * 2 {
            inputs.push(PinArray16::new());
        }
        let output = PinArray16::new();
        let result = Self { inputs, output };
        let mut or16s_in_current_layer = bottom_layer_or_count;
        let mut inputs_for_current_layer: Vec<PinArray16> =
            result.inputs.iter().map(|pin| pin.clone()).collect();
        let mut layer = vec![];
        while or16s_in_current_layer > 0 {
            layer = vec![];
            for _ in 0..or16s_in_current_layer {
                layer.push(TwoInOneOut16::or16());
            }
            for or in layer.iter() {
                for input in or.inputs.iter() {
                    input.feed_from(inputs_for_current_layer.remove(0));
                }
            }
            inputs_for_current_layer = layer.iter().map(|or| or.output.clone()).collect();
            or16s_in_current_layer /= 2;
        }
        result.output.feed_from(layer[0].output.clone());
        result
    }
}

#[test]
fn test_or_n_way_16() {
    let funnel = OrFunnel::new(32);
    for pin_array in funnel.inputs.iter() {
        pin_array.set_values([false; 16]);
    }
    funnel.output.compute();
    assert_eq!(
        funnel.output.clone().pins.map(|pin| pin.value.get()),
        [false; 16]
    );
    funnel.inputs[5].set_values(i16_to_bools(123));
    funnel.output.compute();
    assert_eq!(
        funnel.output.pins.map(|pin| pin.value.get()),
        i16_to_bools(123)
    );
}

struct OrFunnel4Way16 {
    inputs: [PinArray16; 4],
    output: PinArray16,
}

impl OrFunnel4Way16 {
    fn new() -> Self {
        let funnel = OrFunnel::new(2);
        Self {
            inputs: funnel.inputs.try_into().unwrap(),
            output: funnel.output,
        }
    }
}

struct OrFunnel8Way16 {
    inputs: [PinArray16; 8],
    output: PinArray16,
}

impl OrFunnel8Way16 {
    fn new() -> Self {
        let or_n_way = OrFunnel::new(4);
        Self {
            inputs: or_n_way.inputs.try_into().unwrap(),
            output: or_n_way.output,
        }
    }
}

struct Mux4Way16 {
    inputs: [PinArray16; 4],
    sel: [Rc<Pin>; 2],
    output: PinArray16,
}

fn connect_mux_to_sel(mux_idx: usize, sel: &[Rc<Pin>], and_inputs: &[Rc<Pin>]) {
    for j in 0..sel.len() {
        let sel_pin_bit = usize::pow(2, sel.len() as u32 - 1 - j as u32);
        let should_negate_sel_pin = mux_idx & sel_pin_bit == 0;
        if should_negate_sel_pin {
            let not = NotGate::new();
            not.input.feed_from(sel[j].clone());
            and_inputs[j].feed_from(not.output);
        } else {
            and_inputs[j].feed_from(sel[j].clone());
        }
    }
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
                connect_mux_to_sel(i, &result.sel, &and.inputs);
                mux.input_a.feed_from(constant_false.clone());
                mux.input_b.feed_from(result.inputs[i].clone());
                mux.sel.feed_from(and.output);
                mux
            })
            .collect();

        let or4way16 = OrFunnel4Way16::new();
        for (idx, mux) in muxes.iter().enumerate() {
            or4way16.inputs[idx].feed_from(mux.output.clone());
        }
        result.output.feed_from(or4way16.output);
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
            mux.inputs[0].set_values(i16_to_bools(num_a));
            mux.inputs[1].set_values(i16_to_bools(num_b));
            mux.inputs[2].set_values(i16_to_bools(num_c));
            mux.inputs[3].set_values(i16_to_bools(num_d));
            for i in 0..=1 {
                mux.sel[i].value.set(sel[i]);
            }
            mux.output.compute();
            let result = mux.output.pins.map(|pin| pin.value.get());
            let expected = if sel[0] {
                if sel[1] {
                    i16_to_bools(num_d)
                } else {
                    i16_to_bools(num_c)
                }
            } else {
                if sel[1] {
                    i16_to_bools(num_b)
                } else {
                    i16_to_bools(num_a)
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
                and_a.inputs[0].feed_from(and_b.output);
                let and_inputs = [
                    and_a.inputs[1].clone(),
                    and_b.inputs[0].clone(),
                    and_b.inputs[1].clone(),
                ];
                connect_mux_to_sel(i, &result.sel, &and_inputs);
                mux.input_a.feed_from(constant_false.clone());
                mux.input_b.feed_from(result.inputs[i].clone());
                mux.sel.feed_from(and_a.output);
                mux
            })
            .collect();

        let or_8_way = OrFunnel8Way16::new();
        for (idx, mux) in muxes.into_iter().enumerate() {
            or_8_way.inputs[idx].feed_from(mux.output);
        }

        result.output.feed_from(or_8_way.output);
        result
    }
}

#[test]
fn test_mux8way16() {
    let test_cases = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 2, 3, 4, 5, 6, 7, 8],
        [i16::MIN, i16::MAX, 123, 456, 1234, -9999, 1, -1],
    ];
    for test_case in test_cases {
        for sel in [
            [false, false, false],
            [false, true, false],
            [true, false, false],
            [true, true, false],
        ] {
            let mux = Mux8Way16::new();
            for i in 0..8 {
                mux.inputs[i].set_values(i16_to_bools(test_case[i]));
            }
            for i in 0..=2 {
                mux.sel[i].value.set(sel[i]);
            }
            mux.output.compute();
            let result = mux.output.pins.map(|pin| pin.value.get());
            let expected = i16_to_bools(test_case[bools_to_usize(&sel)]);
            assert_eq!(expected, result);
        }
    }
}

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
