use crate::boolean_logic::{Mux16, Not16, NotGate, Or8Way, TwoInOneOut16, TwoInOneOutGate};
use crate::ordering::{all_connected_pins, compute_all};
use crate::pin::{Pin, PinArray16};
use crate::test_utils::{bools_to_usize, i16_to_bools, last_2, last_3, u8_to_bools};
use std::num::Wrapping;
use std::rc::Rc;

struct HalfAdder {
    inputs: [Rc<Pin>; 2],
    outputs: [Rc<Pin>; 2],
}

impl HalfAdder {
    fn new() -> Self {
        // println!("start halfadder");
        let inputs: [Rc<Pin>; 2] = [Pin::new(), Pin::new()];
        let outputs: [Rc<Pin>; 2] = [Pin::new(), Pin::new()];
        let result = Self { inputs, outputs };

        let and = TwoInOneOutGate::and();
        let xor = TwoInOneOutGate::xor();

        result.outputs[0].feed_from(and.output);
        result.outputs[1].feed_from(xor.output);
        and.inputs[0].feed_from(result.inputs[0].clone());
        and.inputs[1].feed_from(result.inputs[1].clone());
        xor.inputs[0].feed_from(result.inputs[0].clone());
        xor.inputs[1].feed_from(result.inputs[1].clone());

        // println!("end halfadder");
        result
    }
}

#[test]
fn test_half_adder() {
    let test_cases = [
        [[false, false], [false, false]],
        [[false, true], [false, true]],
        [[true, false], [false, true]],
        [[true, true], [true, false]],
    ];

    let half_adder = HalfAdder::new();
    let pins = all_connected_pins(half_adder.outputs.to_vec());
    for test_case in test_cases {
        let [inputs, expected_outputs] = test_case;
        for i in 0..=1 {
            half_adder.inputs[i].value.set(inputs[i]);
        }
        let result = compute_all(&half_adder.outputs, &pins);
        assert_eq!(result[0..2], expected_outputs);
    }
}

pub struct FullAdder {
    inputs: [Rc<Pin>; 3],
    pub outputs: [Rc<Pin>; 2],
}

impl FullAdder {
    pub fn new() -> Self {
        // println!("start fulladder");
        let inputs: [Rc<Pin>; 3] = [Pin::new(), Pin::new(), Pin::new()];
        let outputs: [Rc<Pin>; 2] = [Pin::new(), Pin::new()];
        let result = Self { inputs, outputs };

        let half_adder_a = HalfAdder::new();
        let half_adder_b = HalfAdder::new();

        half_adder_a.inputs[0].feed_from(result.inputs[0].clone());
        half_adder_a.inputs[1].feed_from(result.inputs[1].clone());

        half_adder_b.inputs[0].feed_from(half_adder_a.outputs[1].clone());
        half_adder_b.inputs[1].feed_from(result.inputs[2].clone());

        let or = TwoInOneOutGate::or();
        or.inputs[0].feed_from(half_adder_a.outputs[0].clone());
        or.inputs[1].feed_from(half_adder_b.outputs[0].clone());

        result.outputs[0].feed_from(or.output);
        result.outputs[1].feed_from(half_adder_b.outputs[1].clone());
        // println!("end fulladder");

        result
    }
}

#[test]
fn test_full_adder() {
    for i in 0..8 {
        let full_adder = FullAdder::new();
        let pins = all_connected_pins(full_adder.outputs.to_vec());
        let inputs = last_3(u8_to_bools(i as u8));
        for i in 0..3 {
            full_adder.inputs[i].value.set(inputs[i]);
        }
        let result = compute_all(&full_adder.outputs, &pins);
        let expected_output = last_2(u8_to_bools(i32::count_ones(i) as u8));
        assert_eq!(result[0..2], expected_output);
    }
}

#[derive(Debug)]
pub struct Add16 {
    pub inputs: [PinArray16; 2],
    pub output: PinArray16,
}

impl Add16 {
    pub fn new() -> Self {
        let inputs = [PinArray16::new(), PinArray16::new()];
        let output = PinArray16::new();
        let result = Self { inputs, output };

        let first_adder = HalfAdder::new();
        first_adder.inputs[0].feed_from(result.inputs[0].pins[15].clone());
        first_adder.inputs[1].feed_from(result.inputs[1].pins[15].clone());
        result.output.pins[15].feed_from(first_adder.outputs[1].clone());
        let mut carry = first_adder.outputs[0].clone();
        for i in (0..15).rev() {
            let adder = FullAdder::new();
            adder.inputs[0].feed_from(result.inputs[0].pins[i].clone());
            adder.inputs[1].feed_from(result.inputs[1].pins[i].clone());
            adder.inputs[2].feed_from(carry);
            result.output.pins[i].feed_from(adder.outputs[1].clone());
            carry = adder.outputs[0].clone();
        }

        result
    }
}

#[test]
fn test_add16() {
    let test_cases = [0, 1, 1234, -1234, i16::MAX, i16::MIN];
    let add16 = Add16::new();
    let pins = all_connected_pins(add16.output.pins.to_vec());
    for i in test_cases {
        for j in test_cases {
            let input_a = i16_to_bools(i);
            let input_b = i16_to_bools(j);
            add16.inputs[0].set_values(input_a);
            add16.inputs[1].set_values(input_b);
            let result = compute_all(&add16.output.pins, &pins);
            let expected_num = (Wrapping(i) + Wrapping(j)).0;
            assert_eq!(result[0..16], i16_to_bools(expected_num));
        }
    }
}

struct Inc16 {
    input: PinArray16,
    output: PinArray16,
}

impl Inc16 {
    fn new() -> Self {
        let input = PinArray16::new();
        let output = PinArray16::new();
        let result = Self { input, output };

        let one = PinArray16::new();
        one.pins[15].value.set(true);
        let add = Add16::new();
        result.output.feed_from(add.output);
        add.inputs[0].feed_from(one);
        add.inputs[1].feed_from(result.input.clone());

        result
    }
}

#[test]
fn test_inc16() {
    let test_cases = [0, 1, 1234, -1234, i16::MAX, i16::MIN];
    let inc16 = Inc16::new();
    let pins = all_connected_pins(inc16.output.pins.to_vec());
    for i in test_cases {
        for _ in test_cases {
            inc16.input.set_values(i16_to_bools(i));
            let result = compute_all(&inc16.output.pins, &pins);
            let expected_num = (Wrapping(i) + Wrapping(1)).0;
            assert_eq!(result[0..16], i16_to_bools(expected_num));
        }
    }
}

struct IsNonZero16 {
    pub output: Rc<Pin>,
    pub input: PinArray16,
}

impl IsNonZero16 {
    pub fn new() -> Self {
        let output = Pin::new();
        let input = PinArray16::new();
        let result = Self { input, output };

        let or8way_a = Or8Way::new();
        let or8way_b = Or8Way::new();
        let or = TwoInOneOutGate::or();

        or.inputs[0].feed_from(or8way_a.output);
        or.inputs[1].feed_from(or8way_b.output);
        for i in 0..8 {
            or8way_a.input[i].feed_from(result.input.pins[i].clone());
            or8way_b.input[i].feed_from(result.input.pins[i + 8].clone());
        }
        result.output.feed_from(or.output);

        result
    }
}

#[test]
fn test_is_non_zero() {
    let test_cases = [0, 1, 1234, -1234, i16::MAX, i16::MIN];
    let is_non_zero = IsNonZero16::new();
    let pins = all_connected_pins(vec![is_non_zero.output.clone()]);
    for i in test_cases {
        is_non_zero.input.set_values(i16_to_bools(i));
        let output_pins = [is_non_zero.output.clone()];
        let result = compute_all(&output_pins, &pins);
        let expected = i != 0;
        assert_eq!(result[0], expected);
    }
}

struct ALU {
    inputs: [PinArray16; 2],
    output: PinArray16,
    zero_inputs: [Rc<Pin>; 2],
    not_inputs: [Rc<Pin>; 2],
    use_add: Rc<Pin>,
    not_output: Rc<Pin>,
    output_is_zero: Rc<Pin>,
    output_is_negative: Rc<Pin>,
}

impl ALU {
    fn new() -> Self {
        let result = Self {
            inputs: [PinArray16::new(), PinArray16::new()],
            output: PinArray16::new(),
            zero_inputs: [Pin::new(), Pin::new()],
            not_inputs: [Pin::new(), Pin::new()],
            use_add: Pin::new(),
            not_output: Pin::new(),
            output_is_zero: Pin::new(),
            output_is_negative: Pin::new(),
        };

        let constant_false = PinArray16::new();
        let and = TwoInOneOut16::and16();
        let add = Add16::new();

        for i in 0..=1 {
            let zeroing_mux = Mux16::new();
            zeroing_mux.inputs[0].feed_from(result.inputs[i].clone());
            zeroing_mux.inputs[1].feed_from(constant_false.clone());
            zeroing_mux.sel.feed_from(result.zero_inputs[i].clone());

            let not = Not16::new();
            let not_input_mux = Mux16::new();
            not.input.feed_from(zeroing_mux.output.clone());

            not_input_mux.inputs[0].feed_from(zeroing_mux.output);
            not_input_mux.inputs[1].feed_from(not.output);
            not_input_mux.sel.feed_from(result.not_inputs[i].clone());

            // TODO - could potentially be faster to dmux here to either go to
            // AND or ADD, instead of always going to both?
            and.inputs[i].feed_from(not_input_mux.output.clone());
            add.inputs[i].feed_from(not_input_mux.output.clone());
        }
        let sel_function_mux = Mux16::new();
        sel_function_mux.inputs[0].feed_from(and.output);
        sel_function_mux.inputs[1].feed_from(add.output);
        sel_function_mux.sel.feed_from(result.use_add.clone());

        let not = Not16::new();
        not.input.feed_from(sel_function_mux.output.clone());

        let not_output_mux = Mux16::new();
        not_output_mux.inputs[0].feed_from(sel_function_mux.output);
        not_output_mux.inputs[1].feed_from(not.output);
        not_output_mux.sel.feed_from(result.not_output.clone());

        result.output.feed_from(not_output_mux.output.clone());
        result
            .output_is_negative
            .feed_from(not_output_mux.output.pins[0].clone());

        let is_non_zero = IsNonZero16::new();
        is_non_zero.input.feed_from(not_output_mux.output);
        let not = NotGate::new();
        not.input.feed_from(is_non_zero.output);
        result.output_is_zero.feed_from(not.output);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const test_nums: [i16; 7] = [0, 1, 123, 1234, -123, i16::MAX, i16::MIN];

    fn test_alu(
        alu: ALU,
        use_add: bool,
        not_output: bool,
        zero_inputs: [bool; 2],
        not_inputs: [bool; 2],
        f: fn(Wrapping<i16>, Wrapping<i16>) -> Wrapping<i16>,
    ) {
        let mut output_pins = alu.output.pins.to_vec();
        output_pins.push(alu.output_is_zero.clone());
        output_pins.push(alu.output_is_negative.clone());
        let all_pins = all_connected_pins(output_pins.to_vec());

        alu.use_add.value.set(use_add);
        alu.not_output.value.set(not_output);
        for test_num_a in test_nums {
            alu.inputs[0].set_values(i16_to_bools(test_num_a));
            for test_num_b in test_nums {
                alu.inputs[1].set_values(i16_to_bools(test_num_b));
                for i in 0..=1 {
                    alu.zero_inputs[i].value.set(zero_inputs[i]);
                    alu.not_inputs[i].value.set(not_inputs[i]);
                }

                compute_all(&output_pins, &all_pins);
                let expected_result = f(Wrapping(test_num_a), Wrapping(test_num_b));
                assert_eq!(alu.output.get_values(), i16_to_bools(expected_result.0));
                assert_eq!(alu.output_is_zero.value.get(), expected_result.0 == 0);
                assert_eq!(alu.output_is_negative.value.get(), expected_result.0 < 0);
            }
        }
    }

    #[test]
    fn test_alu_zero() {
        test_alu(
            ALU::new(),
            true,
            false,
            [true, true],
            [false, false],
            |_, _| Wrapping(0),
        )
    }

    #[test]
    fn test_alu_one() {
        test_alu(
            ALU::new(),
            true,
            true,
            [true, true],
            [true, true],
            |_, _| Wrapping(1),
        )
    }

    #[test]
    fn test_alu_minus_one() {
        test_alu(
            ALU::new(),
            true,
            false,
            [true, true],
            [true, false],
            |_, _| Wrapping(-1),
        )
    }

    #[test]
    fn test_alu_x() {
        test_alu(
            ALU::new(),
            true,
            false,
            [false, true],
            [false, false],
            |x, _| x,
        )
    }

    #[test]
    fn test_alu_y() {
        test_alu(
            ALU::new(),
            true,
            false,
            [true, false],
            [false, false],
            |_, y| y,
        )
    }

    #[test]
    fn test_alu_not_x() {
        test_alu(
            ALU::new(),
            true,
            false,
            [false, true],
            [true, false],
            |x, _| !x,
        )
    }

    #[test]
    fn test_alu_not_y() {
        test_alu(
            ALU::new(),
            true,
            false,
            [true, false],
            [false, true],
            |_, y| !y,
        )
    }

    #[test]
    fn test_alu_minus_x() {
        test_alu(
            ALU::new(),
            true,
            true,
            [false, true],
            [false, true],
            |x, _| -x,
        )
    }

    #[test]
    fn test_alu_minus_y() {
        test_alu(
            ALU::new(),
            true,
            true,
            [true, false],
            [true, false],
            |_, y| -y,
        )
    }

    #[test]
    fn test_alu_x_plus_one() {
        test_alu(
            ALU::new(),
            true,
            true,
            [false, true],
            [true, true],
            |x, _| x + Wrapping(1),
        )
    }

    #[test]
    fn test_alu_y_plus_one() {
        test_alu(
            ALU::new(),
            true,
            true,
            [true, false],
            [true, true],
            |_, y| y + Wrapping(1),
        )
    }

    #[test]
    fn test_alu_x_minus_one() {
        test_alu(
            ALU::new(),
            true,
            false,
            [false, true],
            [false, true],
            |x, _| x - Wrapping(1),
        )
    }

    #[test]
    fn test_alu_y_minus_one() {
        test_alu(
            ALU::new(),
            true,
            false,
            [true, false],
            [true, false],
            |_, y| y - Wrapping(1),
        )
    }

    #[test]
    fn test_alu_x_plus_y() {
        test_alu(
            ALU::new(),
            true,
            false,
            [false, false],
            [false, false],
            |x, y| x + y,
        )
    }

    #[test]
    fn test_alu_x_minus_y() {
        test_alu(
            ALU::new(),
            true,
            true,
            [false, false],
            [true, false],
            |x, y| x - y,
        )
    }

    #[test]
    fn test_alu_y_minus_x() {
        test_alu(
            ALU::new(),
            true,
            true,
            [false, false],
            [false, true],
            |x, y| y - x,
        )
    }

    #[test]
    fn test_alu_x_and_y() {
        test_alu(
            ALU::new(),
            false,
            false,
            [false, false],
            [false, false],
            |x, y| x & y,
        )
    }

    #[test]
    fn test_alu_x_or_y() {
        // Think De Morgan's.
        test_alu(
            ALU::new(),
            false,
            true,
            [false, false],
            [true, true],
            |x, y| x | y,
        )
    }
}
