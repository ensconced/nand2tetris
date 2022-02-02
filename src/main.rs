mod boolean_arithmetic;
mod boolean_logic;
mod ordering;
mod pin;
mod sequential_logic;
mod test_utils;

use crate::boolean_arithmetic::ALU;
use crate::ordering::{get_all_connected_pins, reverse_topological_sort, sort_and_compute};
use crate::pin::{
    Connection, OptimizedConnection, OptimizedFlipFlop, OptimizedPin, OptimizedPinCollection, Pin,
};
use crate::sequential_logic::{Ram16k, Ram4k, Ram512, Ram8, Register};
use crate::test_utils::i16_to_bools;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

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

    pin_a.feed_from(&pin_b);
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

fn get_pin_idx(pin: &Rc<Pin>, pin_indices: &HashMap<Rc<Pin>, usize>) -> usize {
    *pin_indices.get(pin).unwrap()
}

fn optimize(pins: Vec<Rc<Pin>>) -> OptimizedPinCollection {
    let mut pin_indices = HashMap::new();
    for (pin_idx, pin) in pins.iter().enumerate() {
        pin_indices.insert(pin.clone(), pin_idx);
    }

    let mut result = OptimizedPinCollection {
        pins: vec![],
        flipflops: vec![],
        output_pins: vec![],
    };

    for pin in pins.iter() {
        match pin.connection.borrow().as_ref() {
            Some(Connection::Eq(other_pin)) => {
                let new_connection = OptimizedConnection::Eq(get_pin_idx(other_pin, &pin_indices));
                let new_pin = OptimizedPin {
                    connection: Some(new_connection),
                    value: pin.value.get(),
                };
                result.pins.push(new_pin);
            }
            Some(Connection::Nand(other_pin_a, other_pin_b)) => {
                let new_connection = OptimizedConnection::Nand(
                    get_pin_idx(other_pin_a, &pin_indices),
                    get_pin_idx(other_pin_b, &pin_indices),
                );
                let new_pin = OptimizedPin {
                    connection: Some(new_connection),
                    value: pin.value.get(),
                };
                result.pins.push(new_pin);
            }
            Some(Connection::FlipFlop(other_pin)) => {
                let new_pin = OptimizedPin {
                    connection: None,
                    value: pin.value.get(),
                };
                result.pins.push(new_pin);
                result.flipflops.push(OptimizedFlipFlop {
                    input: get_pin_idx(&pin, &pin_indices),
                    output: get_pin_idx(other_pin, &pin_indices),
                });
            }
            None => {
                let new_pin = OptimizedPin {
                    connection: None,
                    value: pin.value.get(),
                };
                result.pins.push(new_pin);
            }
        }
    }

    result
}

fn main() {
    println!("creating alu");
    let alu = ALU::new();

    let mut output_pins = alu.output.pins.to_vec();
    output_pins.push(alu.output_is_zero.clone());
    output_pins.push(alu.output_is_negative.clone());
    println!("getting connected pins");
    let all_pins = get_all_connected_pins(&output_pins.to_vec());
    let x = i16::MIN;
    let y = i16::MAX;
    alu.use_add.value.set(true);
    alu.not_output.value.set(false);
    alu.inputs[0].set_values(i16_to_bools(x));
    alu.inputs[1].set_values(i16_to_bools(y));

    println!("pin count: {}", all_pins.len());
    println!("getting all pin links");
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

    println!("sorting");
    let sorted_pins = reverse_topological_sort(&all_pins);
    println!("optimizing");
    let mut optimized_pins = optimize(sorted_pins);
    println!("computing");
    optimized_pins.compute();
    println!("ticking");
    optimized_pins.tick();
    println!("computing 10000x");
    let start = Instant::now();
    for i in 0..10000 {
        optimized_pins.compute();
    }
    println!("time elapsed: {:?}", start.elapsed());
    let result: Vec<bool> = output_pins.iter().map(|pin| pin.value.get()).collect();
    println!("{:?}", result);
}
