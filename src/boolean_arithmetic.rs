use crate::boolean_logic::{Mux16, Not16, NotGate, Or8Way, TwoInOneOut16, TwoInOneOutGate};
use crate::ordering::compute_all;
use crate::pin::{Pin, PinArray16};
use crate::test_utils::{bools_to_usize, i16_to_bools, last_2, last_3, u8_to_bools};
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
    for test_case in test_cases {
        let [inputs, expected_outputs] = test_case;
        for i in 0..=1 {
            half_adder.inputs[i].value.set(inputs[i]);
        }
        let result = compute_all(&half_adder.outputs);
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
        let inputs = last_3(u8_to_bools(i as u8));
        for i in 0..3 {
            full_adder.inputs[i].value.set(inputs[i]);
        }
        let result = compute_all(&full_adder.outputs);
        let expected_output = last_2(u8_to_bools(i32::count_ones(i) as u8));
        assert_eq!(result[0..2], expected_output);
    }
}

#[derive(Debug)]
pub struct Add16 {
    inputs: [PinArray16; 2],
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
    for i in test_cases {
        for j in test_cases {
            let input_a = i16_to_bools(i);
            let input_b = i16_to_bools(j);
            add16.inputs[0].set_values(input_a);
            add16.inputs[1].set_values(input_b);
            let result = compute_all(&add16.output.pins);
            let expected_num = (std::num::Wrapping(i) + std::num::Wrapping(j)).0;
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
    for i in test_cases {
        for _ in test_cases {
            inc16.input.set_values(i16_to_bools(i));
            let result = compute_all(&inc16.output.pins);
            let expected_num = (std::num::Wrapping(i) + std::num::Wrapping(1)).0;
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
    for i in test_cases {
        is_non_zero.input.set_values(i16_to_bools(i));
        let result = compute_all(&[is_non_zero.output.clone()]);
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

    fn compute(&self) {
        let mut all_output_pins = self.output.pins.to_vec();
        all_output_pins.push(self.output_is_zero.clone());
        all_output_pins.push(self.output_is_negative.clone());
        compute_all(&all_output_pins);
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
        f: fn(i16, i16) -> i16,
    ) {
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
                alu.compute();
                let expected_result = f(test_num_a, test_num_b);
                assert_eq!(alu.output.get_values(), i16_to_bools(expected_result));
                assert_eq!(alu.output_is_zero.value.get(), expected_result == 0);
                assert_eq!(alu.output_is_negative.value.get(), expected_result < 0);
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
            |_, _| 0,
        )
    }

    #[test]
    fn test_alu_one() {
        test_alu(ALU::new(), true, true, [true, true], [true, true], |_, _| 1)
    }
}

//     #[test]
//     fn test_alu_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, true, true, true, true, true);
//             assert_eq!(result.out, binary(1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_minus_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, false, true, false, false, true);
//             assert_eq!(result.out, binary(-1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, false, true, false, true, false);
//             assert_eq!(result.out, binary(x))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, false, false, false, true, false);
//             assert_eq!(result.out, binary(y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_not_x() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, true, true, false, true, false);
//             assert_eq!(result.out, not16(binary(x)))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_not_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, false, false, true, true, false);
//             assert_eq!(result.out, not16(binary(y)))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_minus_x() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, false, true, true, true, true);
//             assert_eq!(result.out, binary(-x))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_minus_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, true, false, false, true, true);
//             assert_eq!(result.out, binary(-y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_plus_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, true, true, true, true, true);
//             assert_eq!(result.out, binary(x + 1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_y_plus_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, true, false, true, true, true);
//             assert_eq!(result.out, binary(y + 1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_minus_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, false, true, true, true, false);
//             assert_eq!(result.out, binary(x - 1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_y_minus_one() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, true, false, false, true, false);
//             assert_eq!(result.out, binary(y - 1))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_plus_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(
//                 binary(x),
//                 binary(y),
//                 false,
//                 false,
//                 false,
//                 false,
//                 true,
//                 false,
//             );
//             assert_eq!(result.out, binary(x + y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_minus_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, true, false, false, true, true);
//             assert_eq!(result.out, binary(x - y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_y_minus_x() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, false, false, true, true, true);
//             assert_eq!(result.out, binary(y - x))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_and_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(
//                 binary(x),
//                 binary(y),
//                 false,
//                 false,
//                 false,
//                 false,
//                 false,
//                 false,
//             );
//             assert_eq!(result.out, binary(x & y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

//     #[test]
//     fn test_alu_x_or_y() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), false, true, false, true, false, true);
//             assert_eq!(result.out, binary(x | y))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }
// }
