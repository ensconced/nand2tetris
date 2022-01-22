use crate::utils::{binaryi16, binaryu8};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

static mut PIN_COUNT: i32 = 0;

const test_nums: [i16; 6] = [0, 1, 1234, -1234, i16::MAX, i16::MIN];

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

#[derive(Debug)]
enum Connection {
    Eq(Rc<Pin>),
    Nand(Rc<Pin>, Rc<Pin>),
}

#[derive(Debug, Default)]
struct Pin {
    debug_id: i32,
    value: Cell<bool>,
    connection: RefCell<Option<Connection>>,
}

impl Pin {
    fn new() -> Rc<Self> {
        unsafe {
            PIN_COUNT += 1;
            Rc::new(Self {
                debug_id: PIN_COUNT,
                value: Cell::new(false),
                connection: RefCell::new(None),
            })
        }
    }
    fn connect(&self, pin: Rc<Pin>) {
        let mut connection = self.connection.borrow_mut();
        if connection.as_ref().is_some() {
            panic!("pin is already connected");
        }
        connection.replace(Connection::Eq(pin));
    }
    fn nand_connect(&self, input_a: Rc<Pin>, input_b: Rc<Pin>) {
        self.connection
            .borrow_mut()
            .replace(Connection::Nand(input_a, input_b));
    }
    fn compute(&self) {
        // TODO - this is an inefficient "pull" system - would be better
        // to do a toposort and then "push".
        let new_value = match self.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => {
                pin.compute();
                pin.value.get()
            }
            Some(Connection::Nand(pin_a, pin_b)) => {
                pin_a.compute();
                pin_b.compute();
                !(pin_a.value.get() && pin_b.value.get())
            }
            None => self.value.get(),
        };
        self.value.set(new_value);
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

        result.output.connect(nand_c.output);
        nand_c.input_a.connect(nand_a.output);
        nand_c.input_b.connect(nand_b.output);

        nand_a.input_a.connect(result.input_a.clone());
        nand_a.input_b.connect(result.input_a.clone());
        nand_b.input_a.connect(result.input_b.clone());
        nand_b.input_b.connect(result.input_b.clone());

        result
    }
    fn and() -> Self {
        let result = Self::base();
        let nand_gate = TwoInOneOutGate::nand();
        let not_gate = NotGate::new();
        result.output.connect(not_gate.output);
        not_gate.input.connect(nand_gate.output);
        nand_gate.input_a.connect(result.input_a.clone());
        nand_gate.input_b.connect(result.input_b.clone());
        result
    }

    fn xor() -> Self {
        let result = Self::base();

        let nand_a = Self::nand();
        let nand_b = Self::nand();
        let nand_c = Self::nand();
        let nand_d = Self::nand();

        result.output.connect(nand_d.output);
        nand_d.input_a.connect(nand_b.output);
        nand_d.input_b.connect(nand_c.output);

        nand_b.input_a.connect(result.input_a.clone());
        nand_b.input_b.connect(nand_a.output.clone());

        nand_c.input_b.connect(result.input_b.clone());
        nand_c.input_a.connect(nand_a.output);

        nand_a.input_a.connect(result.input_a.clone());
        nand_a.input_b.connect(result.input_b.clone());

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
        result.output.connect(nand_gate.output);
        nand_gate.input_a.connect(result.input.clone());
        nand_gate.input_b.connect(result.input.clone());
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

        result.output.connect(or.output);
        or.input_a.connect(and_a.output);
        or.input_b.connect(and_b.output);

        and_a.input_a.connect(result.input_a.clone());
        and_a.input_b.connect(not.output);
        not.input.connect(result.sel.clone());

        and_b.input_a.connect(result.input_b.clone());
        and_b.input_b.connect(result.sel.clone());

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

        result.output_a.connect(and_a.output);
        and_a.input_a.connect(not.output);
        and_a.input_b.connect(result.input.clone());
        not.input.connect(result.sel.clone());

        result.output_b.connect(and_b.output);
        and_b.input_a.connect(result.sel.clone());
        and_b.input_b.connect(result.input.clone());

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
    input: [Rc<Pin>; 16],
    output: [Rc<Pin>; 16],
}

impl Not16 {
    fn new() -> Self {
        let input: [Rc<Pin>; 16] = Default::default();
        let output: [Rc<Pin>; 16] = Default::default();
        let result = Self { input, output };
        for i in 0..16 {
            let not = NotGate::new();
            result.output[i].connect(not.output);
            not.input.connect(result.input[i].clone());
        }
        result
    }
}

#[test]
fn test_not16() {
    let not16 = Not16::new();
    for num in test_nums {
        let test_input = binaryi16(num);
        for i in 0..16 {
            not16.input[i].value.set(test_input[i]);
        }
        let mut result = [false; 16];
        for i in 0..16 {
            let output_pin = &not16.output[i];
            output_pin.compute();
            result[i] = output_pin.value.get();
        }
        let expected = binaryi16(!num);
        assert_eq!(result, expected);
    }
}

struct TwoInOneOut16 {
    input_a: [Rc<Pin>; 16],
    input_b: [Rc<Pin>; 16],
    output: [Rc<Pin>; 16],
}

impl TwoInOneOut16 {
    fn base() -> Self {
        let input_a: [Rc<Pin>; 16] = Default::default();
        let input_b: [Rc<Pin>; 16] = Default::default();
        let output: [Rc<Pin>; 16] = Default::default();
        Self {
            input_a,
            input_b,
            output,
        }
    }
    fn and16() -> Self {
        let result = Self::base();
        for i in 0..16 {
            let and = TwoInOneOutGate::and();
            result.output[i].connect(and.output);
            and.input_a.connect(result.input_a[i].clone());
            and.input_b.connect(result.input_b[i].clone());
        }
        result
    }
    fn or16() -> Self {
        let result = Self::base();
        for i in 0..16 {
            let or = TwoInOneOutGate::or();
            result.output[i].connect(or.output);
            or.input_a.connect(result.input_a[i].clone());
            or.input_b.connect(result.input_b[i].clone());
        }
        result
    }
}

#[test]
fn test_and16() {
    let and16 = TwoInOneOut16::and16();
    for num_a in test_nums {
        for num_b in test_nums {
            let test_input_a = binaryi16(num_a);
            let test_input_b = binaryi16(num_b);
            for i in 0..16 {
                and16.input_a[i].value.set(test_input_a[i]);
                and16.input_b[i].value.set(test_input_b[i]);
            }
            let mut result = [false; 16];
            for i in 0..16 {
                let output_pin = &and16.output[i];
                output_pin.compute();
                result[i] = output_pin.value.get();
            }
            let expected = binaryi16(num_a & num_b);
            assert_eq!(result, expected);
        }
    }
}

#[test]
fn test_or16() {
    let or16 = TwoInOneOut16::or16();
    for num_a in test_nums {
        for num_b in test_nums {
            let test_input_a = binaryi16(num_a);
            let test_input_b = binaryi16(num_b);
            for i in 0..16 {
                or16.input_a[i].value.set(test_input_a[i]);
                or16.input_b[i].value.set(test_input_b[i]);
            }
            let mut result = [false; 16];
            for i in 0..16 {
                let output_pin = &or16.output[i];
                output_pin.compute();
                result[i] = output_pin.value.get();
            }
            let expected = binaryi16(num_a | num_b);
            assert_eq!(result, expected);
        }
    }
}

struct Mux16 {
    input_a: [Rc<Pin>; 16],
    input_b: [Rc<Pin>; 16],
    sel: Rc<Pin>,
    output: [Rc<Pin>; 16],
}

impl Mux16 {
    fn new() -> Self {
        let input_a: [Rc<Pin>; 16] = Default::default();
        let input_b: [Rc<Pin>; 16] = Default::default();
        let output: [Rc<Pin>; 16] = Default::default();
        let sel = Pin::new();

        let result = Self {
            input_a,
            input_b,
            sel,
            output,
        };

        for i in 0..16 {
            let mux = Mux::new();
            mux.sel.connect(result.sel.clone());
            mux.input_a.connect(result.input_a[i].clone());
            mux.input_b.connect(result.input_b[i].clone());
            result.output[i].connect(mux.output);
        }

        result
    }
}

#[test]
fn test_mux16() {
    let mux16 = Mux16::new();
    for num_a in test_nums {
        for num_b in test_nums {
            for sel in [true, false] {
                let test_input_a = binaryi16(num_a);
                let test_input_b = binaryi16(num_b);
                for i in 0..16 {
                    mux16.input_a[i].value.set(test_input_a[i]);
                    mux16.input_b[i].value.set(test_input_b[i]);
                }
                mux16.sel.value.set(sel);
                let mut result = [false; 16];
                for i in 0..16 {
                    let output_pin = &mux16.output[i];
                    output_pin.compute();
                    result[i] = output_pin.value.get();
                }
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

        result.output.connect(or_a.output);

        or_a.input_a.connect(or_b.output);
        or_a.input_b.connect(or_e.output);

        or_b.input_a.connect(or_c.output);
        or_b.input_b.connect(or_d.output);
        or_e.input_a.connect(or_f.output);
        or_e.input_b.connect(or_g.output);

        or_c.input_a.connect(result.input[0].clone());
        or_c.input_b.connect(result.input[1].clone());
        or_d.input_a.connect(result.input[2].clone());
        or_d.input_b.connect(result.input[3].clone());
        or_f.input_a.connect(result.input[4].clone());
        or_f.input_b.connect(result.input[5].clone());
        or_g.input_a.connect(result.input[6].clone());
        or_g.input_b.connect(result.input[7].clone());

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

struct Mux4Way16 {
    input_a: [Rc<Pin>; 16],
    input_b: [Rc<Pin>; 16],
    input_c: [Rc<Pin>; 16],
    input_d: [Rc<Pin>; 16],
    sel: [Rc<Pin>; 2],
    output: [Rc<Pin>; 16],
}

impl Mux4Way16 {
    fn new() -> Self {
        let input_a: [Rc<Pin>; 16] = Default::default();
        let input_b: [Rc<Pin>; 16] = Default::default();
        let input_c: [Rc<Pin>; 16] = Default::default();
        let input_d: [Rc<Pin>; 16] = Default::default();
        let sel: [Rc<Pin>; 2] = Default::default();
        let output: [Rc<Pin>; 16] = Default::default();
        let result = Self {
            input_a,
            input_b,
            input_c,
            input_d,
            sel,
            output,
        };

        let or16_a = TwoInOneOut16::or16();
        let or16_b = TwoInOneOut16::or16();
        let or16_c = TwoInOneOut16::or16();

        let mux16_a = Mux16::new();
        let mux16_b = Mux16::new();
        let mux16_c = Mux16::new();
        let mux16_d = Mux16::new();

        let and_a = TwoInOneOutGate::and();
        let and_b = TwoInOneOutGate::and();
        let and_c = TwoInOneOutGate::and();
        let and_d = TwoInOneOutGate::and();

        let not_a = NotGate::new();
        let not_b = NotGate::new();
        let not_c = NotGate::new();

        result
    }
}

// fn mux4way16(
//     input_a: [bool; 16],
//     input_b: [bool; 16],
//     input_c: [bool; 16],
//     input_d: [bool; 16],
//     sel: [bool; 2],
// ) -> [bool; 16] {
//     or16a(
//         or16b(
//             mux16a([false; 16], input_a, anda(nota(sel[0]), notb(sel[1]))),
//             mux16b([false; 16], input_b, andb(notc(sel[0]), sel[1])),
//         ),
//         or16c(
//             mux16c([false; 16], input_c, andc(sel[0], notd(sel[1]))),
//             mux16d([false; 16], input_d, andd(sel[0], sel[1])),
//         ),
//     )
// }

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
//     or16(
//         or16(
//             or16(
//                 mux16(
//                     [false; 16],
//                     input_a,
//                     and(not(sel[0]), and(not(sel[1]), not(sel[2]))),
//                 ),
//                 mux16(
//                     [false; 16],
//                     input_b,
//                     and(not(sel[0]), and(not(sel[1]), sel[2])),
//                 ),
//             ),
//             or16(
//                 mux16(
//                     [false; 16],
//                     input_c,
//                     and(not(sel[0]), and(sel[1], not(sel[2]))),
//                 ),
//                 mux16([false; 16], input_d, and(not(sel[0]), and(sel[1], sel[2]))),
//             ),
//         ),
//         or16(
//             or16(
//                 mux16(
//                     [false; 16],
//                     input_e,
//                     and(sel[0], and(not(sel[1]), not(sel[2]))),
//                 ),
//                 mux16([false; 16], input_f, and(sel[0], and(not(sel[1]), sel[2]))),
//             ),
//             or16(
//                 mux16([false; 16], input_g, and(sel[0], and(sel[1], not(sel[2])))),
//                 mux16([false; 16], input_h, and(sel[0], and(sel[1], sel[2]))),
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
//     use super::*;
//     #[test]
//     fn test_nand() {
//         assert_eq!(nand(false, false), true);
//         assert_eq!(nand(false, true), true);
//         assert_eq!(nand(true, false), true);
//         assert_eq!(nand(true, true), false);
//     }

//     #[test]
//     fn test_not() {
//         assert_eq!(not(true), false);
//         assert_eq!(not(false), true);
//     }

//     #[test]
//     fn test_or() {
//         assert_eq!(or(false, false), false);
//         assert_eq!(or(false, true), true);
//         assert_eq!(or(true, false), true);
//         assert_eq!(or(true, true), true);
//     }

//     #[test]
//     fn test_xor() {
//         assert_eq!(xor(false, false), false);
//         assert_eq!(xor(false, true), true);
//         assert_eq!(xor(true, false), true);
//         assert_eq!(xor(true, true), false);
//     }

//     #[test]
//     fn test_mux() {
//         assert_eq!(mux(false, false, false), false);
//         assert_eq!(mux(false, true, false), false);
//         assert_eq!(mux(true, false, false), true);
//         assert_eq!(mux(true, true, false), true);
//         assert_eq!(mux(false, false, true), false);
//         assert_eq!(mux(false, true, true), true);
//         assert_eq!(mux(true, false, true), false);
//         assert_eq!(mux(true, true, true), true);
//     }

//     #[test]
//     fn test_dmux() {
//         assert_eq!(dmux(false, false), [false, false]);
//         assert_eq!(dmux(false, true), [false, false]);
//         assert_eq!(dmux(true, false), [true, false]);
//         assert_eq!(dmux(true, true), [false, true]);
//     }

//     #[test]
//     fn test_not16() {
//         assert_eq!(
//             not16([
//                 true, false, false, true, true, true, false, true, false, true, false, true, true,
//                 false, false, true
//             ]),
//             [
//                 false, true, true, false, false, false, true, false, true, false, true, false,
//                 false, true, true, false
//             ]
//         );
//     }

//     #[test]
//     fn test_and16() {
//         assert_eq!(
//             and16(
//                 [
//                     false, true, true, false, false, false, true, false, true, false, true, false,
//                     false, true, true, false
//                 ],
//                 [
//                     true, true, false, false, true, false, false, true, false, true, false, false,
//                     false, true, false, false
//                 ]
//             ),
//             [
//                 false, true, false, false, false, false, false, false, false, false, false, false,
//                 false, true, false, false
//             ]
//         );
//     }

//     #[test]
//     fn test_or16() {
//         assert_eq!(
//             or16(
//                 [
//                     false, true, true, false, false, false, true, false, true, false, true, false,
//                     false, true, true, false
//                 ],
//                 [
//                     true, true, false, false, true, false, false, true, false, true, false, false,
//                     false, true, false, false
//                 ]
//             ),
//             [
//                 true, true, true, false, true, false, true, true, true, true, true, false, false,
//                 true, true, false
//             ]
//         );
//     }

//     #[test]
//     fn test_mux16() {
//         assert_eq!(
//             mux16(
//                 [
//                     false, true, true, false, false, false, true, false, true, false, true, false,
//                     false, true, true, false
//                 ],
//                 [
//                     true, true, false, false, true, false, false, true, false, true, false, false,
//                     false, true, false, false
//                 ],
//                 false
//             ),
//             [
//                 false, true, true, false, false, false, true, false, true, false, true, false,
//                 false, true, true, false
//             ]
//         );
//         assert_eq!(
//             mux16(
//                 [
//                     false, true, true, false, false, false, true, false, true, false, true, false,
//                     false, true, true, false
//                 ],
//                 [
//                     true, true, false, false, true, false, false, true, false, true, false, false,
//                     false, true, false, false
//                 ],
//                 true
//             ),
//             [
//                 true, true, false, false, true, false, false, true, false, true, false, false,
//                 false, true, false, false
//             ],
//         );
//     }

//     #[test]
//     fn test_or8way() {
//         assert_eq!(
//             or8way([false, false, false, false, false, false, false, false]),
//             false
//         );
//         assert_eq!(
//             or8way([false, false, false, false, true, false, false, false]),
//             true
//         );
//         assert_eq!(
//             or8way([true, true, true, true, true, true, true, true]),
//             true
//         );
//     }

//     #[test]
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
