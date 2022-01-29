mod boolean_arithmetic;
mod boolean_logic;
mod ordering;
mod pin;
mod sequential_logic;
mod test_utils;

use crate::ordering::{compute_all, get_all_connected_pins};
use crate::pin::{Connection, Pin};
use crate::sequential_logic::Ram512;
use crate::test_utils::i16_to_bools;
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

    let all_pins = get_all_connected_pins([pin_a.clone()].to_vec());
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

fn main() {
    println!("creating ram");
    let ram = Ram512::new();
    let all_pins = get_all_connected_pins(ram.output.pins.to_vec());
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
