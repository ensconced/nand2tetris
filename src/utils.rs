pub fn binaryi16(num: i16) -> [bool; 16] {
    let bit_string = format!("{:016b}", num);
    let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
    bit_vec.try_into().unwrap()
}

pub fn binaryu8(num: u8) -> [bool; 8] {
    let bit_string = format!("{:08b}", num);
    let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
    bit_vec.try_into().unwrap()
}

#[test]
fn test_binary() {
    assert_eq!(binaryi16(0), [false; 16]);
    assert_eq!(
        binaryi16(9),
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, true
        ]
    );
}
