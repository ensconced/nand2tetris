pub fn i16_to_bools(num: i16) -> [bool; 16] {
    let bit_string = format!("{:016b}", num);
    let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
    bit_vec.try_into().unwrap()
}

#[test]
fn test_binaryi16() {
    assert_eq!(i16_to_bools(0), [false; 16]);
    assert_eq!(
        i16_to_bools(9),
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, true
        ]
    );
}

pub fn u8_to_bools(num: u8) -> [bool; 8] {
    let bit_string = format!("{:08b}", num);
    let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
    bit_vec.try_into().unwrap()
}

#[test]
fn test_binaryu8() {
    assert_eq!(u8_to_bools(0), [false; 8]);
    assert_eq!(u8_to_bools(u8::MAX), [true; 8]);
    assert_eq!(
        u8_to_bools(123),
        [false, true, true, true, true, false, true, true]
    );
}

pub fn bools_to_usize(bools: &[bool]) -> usize {
    let mut result = 0;
    for (idx, val) in bools.iter().rev().enumerate() {
        let bit = if *val { 1 } else { 0 };
        result += usize::pow(2, idx as u32) * bit;
    }
    result
}

#[test]
fn test_bools_to_usize() {
    assert_eq!(bools_to_usize(&[]), 0);
    assert_eq!(bools_to_usize(&[true]), 1);
    assert_eq!(bools_to_usize(&[true, true]), 3);
    assert_eq!(bools_to_usize(&[false]), 0);
    assert_eq!(bools_to_usize(&[false, false]), 0);
    assert_eq!(
        bools_to_usize(&[false, true, true, true, true, false, true, true]),
        123
    );
}

fn last_n(arr: [bool; 8], n: usize) -> Vec<bool> {
    arr.into_iter().rev().take(n).rev().collect()
}

pub fn last_3(arr: [bool; 8]) -> [bool; 3] {
    last_n(arr, 3).try_into().unwrap()
}

pub fn last_2(arr: [bool; 8]) -> [bool; 2] {
    last_n(arr, 2).try_into().unwrap()
}
