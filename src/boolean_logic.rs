use crate::ordering::{get_all_connected_pins, sort_and_compute};
use crate::pin::{Pin, PinArray16};
use crate::test_utils::{bools_to_usize, i16_to_bools, u8_to_bools};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const TEST_NUMS: [i16; 6] = [0, 1, 1234, -1234, i16::MAX, i16::MIN];

// TODO - might be good to use macro for this to allow different number of
// inputs and automatically generating full set of inputs
fn exhaustively_test_two_in_one_out(gate: TwoInOneOutGate, f: fn(bool, bool) -> bool) {
    let pins = get_all_connected_pins(vec![gate.output.clone()]);
    let all_inputs = [[false, false], [false, true], [true, false], [true, true]];
    for input in all_inputs {
        gate.inputs[0].value.set(input[0]);
        gate.inputs[1].value.set(input[1]);
        let result = sort_and_compute(&[gate.output.clone()], &pins);
        assert_eq!(result[0], f(input[0], input[1]));
    }
}

fn exhaustively_test_three_in_one_out(gate: Mux, f: fn(bool, bool, bool) -> bool) {
    let all_pins = get_all_connected_pins(vec![gate.output.clone()]);
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
        let result = sort_and_compute(&[gate.output.clone()], &all_pins);
        assert_eq!(result[0], f(input[0], input[1], input[2]));
    }
}

fn exhaustively_test_two_in_two_out(gate: DMux, f: fn(bool, bool) -> [bool; 2]) {
    let all_pins = get_all_connected_pins([gate.output_a.clone(), gate.output_b.clone()].to_vec());
    let all_inputs = [[false, false], [false, true], [true, false], [true, true]];
    for input in all_inputs {
        gate.input.value.set(input[0]);
        gate.sel.value.set(input[1]);
        let result = sort_and_compute(&[gate.output_a.clone(), gate.output_b.clone()], &all_pins);
        let expected = f(input[0], input[1]);
        assert_eq!(result, expected);
    }
}

pub struct TwoInOneOutGate {
    pub inputs: [Rc<Pin>; 2],
    pub output: Rc<Pin>,
}

impl TwoInOneOutGate {
    fn base() -> Self {
        let output = Pin::new();
        Self {
            inputs: [Pin::new(), Pin::new()],
            output,
        }
    }

    pub fn nand() -> Self {
        let result = Self::base();
        result
            .output
            .nand_connect(result.inputs[0].clone(), result.inputs[1].clone());
        result
    }

    pub fn or() -> Self {
        // println!("start or");
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
        // println!("end or");

        result
    }
    pub fn and() -> Self {
        // println!("start and");
        let result = Self::base();
        let nand_gate = TwoInOneOutGate::nand();
        let not_gate = NotGate::new();
        result.output.feed_from(not_gate.output);
        not_gate.input.feed_from(nand_gate.output);
        nand_gate.inputs[0].feed_from(result.inputs[0].clone());
        nand_gate.inputs[1].feed_from(result.inputs[1].clone());
        // println!("end and");
        result
    }

    pub fn xor() -> Self {
        // println!("start xor");
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

        // println!("end xor");
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

pub struct NotGate {
    pub input: Rc<Pin>,
    pub output: Rc<Pin>,
}

impl NotGate {
    pub fn new() -> Self {
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
    let all_pins = get_all_connected_pins(vec![not_gate.output.clone()]);
    not_gate.input.value.set(true);
    let result = sort_and_compute(&[not_gate.output.clone()], &all_pins);
    assert_eq!(result[0], false);
    not_gate.input.value.set(false);
    let result = sort_and_compute(&[not_gate.output], &all_pins);
    assert_eq!(result[0], true);
}

#[derive(Default)]
pub struct Mux {
    pub input_a: Rc<Pin>,
    pub input_b: Rc<Pin>,
    pub sel: Rc<Pin>,
    pub output: Rc<Pin>,
}

impl Mux {
    pub fn new() -> Self {
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

pub struct Not16 {
    pub input: PinArray16,
    pub output: PinArray16,
}

impl Not16 {
    pub fn new() -> Self {
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
        let all_pins = get_all_connected_pins(not16.output.pins.to_vec());
        not16.input.set_values(i16_to_bools(num));
        let result = sort_and_compute(&not16.output.pins, &all_pins);
        assert_eq!(result, i16_to_bools(!num));
    }
}

pub struct TwoInOneOut16 {
    pub inputs: [PinArray16; 2],
    pub output: PinArray16,
}

impl TwoInOneOut16 {
    fn base() -> Self {
        let inputs = [PinArray16::new(), PinArray16::new()];
        let output = PinArray16::new();
        Self { inputs, output }
    }
    pub fn and16() -> Self {
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
            let all_pins = get_all_connected_pins(and16.output.pins.to_vec());
            let test_input_a = i16_to_bools(num_a);
            let test_input_b = i16_to_bools(num_b);
            and16.inputs[0].set_values(test_input_a);
            and16.inputs[1].set_values(test_input_b);
            let result = sort_and_compute(&and16.output.pins, &all_pins);
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
            let all_pins = get_all_connected_pins(or16.output.pins.to_vec());
            let test_input_a = i16_to_bools(num_a);
            let test_input_b = i16_to_bools(num_b);
            or16.inputs[0].set_values(test_input_a);
            or16.inputs[1].set_values(test_input_b);
            let result = sort_and_compute(&or16.output.pins, &all_pins);
            let expected = i16_to_bools(num_a | num_b);
            assert_eq!(result, expected);
        }
    }
}

pub struct Mux16 {
    pub inputs: [PinArray16; 2],
    pub sel: Rc<Pin>,
    pub output: PinArray16,
}

impl Mux16 {
    pub fn new() -> Self {
        let inputs = [PinArray16::new(), PinArray16::new()];
        let output = PinArray16::new();
        let sel = Pin::new();

        let result = Self {
            inputs,
            sel,
            output,
        };

        for i in 0..16 {
            let mux = Mux::new();
            mux.sel.feed_from(result.sel.clone());
            mux.input_a.feed_from(result.inputs[0].pins[i].clone());
            mux.input_b.feed_from(result.inputs[1].pins[i].clone());
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
                let all_pins = get_all_connected_pins(mux16.output.pins.to_vec());
                let test_input_a = i16_to_bools(num_a);
                let test_input_b = i16_to_bools(num_b);
                mux16.inputs[0].set_values(test_input_a);
                mux16.inputs[1].set_values(test_input_b);
                mux16.sel.value.set(sel);
                let result = sort_and_compute(&mux16.output.pins, &all_pins);
                let expected = if sel { test_input_b } else { test_input_a };
                assert_eq!(result, expected);
            }
        }
    }
}

pub struct Or8Way {
    pub input: [Rc<Pin>; 8],
    pub output: Rc<Pin>,
}

impl Or8Way {
    pub fn new() -> Self {
        let mut input: [Rc<Pin>; 8] = Default::default();
        for i in 0..8 {
            input[i] = Pin::new();
        }
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
    let all_pins = get_all_connected_pins(vec![or8way.output.clone()]);
    let test_bytes = [0, 1, 2, 123, u8::MAX];
    for num in test_bytes {
        let test_input = u8_to_bools(num);
        for i in 0..8 {
            or8way.input[i].value.set(test_input[i]);
        }
        let result = sort_and_compute(&[or8way.output.clone()], &all_pins);
        assert_eq!(result[0], num != 0);
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
    let all_pins = get_all_connected_pins(funnel.output.pins.to_vec());
    for pin_array in funnel.inputs.iter() {
        pin_array.set_values([false; 16]);
    }
    let result = sort_and_compute(&funnel.output.pins, &all_pins);
    assert_eq!(result, [false; 16]);
    funnel.inputs[5].set_values(i16_to_bools(123));
    let result = sort_and_compute(&funnel.output.pins, &all_pins);
    assert_eq!(result, i16_to_bools(123));
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

pub struct Mux4Way16 {
    pub inputs: [PinArray16; 4],
    pub sel: [Rc<Pin>; 2],
    pub output: PinArray16,
}

fn select_by_idx(idx: usize, sel: &[Rc<Pin>], and_inputs: &[Rc<Pin>]) {
    for j in 0..sel.len() {
        let sel_pin_bit = usize::pow(2, sel.len() as u32 - 1 - j as u32);
        let should_negate_sel_pin = idx & sel_pin_bit == 0;
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
    pub fn new() -> Self {
        let inputs: [PinArray16; 4] = [
            PinArray16::new(),
            PinArray16::new(),
            PinArray16::new(),
            PinArray16::new(),
        ];
        let sel: [Rc<Pin>; 2] = [Pin::new(), Pin::new()];
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
                select_by_idx(i, &result.sel, &and.inputs);
                mux.inputs[0].feed_from(constant_false.clone());
                mux.inputs[1].feed_from(result.inputs[i].clone());
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
            let all_pins = get_all_connected_pins(mux.output.pins.to_vec());
            mux.inputs[0].set_values(i16_to_bools(num_a));
            mux.inputs[1].set_values(i16_to_bools(num_b));
            mux.inputs[2].set_values(i16_to_bools(num_c));
            mux.inputs[3].set_values(i16_to_bools(num_d));
            for i in 0..=1 {
                mux.sel[i].value.set(sel[i]);
            }
            let result = sort_and_compute(&mux.output.pins, &all_pins);
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
            assert_eq!(result, expected);
        }
    }
}

pub struct Mux8Way16 {
    pub inputs: [PinArray16; 8],
    pub sel: [Rc<Pin>; 3],
    pub output: PinArray16,
}

impl Mux8Way16 {
    pub fn new() -> Self {
        let mut inputs: [PinArray16; 8] = Default::default();
        for i in 0..8 {
            inputs[i] = PinArray16::new();
        }
        let sel: [Rc<Pin>; 3] = [Pin::new(), Pin::new(), Pin::new()];
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
                select_by_idx(i, &result.sel, &and_inputs);
                mux.inputs[0].feed_from(constant_false.clone());
                mux.inputs[1].feed_from(result.inputs[i].clone());
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
            let all_pins = get_all_connected_pins(mux.output.pins.to_vec());
            for i in 0..8 {
                mux.inputs[i].set_values(i16_to_bools(test_case[i]));
            }
            for i in 0..=2 {
                mux.sel[i].value.set(sel[i]);
            }
            let result = sort_and_compute(&mux.output.pins, &all_pins);
            let expected = i16_to_bools(test_case[bools_to_usize(&sel)]);
            assert_eq!(result, expected);
        }
    }
}

pub struct DMux4Way {
    pub input: Rc<Pin>,
    pub sel: [Rc<Pin>; 2],
    pub outputs: [Rc<Pin>; 4],
}

impl DMux4Way {
    pub fn new() -> Self {
        let input = Pin::new();
        let sel: [Rc<Pin>; 2] = [Pin::new(), Pin::new()];
        let output: [Rc<Pin>; 4] = [Pin::new(), Pin::new(), Pin::new(), Pin::new()];
        let result = Self {
            input,
            sel,
            outputs: output,
        };

        for (idx, pin) in result.outputs.iter().enumerate() {
            let sel_and = TwoInOneOutGate::and();
            select_by_idx(idx, &result.sel, &sel_and.inputs);
            let and = TwoInOneOutGate::and();
            and.inputs[1].feed_from(sel_and.output);
            and.inputs[0].feed_from(result.input.clone());
            pin.feed_from(and.output);
        }

        result
    }
}

#[test]
fn test_dmux_4_way() {
    let dmux = DMux4Way::new();
    let all_pins = get_all_connected_pins(dmux.outputs.to_vec());
    for val in [false, true] {
        for sel_pin_idx in 0..4 {
            dmux.input.value.set(val);
            dmux.sel[0].value.set(sel_pin_idx & 2 == 2);
            dmux.sel[1].value.set(sel_pin_idx & 1 == 1);
            for (pin_idx, output) in dmux.outputs.iter().enumerate() {
                let result = sort_and_compute(&[output.clone()], &all_pins);
                assert_eq!(result[0], val && pin_idx == sel_pin_idx)
            }
        }
    }
}

pub struct DMux8Way {
    pub input: Rc<Pin>,
    pub sel: [Rc<Pin>; 3],
    pub outputs: [Rc<Pin>; 8],
}

impl DMux8Way {
    pub fn new() -> Self {
        let input = Pin::new();
        let sel: [Rc<Pin>; 3] = [Pin::new(), Pin::new(), Pin::new()];
        let mut outputs: [Rc<Pin>; 8] = Default::default();
        for i in 0..8 {
            outputs[i] = Pin::new();
        }
        let result = Self {
            input,
            sel,
            outputs,
        };

        for (idx, pin) in result.outputs.iter().enumerate() {
            let sel_and_a = TwoInOneOutGate::and();
            let sel_and_b = TwoInOneOutGate::and();
            sel_and_b.inputs[1].feed_from(sel_and_a.output);
            let sel_inputs = [
                sel_and_a.inputs[0].clone(),
                sel_and_a.inputs[1].clone(),
                sel_and_b.inputs[0].clone(),
            ];
            select_by_idx(idx, &result.sel, &sel_inputs);
            let and = TwoInOneOutGate::and();
            and.inputs[1].feed_from(sel_and_b.output);
            and.inputs[0].feed_from(result.input.clone());
            pin.feed_from(and.output);
        }

        result
    }
}

#[test]
fn test_dmux_8_way() {
    let dmux = DMux8Way::new();
    let all_pins = get_all_connected_pins(dmux.outputs.to_vec());
    for val in [false, true] {
        for sel_pin_idx in 0..8 {
            dmux.input.value.set(val);
            dmux.sel[0].value.set(sel_pin_idx & 4 == 4);
            dmux.sel[1].value.set(sel_pin_idx & 2 == 2);
            dmux.sel[2].value.set(sel_pin_idx & 1 == 1);
            for (pin_idx, output) in dmux.outputs.iter().enumerate() {
                let result = sort_and_compute(&[output.clone()], &all_pins);
                assert_eq!(result[0], val && pin_idx == sel_pin_idx)
            }
        }
    }
}
