use crate::boolean_logic::{and, or, xor};

fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [and(a, b), xor(a, b)]
}

// returns (carry, sum)
fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    let first_half_adder_output = half_adder(a, b);
    let second_half_adder_output = half_adder(first_half_adder_output[1], c);
    [
        or(first_half_adder_output[0], second_half_adder_output[0]),
        second_half_adder_output[1],
    ]
}

// integer 2's complement addition - overflow is neither detected nor handled
fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let adder1 = half_adder(a[15], b[15]);
    let adder2 = full_adder(a[14], b[14], adder1[0]);
    let adder3 = full_adder(a[13], b[13], adder2[0]);
    let adder4 = full_adder(a[12], b[12], adder3[0]);
    let adder5 = full_adder(a[11], b[11], adder4[0]);
    let adder6 = full_adder(a[10], b[10], adder5[0]);
    let adder7 = full_adder(a[9], b[9], adder6[0]);
    let adder8 = full_adder(a[8], b[8], adder7[0]);
    let adder9 = full_adder(a[7], b[7], adder8[0]);
    let adder10 = full_adder(a[6], b[6], adder9[0]);
    let adder11 = full_adder(a[5], b[5], adder10[0]);
    let adder12 = full_adder(a[4], b[4], adder11[0]);
    let adder13 = full_adder(a[3], b[3], adder12[0]);
    let adder14 = full_adder(a[2], b[2], adder13[0]);
    let adder15 = full_adder(a[1], b[1], adder14[0]);
    let adder16 = full_adder(a[0], b[0], adder15[0]);
    [
        adder16[1], adder15[1], adder14[1], adder13[1], adder12[1], adder11[1], adder10[1],
        adder9[1], adder8[1], adder7[1], adder6[1], adder5[1], adder4[1], adder3[1], adder2[1],
        adder1[1],
    ]
}

fn inc16(input: [bool; 16]) -> [bool; 16] {
    let mut one = [false; 16];
    one[15] = true;
    add16(input, one)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binary(num: i16) -> [bool; 16] {
        let bit_string = format!("{:016b}", num);
        let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
        bit_vec.try_into().unwrap()
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), [false, false]);
        assert_eq!(half_adder(false, true), [false, true]);
        assert_eq!(half_adder(true, false), [false, true]);
        assert_eq!(half_adder(true, true), [true, false]);
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), [false, false]);
        assert_eq!(full_adder(false, false, true), [false, true]);
        assert_eq!(full_adder(false, true, false), [false, true]);
        assert_eq!(full_adder(false, true, true), [true, false]);
        assert_eq!(full_adder(true, false, false), [false, true]);
        assert_eq!(full_adder(true, false, true), [true, false]);
        assert_eq!(full_adder(true, true, false), [true, false]);
        assert_eq!(full_adder(true, true, true), [true, true]);
    }

    #[test]
    fn test_add16() {
        assert_eq!(add16(binary(0), binary(0)), binary(0));
        assert_eq!(add16(binary(0), binary(1)), binary(1));
        assert_eq!(add16(binary(1), binary(0)), binary(1));
        assert_eq!(add16(binary(1), binary(-1)), binary(0));
        assert_eq!(add16(binary(123), binary(-123)), binary(0));
        assert_eq!(add16(binary(1000), binary(1000)), binary(2000));
    }

    #[test]
    fn test_inc16() {
        assert_eq!(inc16(binary(0)), binary(1));
        assert_eq!(inc16(binary(123)), binary(124));
        assert_eq!(inc16(binary(i16::MAX)), binary(i16::MIN));
    }
}
