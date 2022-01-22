use std::cell::{Cell, RefCell};
use std::rc::Rc;

enum Connection {
    Eq(Rc<Pin>),
    Nand(Rc<Pin>, Rc<Pin>),
}

struct Pin {
    value: Cell<bool>,
    connection: RefCell<Option<Connection>>,
}

impl Pin {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            value: Cell::new(false),
            connection: RefCell::new(None),
        })
    }
    fn connect(&self, connection: Connection) {
        self.connection.borrow_mut().replace(connection);
    }
    fn compute(&self) {
        let new_value = match self.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => pin.value.get(),
            Some(Connection::Nand(pin_a, pin_b)) => !(pin_a.value.get() && pin_b.value.get()),
            None => {
                panic!("can't compute pin without any connections");
            }
        };
        self.value.set(new_value);
    }
}

#[test]
fn test_nand_gate() {
    let output = Pin::new();
    let input_a = Pin::new();
    let input_b = Pin::new();
    output.connect(Connection::Nand(input_a.clone(), input_b.clone()));
    input_a.value.set(false);
    input_b.value.set(false);
}

// struct NotGate {
//     input: Rc<Pin>,
//     output: Rc<Pin>,
//     nand_gate: NandGate,
// }

// impl NotGate {
//     fn new() -> Self {
//         let input = Pin::new();
//         let output = Pin::new();
//         let nand_gate = NandGate::new();
//         let result = Self {
//             input,
//             output,
//             nand_gate,
//         };
//         result.output.connect(&result.nand_gate.output);
//         result.nand_gate.input_a.connect(&result.input);
//         result.nand_gate.input_b.connect(&result.input);
//         result
//     }
// }

// #[test]
// fn test_not_gate() {
//     let not_gate = NotGate::new();
// }

// // struct AndGate {
// //     input_a: Rc<Pin>,
// //     input_b: Rc<Pin>,
// //     output: Rc<Pin>,
// // }

// // impl AndGate {
// //     fn new() -> Self {
// //         let input_a = Rc::new(Pin::new());
// //         let input_b = Rc::new(Pin::new());
// //         let output = Rc::new(Pin::new());
// //         let nand_gate = NandGate::new();
// //         let not_gate = NotGate::new();
// //         input_a.connect(nand_gate.input_a);
// //         input_b.connect(nand_gate.input_b);
// //         nand_gate.output.connect(not_gate.input);
// //         not_gate.output.connect(output.clone());
// //         Self {
// //             input_a,
// //             input_b,
// //             output,
// //         }
// //     }
// // }

// // struct OrGate {
// //     input_a: Rc<Pin>,
// //     input_b: Rc<Pin>,
// //     output: Rc<Pin>,
// // }

// // impl OrGate {
// //     fn new() -> Self {
// //         let input_a = Rc::new(Pin::new());
// //         let input_b = Rc::new(Pin::new());
// //         let output = Rc::new(Pin::new());

// //         let nand_a = NandGate::new();
// //         let nand_b = NandGate::new();
// //         let nand_c = NandGate::new();

// //         input_a.connect(nand_a.input_a);
// //         input_a.connect(nand_a.input_b);

// //         input_b.connect(nand_b.input_a);
// //         input_b.connect(nand_b.input_b);

// //         nand_a.output.connect(nand_c.input_a);
// //         nand_b.output.connect(nand_c.input_b);
// //         nand_c.output.connect(output.clone());

// //         Self {
// //             input_a,
// //             input_b,
// //             output,
// //         }
// //     }
// // }

// // // // pub fn or(input_a: bool, input_b: bool) -> bool {
// // // //     nand(nand(input_a, input_a), nand(input_b, input_b))
// // // // }

// // // // pub fn xor(input_a: bool, input_b: bool) -> bool {
// // // //     nand(
// // // //         nand(input_a, nand(input_a, input_b)),
// // // //         nand(nand(input_a, input_b), input_b),
// // // //     )
// // // // }

// // // // pub fn mux(input_a: bool, input_b: bool, sel: bool) -> bool {
// // // //     or(and(input_a, not(sel)), and(input_b, sel))
// // // // }

// // // // fn dmux(input: bool, sel: bool) -> [bool; 2] {
// // // //     [and(not(sel), input), and(sel, input)]
// // // // }

// // // // pub fn not16(input: [bool; 16]) -> [bool; 16] {
// // // //     [
// // // //         not(input[0]),
// // // //         not(input[1]),
// // // //         not(input[2]),
// // // //         not(input[3]),
// // // //         not(input[4]),
// // // //         not(input[5]),
// // // //         not(input[6]),
// // // //         not(input[7]),
// // // //         not(input[8]),
// // // //         not(input[9]),
// // // //         not(input[10]),
// // // //         not(input[11]),
// // // //         not(input[12]),
// // // //         not(input[13]),
// // // //         not(input[14]),
// // // //         not(input[15]),
// // // //     ]
// // // // }

// // // // pub fn and16(input_a: [bool; 16], input_b: [bool; 16]) -> [bool; 16] {
// // // //     [
// // // //         and(input_a[0], input_b[0]),
// // // //         and(input_a[1], input_b[1]),
// // // //         and(input_a[2], input_b[2]),
// // // //         and(input_a[3], input_b[3]),
// // // //         and(input_a[4], input_b[4]),
// // // //         and(input_a[5], input_b[5]),
// // // //         and(input_a[6], input_b[6]),
// // // //         and(input_a[7], input_b[7]),
// // // //         and(input_a[8], input_b[8]),
// // // //         and(input_a[9], input_b[9]),
// // // //         and(input_a[10], input_b[10]),
// // // //         and(input_a[11], input_b[11]),
// // // //         and(input_a[12], input_b[12]),
// // // //         and(input_a[13], input_b[13]),
// // // //         and(input_a[14], input_b[14]),
// // // //         and(input_a[15], input_b[15]),
// // // //     ]
// // // // }

// // // // fn or16(input_a: [bool; 16], input_b: [bool; 16]) -> [bool; 16] {
// // // //     [
// // // //         or(input_a[0], input_b[0]),
// // // //         or(input_a[1], input_b[1]),
// // // //         or(input_a[2], input_b[2]),
// // // //         or(input_a[3], input_b[3]),
// // // //         or(input_a[4], input_b[4]),
// // // //         or(input_a[5], input_b[5]),
// // // //         or(input_a[6], input_b[6]),
// // // //         or(input_a[7], input_b[7]),
// // // //         or(input_a[8], input_b[8]),
// // // //         or(input_a[9], input_b[9]),
// // // //         or(input_a[10], input_b[10]),
// // // //         or(input_a[11], input_b[11]),
// // // //         or(input_a[12], input_b[12]),
// // // //         or(input_a[13], input_b[13]),
// // // //         or(input_a[14], input_b[14]),
// // // //         or(input_a[15], input_b[15]),
// // // //     ]
// // // // }

// // // // pub fn mux16(input_a: [bool; 16], input_b: [bool; 16], sel: bool) -> [bool; 16] {
// // // //     [
// // // //         mux(input_a[0], input_b[0], sel),
// // // //         mux(input_a[1], input_b[1], sel),
// // // //         mux(input_a[2], input_b[2], sel),
// // // //         mux(input_a[3], input_b[3], sel),
// // // //         mux(input_a[4], input_b[4], sel),
// // // //         mux(input_a[5], input_b[5], sel),
// // // //         mux(input_a[6], input_b[6], sel),
// // // //         mux(input_a[7], input_b[7], sel),
// // // //         mux(input_a[8], input_b[8], sel),
// // // //         mux(input_a[9], input_b[9], sel),
// // // //         mux(input_a[10], input_b[10], sel),
// // // //         mux(input_a[11], input_b[11], sel),
// // // //         mux(input_a[12], input_b[12], sel),
// // // //         mux(input_a[13], input_b[13], sel),
// // // //         mux(input_a[14], input_b[14], sel),
// // // //         mux(input_a[15], input_b[15], sel),
// // // //     ]
// // // // }

// // // // pub fn or8way(input: [bool; 8]) -> bool {
// // // //     or(
// // // //         or(or(input[0], input[1]), or(input[2], input[3])),
// // // //         or(or(input[4], input[5]), or(input[6], input[7])),
// // // //     )
// // // // }

// // // // fn mux4way16(
// // // //     input_a: [bool; 16],
// // // //     input_b: [bool; 16],
// // // //     input_c: [bool; 16],
// // // //     input_d: [bool; 16],
// // // //     sel: [bool; 2],
// // // // ) -> [bool; 16] {
// // // //     or16(
// // // //         or16(
// // // //             mux16([false; 16], input_a, and(not(sel[0]), not(sel[1]))),
// // // //             mux16([false; 16], input_b, and(not(sel[0]), sel[1])),
// // // //         ),
// // // //         or16(
// // // //             mux16([false; 16], input_c, and(sel[0], not(sel[1]))),
// // // //             mux16([false; 16], input_d, and(sel[0], sel[1])),
// // // //         ),
// // // //     )
// // // // }

// // // // fn mux8way16(
// // // //     input_a: [bool; 16],
// // // //     input_b: [bool; 16],
// // // //     input_c: [bool; 16],
// // // //     input_d: [bool; 16],
// // // //     input_e: [bool; 16],
// // // //     input_f: [bool; 16],
// // // //     input_g: [bool; 16],
// // // //     input_h: [bool; 16],
// // // //     sel: [bool; 3],
// // // // ) -> [bool; 16] {
// // // //     or16(
// // // //         or16(
// // // //             or16(
// // // //                 mux16(
// // // //                     [false; 16],
// // // //                     input_a,
// // // //                     and(not(sel[0]), and(not(sel[1]), not(sel[2]))),
// // // //                 ),
// // // //                 mux16(
// // // //                     [false; 16],
// // // //                     input_b,
// // // //                     and(not(sel[0]), and(not(sel[1]), sel[2])),
// // // //                 ),
// // // //             ),
// // // //             or16(
// // // //                 mux16(
// // // //                     [false; 16],
// // // //                     input_c,
// // // //                     and(not(sel[0]), and(sel[1], not(sel[2]))),
// // // //                 ),
// // // //                 mux16([false; 16], input_d, and(not(sel[0]), and(sel[1], sel[2]))),
// // // //             ),
// // // //         ),
// // // //         or16(
// // // //             or16(
// // // //                 mux16(
// // // //                     [false; 16],
// // // //                     input_e,
// // // //                     and(sel[0], and(not(sel[1]), not(sel[2]))),
// // // //                 ),
// // // //                 mux16([false; 16], input_f, and(sel[0], and(not(sel[1]), sel[2]))),
// // // //             ),
// // // //             or16(
// // // //                 mux16([false; 16], input_g, and(sel[0], and(sel[1], not(sel[2])))),
// // // //                 mux16([false; 16], input_h, and(sel[0], and(sel[1], sel[2]))),
// // // //             ),
// // // //         ),
// // // //     )
// // // // }

// // // // fn dmux4way(input: bool, sel: [bool; 2]) -> [bool; 4] {
// // // //     [
// // // //         and(input, and(not(sel[0]), not(sel[1]))),
// // // //         and(input, and(not(sel[0]), sel[1])),
// // // //         and(input, and(sel[0], not(sel[1]))),
// // // //         and(input, and(sel[0], sel[1])),
// // // //     ]
// // // // }

// // // // fn dmux8way(input: bool, sel: [bool; 3]) -> [bool; 8] {
// // // //     [
// // // //         and(input, and(not(sel[0]), and(not(sel[1]), not(sel[2])))),
// // // //         and(input, and(not(sel[0]), and(not(sel[1]), sel[2]))),
// // // //         and(input, and(not(sel[0]), and(sel[1], not(sel[2])))),
// // // //         and(input, and(not(sel[0]), and(sel[1], sel[2]))),
// // // //         and(input, and(sel[0], and(not(sel[1]), not(sel[2])))),
// // // //         and(input, and(sel[0], and(not(sel[1]), sel[2]))),
// // // //         and(input, and(sel[0], and(sel[1], not(sel[2])))),
// // // //         and(input, and(sel[0], and(sel[1], sel[2]))),
// // // //     ]
// // // // }

// // // // #[cfg(test)]
// // // // mod tests {
// // // //     use super::*;
// // // //     #[test]
// // // //     fn test_nand() {
// // // //         assert_eq!(nand(false, false), true);
// // // //         assert_eq!(nand(false, true), true);
// // // //         assert_eq!(nand(true, false), true);
// // // //         assert_eq!(nand(true, true), false);
// // // //     }

// // // //     #[test]
// // // //     fn test_not() {
// // // //         assert_eq!(not(true), false);
// // // //         assert_eq!(not(false), true);
// // // //     }

// // // //     #[test]
// // // //     fn test_and() {
// // // //         assert_eq!(and(false, false), false);
// // // //         assert_eq!(and(false, true), false);
// // // //         assert_eq!(and(true, false), false);
// // // //         assert_eq!(and(true, true), true);
// // // //     }

// // // //     #[test]
// // // //     fn test_or() {
// // // //         assert_eq!(or(false, false), false);
// // // //         assert_eq!(or(false, true), true);
// // // //         assert_eq!(or(true, false), true);
// // // //         assert_eq!(or(true, true), true);
// // // //     }

// // // //     #[test]
// // // //     fn test_xor() {
// // // //         assert_eq!(xor(false, false), false);
// // // //         assert_eq!(xor(false, true), true);
// // // //         assert_eq!(xor(true, false), true);
// // // //         assert_eq!(xor(true, true), false);
// // // //     }

// // // //     #[test]
// // // //     fn test_mux() {
// // // //         assert_eq!(mux(false, false, false), false);
// // // //         assert_eq!(mux(false, true, false), false);
// // // //         assert_eq!(mux(true, false, false), true);
// // // //         assert_eq!(mux(true, true, false), true);
// // // //         assert_eq!(mux(false, false, true), false);
// // // //         assert_eq!(mux(false, true, true), true);
// // // //         assert_eq!(mux(true, false, true), false);
// // // //         assert_eq!(mux(true, true, true), true);
// // // //     }

// // // //     #[test]
// // // //     fn test_dmux() {
// // // //         assert_eq!(dmux(false, false), [false, false]);
// // // //         assert_eq!(dmux(false, true), [false, false]);
// // // //         assert_eq!(dmux(true, false), [true, false]);
// // // //         assert_eq!(dmux(true, true), [false, true]);
// // // //     }

// // // //     #[test]
// // // //     fn test_not16() {
// // // //         assert_eq!(
// // // //             not16([
// // // //                 true, false, false, true, true, true, false, true, false, true, false, true, true,
// // // //                 false, false, true
// // // //             ]),
// // // //             [
// // // //                 false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                 false, true, true, false
// // // //             ]
// // // //         );
// // // //     }

// // // //     #[test]
// // // //     fn test_and16() {
// // // //         assert_eq!(
// // // //             and16(
// // // //                 [
// // // //                     false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                     false, true, true, false
// // // //                 ],
// // // //                 [
// // // //                     true, true, false, false, true, false, false, true, false, true, false, false,
// // // //                     false, true, false, false
// // // //                 ]
// // // //             ),
// // // //             [
// // // //                 false, true, false, false, false, false, false, false, false, false, false, false,
// // // //                 false, true, false, false
// // // //             ]
// // // //         );
// // // //     }

// // // //     #[test]
// // // //     fn test_or16() {
// // // //         assert_eq!(
// // // //             or16(
// // // //                 [
// // // //                     false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                     false, true, true, false
// // // //                 ],
// // // //                 [
// // // //                     true, true, false, false, true, false, false, true, false, true, false, false,
// // // //                     false, true, false, false
// // // //                 ]
// // // //             ),
// // // //             [
// // // //                 true, true, true, false, true, false, true, true, true, true, true, false, false,
// // // //                 true, true, false
// // // //             ]
// // // //         );
// // // //     }

// // // //     #[test]
// // // //     fn test_mux16() {
// // // //         assert_eq!(
// // // //             mux16(
// // // //                 [
// // // //                     false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                     false, true, true, false
// // // //                 ],
// // // //                 [
// // // //                     true, true, false, false, true, false, false, true, false, true, false, false,
// // // //                     false, true, false, false
// // // //                 ],
// // // //                 false
// // // //             ),
// // // //             [
// // // //                 false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                 false, true, true, false
// // // //             ]
// // // //         );
// // // //         assert_eq!(
// // // //             mux16(
// // // //                 [
// // // //                     false, true, true, false, false, false, true, false, true, false, true, false,
// // // //                     false, true, true, false
// // // //                 ],
// // // //                 [
// // // //                     true, true, false, false, true, false, false, true, false, true, false, false,
// // // //                     false, true, false, false
// // // //                 ],
// // // //                 true
// // // //             ),
// // // //             [
// // // //                 true, true, false, false, true, false, false, true, false, true, false, false,
// // // //                 false, true, false, false
// // // //             ],
// // // //         );
// // // //     }

// // // //     #[test]
// // // //     fn test_or8way() {
// // // //         assert_eq!(
// // // //             or8way([false, false, false, false, false, false, false, false]),
// // // //             false
// // // //         );
// // // //         assert_eq!(
// // // //             or8way([false, false, false, false, true, false, false, false]),
// // // //             true
// // // //         );
// // // //         assert_eq!(
// // // //             or8way([true, true, true, true, true, true, true, true]),
// // // //             true
// // // //         );
// // // //     }

// // // //     #[test]
// // // //     fn test_mux4way16() {
// // // //         let a = [
// // // //             true, false, true, false, false, true, false, true, false, false, false, true, false,
// // // //             true, true, false,
// // // //         ];
// // // //         let b = [
// // // //             false, false, true, false, true, true, false, false, true, false, true, true, false,
// // // //             false, false, true,
// // // //         ];
// // // //         let c = [
// // // //             false, false, true, true, false, false, true, true, false, true, true, false, false,
// // // //             true, false, true,
// // // //         ];
// // // //         let d = [
// // // //             true, true, false, false, true, true, false, true, false, false, false, true, true,
// // // //             false, false, true,
// // // //         ];
// // // //         assert_eq!(mux4way16(a, b, c, d, [false, false]), a);
// // // //         assert_eq!(mux4way16(a, b, c, d, [false, true]), b);
// // // //         assert_eq!(mux4way16(a, b, c, d, [true, false]), c);
// // // //         assert_eq!(mux4way16(a, b, c, d, [true, true]), d);
// // // //     }

// // // //     #[test]
// // // //     fn test_mux8way16() {
// // // //         let a = [
// // // //             true, false, true, false, false, true, false, true, false, false, false, true, false,
// // // //             true, true, false,
// // // //         ];
// // // //         let b = [
// // // //             false, false, true, false, true, true, false, false, true, false, true, true, false,
// // // //             false, false, true,
// // // //         ];
// // // //         let c = [
// // // //             false, false, true, true, false, false, true, true, false, true, true, false, false,
// // // //             true, false, true,
// // // //         ];
// // // //         let d = [
// // // //             true, true, false, false, true, true, false, true, false, false, false, true, true,
// // // //             false, false, true,
// // // //         ];
// // // //         let e = [
// // // //             false, false, true, true, false, true, false, false, true, false, true, false, false,
// // // //             true, false, false,
// // // //         ];
// // // //         let f = [
// // // //             true, true, true, true, false, false, false, true, false, true, false, false, false,
// // // //             true, false, true,
// // // //         ];
// // // //         let g = [
// // // //             false, true, false, true, false, false, true, true, false, true, false, false, true,
// // // //             false, true, true,
// // // //         ];
// // // //         let h = [
// // // //             false, false, false, true, true, false, true, false, true, false, false, false, true,
// // // //             true, false, true,
// // // //         ];
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, false]), a);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, false, true]), b);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, false]), c);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [false, true, true]), d);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, false]), e);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, false, true]), f);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, false]), g);
// // // //         assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [true, true, true]), h);
// // // //     }

// // // //     #[test]
// // // //     fn test_dmux4way() {
// // // //         assert_eq!(
// // // //             dmux4way(false, [false, false]),
// // // //             [false, false, false, false]
// // // //         );
// // // //         assert_eq!(dmux4way(false, [false, true]), [false, false, false, false]);
// // // //         assert_eq!(dmux4way(false, [true, false]), [false, false, false, false]);
// // // //         assert_eq!(dmux4way(false, [true, true]), [false, false, false, false]);
// // // //         assert_eq!(dmux4way(true, [false, false]), [true, false, false, false]);
// // // //         assert_eq!(dmux4way(true, [false, true]), [false, true, false, false]);
// // // //         assert_eq!(dmux4way(true, [true, false]), [false, false, true, false]);
// // // //         assert_eq!(dmux4way(true, [true, true]), [false, false, false, true]);
// // // //     }

// // // //     #[test]
// // // //     fn test_dmux8way() {
// // // //         assert_eq!(
// // // //             dmux8way(false, [false, false, false]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [false, false, true]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [false, true, false]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [false, true, true]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [true, false, false]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [true, false, true]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [true, true, false]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(false, [true, true, true]),
// // // //             [false, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [false, false, false]),
// // // //             [true, false, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [false, false, true]),
// // // //             [false, true, false, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [false, true, false]),
// // // //             [false, false, true, false, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [false, true, true]),
// // // //             [false, false, false, true, false, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [true, false, false]),
// // // //             [false, false, false, false, true, false, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [true, false, true]),
// // // //             [false, false, false, false, false, true, false, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [true, true, false]),
// // // //             [false, false, false, false, false, false, true, false]
// // // //         );
// // // //         assert_eq!(
// // // //             dmux8way(true, [true, true, true]),
// // // //             [false, false, false, false, false, false, false, true]
// // // //         );
// // // //     }
// // // // }
