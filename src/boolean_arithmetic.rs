use crate::boolean_logic::{Mux16, Not16, NotGate, Or8Way, TwoInOneOut16, TwoInOneOutGate};
use crate::pin::{Pin, PinArray16};
use crate::utils::{bools_to_usize, i16_to_bools, last_2, last_3, u8_to_bools};
use std::iter;
use std::rc::Rc;

struct HalfAdder {
    inputs: [Rc<Pin>; 2],
    outputs: [Rc<Pin>; 2],
}

impl HalfAdder {
    fn new() -> Self {
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
        let mut result = [false; 2];
        for i in 0..=1 {
            half_adder.outputs[i].compute();
            result[i] = half_adder.outputs[i].value.get();
        }
        assert_eq!(result, expected_outputs);
    }
}

struct FullAdder {
    inputs: [Rc<Pin>; 3],
    outputs: [Rc<Pin>; 2],
}

impl FullAdder {
    fn new() -> Self {
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
        for i in 0..2 {
            full_adder.outputs[i].compute();
        }
        let outputs = full_adder.outputs.map(|pin| pin.value.get());

        let expected_output = last_2(u8_to_bools(i32::count_ones(i) as u8));
        assert_eq!(outputs, expected_output);
    }
}

// integer 2's complement addition - overflow is neither detected nor handled
#[derive(Debug)]
struct Add16 {
    inputs: [PinArray16; 2],
    output: PinArray16,
}

impl Add16 {
    fn new() -> Self {
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
    println!("here");
    for i in test_cases {
        for j in test_cases {
            let input_a = i16_to_bools(i);
            let input_b = i16_to_bools(j);
            add16.inputs[0].set_values(input_a);
            add16.inputs[1].set_values(input_b);
            let mut result = [false; 16];
            for (pin_idx, pin) in add16.output.pins.iter().enumerate() {
                pin.compute();
                result[pin_idx] = pin.value.get();
            }
            assert_eq!(result, i16_to_bools(i + j));
        }
    }
}

// fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
//     let adder1 = half_adder(a[15], b[15]);
//     let adder2 = full_adder(a[14], b[14], adder1[0]);
//     let adder3 = full_adder(a[13], b[13], adder2[0]);
//     let adder4 = full_adder(a[12], b[12], adder3[0]);
//     let adder5 = full_adder(a[11], b[11], adder4[0]);
//     let adder6 = full_adder(a[10], b[10], adder5[0]);
//     let adder7 = full_adder(a[9], b[9], adder6[0]);
//     let adder8 = full_adder(a[8], b[8], adder7[0]);
//     let adder9 = full_adder(a[7], b[7], adder8[0]);
//     let adder10 = full_adder(a[6], b[6], adder9[0]);
//     let adder11 = full_adder(a[5], b[5], adder10[0]);
//     let adder12 = full_adder(a[4], b[4], adder11[0]);
//     let adder13 = full_adder(a[3], b[3], adder12[0]);
//     let adder14 = full_adder(a[2], b[2], adder13[0]);
//     let adder15 = full_adder(a[1], b[1], adder14[0]);
//     let adder16 = full_adder(a[0], b[0], adder15[0]);
//     [
//         adder16[1], adder15[1], adder14[1], adder13[1], adder12[1], adder11[1], adder10[1],
//         adder9[1], adder8[1], adder7[1], adder6[1], adder5[1], adder4[1], adder3[1], adder2[1],
//         adder1[1],
//     ]
// }

// fn inc16(input: [bool; 16]) -> [bool; 16] {
//     let mut one = [false; 16];
//     one[15] = true;
//     add16(input, one)
// }

// fn is_non_zero(a: [bool; 16]) -> bool {
//     or(
//         or8way([a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]]),
//         or8way([a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15]]),
//     )
// }

// struct ALUOutput {
//     out: [bool; 16],
//     zr: bool,
//     ng: bool,
// }

// fn alu(
//     x: [bool; 16],
//     y: [bool; 16],
//     zx: bool,
//     nx: bool,
//     zy: bool,
//     ny: bool,
//     f: bool,
//     no: bool,
// ) -> ALUOutput {
//     let stage1x = mux16(x, [false; 16], zx);
//     let stage1y = mux16(y, [false; 16], zy);
//     let stage2x = mux16(stage1x, not16(stage1x), nx);
//     let stage2y = mux16(stage1y, not16(stage1y), ny);
//     let anded = and16(stage2x, stage2y);
//     let added = add16(stage2x, stage2y);
//     let fed = mux16(anded, added, f);
//     let out = mux16(fed, not16(fed), no);
//     let zr = not(is_non_zero(out));
//     let ng = out[0];
//     ALUOutput { out, zr, ng }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::binary;

//     #[test]
//     fn test_full_adder() {
//         assert_eq!(full_adder(false, false, false), [false, false]);
//         assert_eq!(full_adder(false, false, true), [false, true]);
//         assert_eq!(full_adder(false, true, false), [false, true]);
//         assert_eq!(full_adder(false, true, true), [true, false]);
//         assert_eq!(full_adder(true, false, false), [false, true]);
//         assert_eq!(full_adder(true, false, true), [true, false]);
//         assert_eq!(full_adder(true, true, false), [true, false]);
//         assert_eq!(full_adder(true, true, true), [true, true]);
//     }

//     #[test]
//     fn test_add16() {
//         assert_eq!(add16(binary(0), binary(0)), binary(0));
//         assert_eq!(add16(binary(0), binary(1)), binary(1));
//         assert_eq!(add16(binary(1), binary(0)), binary(1));
//         assert_eq!(add16(binary(1), binary(-1)), binary(0));
//         assert_eq!(add16(binary(123), binary(-123)), binary(0));
//         assert_eq!(add16(binary(1000), binary(1000)), binary(2000));
//     }

//     #[test]
//     fn test_inc16() {
//         fn test(num: i16) {
//             let wrapped_num = std::num::Wrapping(num);
//             let correct_result = (wrapped_num + std::num::Wrapping(1)).0;
//             assert_eq!(inc16(binary(wrapped_num.0)), binary(correct_result));
//         }
//         test(0);
//         test(123);
//         test(i16::MAX);
//     }

//     #[test]
//     fn test_is_non_zero() {
//         fn test(num: i16) {
//             assert_eq!(is_non_zero(binary(num)), num != 0);
//         }
//         test(-1);
//         test(123);
//         test(-123);
//         test(0);
//     }

//     #[test]
//     fn test_alu_zero() {
//         fn test(x: i16, y: i16) {
//             let result = alu(binary(x), binary(y), true, false, true, false, true, false);
//             assert_eq!(result.out, binary(0))
//         }
//         test(0, 0);
//         test(1, 0);
//         test(0, 1);
//         test(123, 1234);
//         test(-123, -1234);
//     }

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
