mod boolean_arithmetic;
mod boolean_logic;
mod ordering;
mod pin;
mod sequential_logic;
mod test_utils;

use crate::ordering::{all_connected_pins, compute_all};
use crate::sequential_logic::Ram512;
use crate::test_utils::i16_to_bools;

fn main() {
    println!("creating ram");
    let ram = Ram512::new();
    let all_pins = all_connected_pins(ram.output.pins.to_vec());
    ram.input.set_values(i16_to_bools(1234));
    ram.load.value.set(true);
    let address = [false; 9];
    for i in 0..address.len() {
        ram.address[i].value.set(address[i]);
    }
    println!("computing");
    compute_all(&ram.output.pins, &all_pins);
    println!("ticking");
    ram.tick();
    println!("computing");
    let result = compute_all(&ram.output.pins, &all_pins);
    println!("{:?}", result);
}
