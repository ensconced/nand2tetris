pub fn binary(num: i16) -> [bool; 16] {
    let bit_string = format!("{:016b}", num);
    let bit_vec: Vec<bool> = bit_string.chars().map(|char| char == '1').collect();
    bit_vec.try_into().unwrap()
}

#[test]
fn test_binary() {
    assert_eq!(binary(0), [false; 16]);
    assert_eq!(
        binary(9),
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, true
        ]
    );
}
