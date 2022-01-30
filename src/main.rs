mod boolean_arithmetic;
mod boolean_logic;
mod ordering;
mod pin;
mod sequential_logic;
mod test_utils;

use crate::ordering::{get_all_connected_pins, reverse_topological_sort, sort_and_compute};
use crate::pin::{
    Connection, OptimizedConnection, OptimizedFlipFlop, OptimizedPin, OptimizedPinCollection, Pin,
};
use crate::sequential_logic::{Ram16k, Ram4k, Ram512};
use crate::test_utils::i16_to_bools;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct PinLinks {
    incoming: Vec<Connection>,
    outgoing: Option<Connection>,
}

impl PinLinks {
    fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: None,
        }
    }

    fn is_useless(&self) -> bool {
        if self.incoming.len() == 1 {
            match self.incoming[0] {
                Connection::Eq(_) => match self.outgoing {
                    Some(Connection::Eq(_)) => true,
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }
}

fn get_all_pin_links(all_pins: &HashSet<Rc<Pin>>) -> HashMap<Rc<Pin>, PinLinks> {
    let mut pin_map = HashMap::new();
    for pin in all_pins {
        let connections_for_source_pin = pin_map.entry(pin.clone()).or_insert(PinLinks::new());
        let connection_ref = pin.connection.borrow();
        match connection_ref.as_ref() {
            Some(connection) => {
                connections_for_source_pin
                    .outgoing
                    .replace((*connection).clone());
                for target_pin in connection.pins() {
                    let connections_for_target_pin =
                        pin_map.entry(target_pin.clone()).or_insert(PinLinks::new());
                    connections_for_target_pin.incoming.push(connection.clone());
                }
            }
            None => {}
        }
    }
    pin_map
}

#[test]
fn test_get_all_pin_links() {
    let pin_a = Pin::new();
    let pin_b = Pin::new();
    let pin_c = Pin::new();
    let pin_d = Pin::new();
    let pin_e = Pin::new();

    pin_a.feed_from(pin_b.clone());
    pin_b.nand_connect(pin_c.clone(), pin_d.clone());
    pin_d.flipflop_connect(pin_e.clone());

    let all_pins = get_all_connected_pins(&[pin_a.clone()].to_vec());
    let pin_links = get_all_pin_links(&all_pins);
    let mut expected = HashMap::new();
    let eq = Connection::Eq(pin_b.clone());
    let nand = Connection::Nand(pin_c.clone(), pin_d.clone());
    let ff = Connection::FlipFlop(pin_e.clone());
    expected.insert(
        pin_a,
        PinLinks {
            incoming: vec![],
            outgoing: Some(eq.clone()),
        },
    );
    expected.insert(
        pin_b.clone(),
        PinLinks {
            incoming: vec![eq.clone()],
            outgoing: Some(nand.clone()),
        },
    );
    expected.insert(
        pin_c.clone(),
        PinLinks {
            incoming: vec![nand.clone()],
            outgoing: None,
        },
    );
    expected.insert(
        pin_d,
        PinLinks {
            incoming: vec![nand],
            outgoing: Some(ff.clone()),
        },
    );
    expected.insert(
        pin_e,
        PinLinks {
            incoming: vec![ff],
            outgoing: None,
        },
    );
    assert_eq!(pin_links, expected);
}

fn optimize(pins: Vec<Rc<Pin>>) -> OptimizedPinCollection {
    let mut pin_indices = HashMap::new();
    for (pin_idx, pin) in pins.iter().enumerate() {
        pin_indices.insert(pin.clone(), pin_idx);
    }

    let mut result = OptimizedPinCollection {
        pins: vec![],
        flipflops: vec![],
    };

    let get_pin_idx = |pin| *pin_indices.get(pin).unwrap();

    for pin in pins.into_iter() {
        let foo = pin.connection.borrow();
        match foo.as_ref() {
            Some(Connection::Eq(other_pin)) => {
                let new_connection = OptimizedConnection::Eq(get_pin_idx(other_pin));
                let new_pin = OptimizedPin {
                    connection: Some(new_connection),
                    value: Cell::new(pin.value.get()),
                };
                result.pins.push(new_pin);
            }
            Some(Connection::Nand(other_pin_a, other_pin_b)) => {
                let new_connection =
                    OptimizedConnection::Nand(get_pin_idx(other_pin_a), get_pin_idx(other_pin_b));
                let new_pin = OptimizedPin {
                    connection: Some(new_connection),
                    value: Cell::new(pin.value.get()),
                };
                result.pins.push(new_pin);
            }
            Some(Connection::FlipFlop(other_pin)) => {
                let new_pin = OptimizedPin {
                    connection: None,
                    value: Cell::new(pin.value.get()),
                };
                result.pins.push(new_pin);
                result.flipflops.push(OptimizedFlipFlop {
                    input: get_pin_idx(&pin),
                    output: get_pin_idx(other_pin),
                });
            }
            None => {}
        }
    }

    result
}

fn main() {
    println!("creating ram");
    let ram = Ram16k::new();
    let output_pins = ram.output.pins.to_vec();
    let all_pins = get_all_connected_pins(&output_pins);
    let pin_links = get_all_pin_links(&all_pins);
    let useless_pin_count = pin_links
        .iter()
        .filter(|(_, links)| links.is_useless())
        .count();

    println!(
        "{} of {} pins are useless",
        useless_pin_count,
        all_pins.len()
    );

    ram.input.set_values(i16_to_bools(1234));
    ram.load.value.set(true);
    let address = [false; 14];
    for i in 0..address.len() {
        ram.address[i].value.set(address[i]);
    }
    println!("sorting");
    let sorted_pins = reverse_topological_sort(&all_pins);
    println!("optimizing");
    let optimized_pins = optimize(sorted_pins);
    println!("computing");
    for pin in optimized_pins.pins {
        // TODO - need to impl this...
        pin.compute();
    }
    println!("ticking");
    for flipflop in optimized_pins.flipflops {
        let output_pin = optimized_pins.pins[flipflop.output];
        let input_pin = optimized_pins.pins[flipflop.input];
        output_pin.value.set(input_pin.value.get());
    }
    println!("computing");
    for pin in optimized_pins.pins {
        pin.compute();
    }
    // TODO - read output from optimized pins...
    let result: Vec<bool> = output_pins.iter().map(|pin| pin.value.get()).collect();
    println!("{:?}", result);
}
