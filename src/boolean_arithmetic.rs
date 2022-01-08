use crate::boolean_logic::{and, or, xor};

fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [and(a, b), xor(a, b)]
}

fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let first_half_adder_output = half_adder(a, b);
    let second_half_adder_output = half_adder(first_half_adder_output[1], c);
    [
        or(first_half_adder_output[0], second_half_adder_output[0]),
        second_half_adder_output[1],
    ]
}
