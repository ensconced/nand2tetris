use crate::pin::{Connection, Pin};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn compute_all(output_pins: &[Rc<Pin>]) -> Vec<bool> {
    for pin in reverse_topological_sort(all_connected_pins(output_pins.to_vec())) {
        pin.compute();
    }
    output_pins.iter().map(|pin| pin.value.get()).collect()
}

fn reverse_topological_sort(all_pins: HashSet<Rc<Pin>>) -> Vec<Rc<Pin>> {
    let mut done = HashSet::new();
    let mut doing = HashSet::new();
    let mut all_pins_iter = all_pins.into_iter();
    let mut result = Vec::new();

    fn visit(
        node: &Rc<Pin>,
        done: &mut HashSet<Rc<Pin>>,
        doing: &mut HashSet<Rc<Pin>>,
        result: &mut Vec<Rc<Pin>>,
    ) {
        if done.contains(node) {
            return;
        }
        if doing.contains(node) {
            panic!("found a cycle");
        }

        doing.insert(node.clone());

        match node.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => {
                visit(pin, done, doing, result);
            }
            Some(Connection::Nand(pin_a, pin_b)) => {
                visit(pin_a, done, doing, result);
                visit(pin_b, done, doing, result);
            }
            None => {}
        }

        doing.remove(node);
        done.insert(node.clone());
        // pushing here instead of inserting at the start is the only thing that
        // makes this a "reverse" topological sort
        result.push(node.clone());
    }

    while let Some(node) = all_pins_iter.next() {
        visit(&node, &mut done, &mut doing, &mut result);
    }

    result
}

#[cfg(test)]
mod test_reverse_topological_sort {
    use crate::boolean_arithmetic::{Add16, FullAdder};
    use crate::boolean_logic::TwoInOneOutGate;

    use super::*;

    fn pins_are_in_order(pins: Vec<Rc<Pin>>) -> bool {
        let mut indices = HashMap::new();
        for (idx, pin) in pins.iter().enumerate() {
            indices.insert(pin, idx);
        }
        pins.iter().all(|pin| {
            let follows = |after_pin, before_pin| {
                if let Some(after_pin_idx) = indices.get(after_pin) {
                    if let Some(before_pin_idx) = indices.get(before_pin) {
                        return after_pin_idx > before_pin_idx;
                    } else {
                        panic!("couldn't find before pin");
                    }
                } else {
                    panic!("couldn't find after pin");
                }
            };

            match pin.connection.borrow().as_ref() {
                Some(Connection::Eq(pin_a)) => follows(pin, pin_a),
                Some(Connection::Nand(pin_a, pin_b)) => follows(pin, pin_a) && follows(pin, pin_b),
                None => true,
            }
        })
    }

    #[test]
    fn empty_set() {
        let pins = HashSet::new();
        let sorted = reverse_topological_sort(pins);
        assert_eq!(sorted, Vec::new());
    }

    #[test]
    fn single_pin() {
        let mut pins = HashSet::new();
        let pin = Pin::new();
        pins.insert(pin.clone());
        let sorted = reverse_topological_sort(pins);
        assert_eq!(sorted, vec![pin]);
    }

    #[test]
    fn simple_pair() {
        let pin_a = Pin::new();
        let pin_b = Pin::new();
        pin_a.feed_from(pin_b.clone());
        let sorted = reverse_topological_sort(all_connected_pins(vec![pin_a.clone()]));
        assert_eq!(sorted, vec![pin_b, pin_a]);
    }

    #[test]
    fn chain() {
        let output_pin = Pin::new();
        let mut current_pin = output_pin.clone();
        for _ in 0..10 {
            let pin = Pin::new();
            current_pin.feed_from(pin.clone());
            current_pin = pin;
        }
        let sorted = reverse_topological_sort(all_connected_pins(vec![output_pin]));
        assert_eq!(sorted.len(), 11);
        assert!(pins_are_in_order(sorted));
    }

    #[test]
    fn single_nand() {
        let nand = TwoInOneOutGate::nand();
        let sorted = reverse_topological_sort(all_connected_pins(vec![nand.output]));
        assert_eq!(sorted.len(), 3);
        assert!(pins_are_in_order(sorted));
    }

    #[test]
    fn single_xor() {
        let xor = TwoInOneOutGate::xor();
        let sorted = reverse_topological_sort(all_connected_pins(vec![xor.output]));
        // xor consists of 4 nands plus its own two input and single output pins
        assert_eq!(sorted.len(), 15);
        assert!(pins_are_in_order(sorted));
    }

    #[test]
    fn full_adder() {
        let full_adder = FullAdder::new();
        let sorted = reverse_topological_sort(all_connected_pins(full_adder.outputs.to_vec()));
        assert!(pins_are_in_order(sorted));
    }

    #[test]
    fn add16() {
        let add16 = Add16::new();
        let sorted = reverse_topological_sort(all_connected_pins(add16.output.pins.to_vec()));
        assert!(pins_are_in_order(sorted));
    }
}

fn all_connected_pins(outputs: Vec<Rc<Pin>>) -> HashSet<Rc<Pin>> {
    fn add_connected_pins(pin: Rc<Pin>, all_connected: &mut HashSet<Rc<Pin>>) {
        if all_connected.contains(&pin) {
            return;
        }
        all_connected.insert(pin.clone());
        match pin.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => add_connected_pins(pin.clone(), all_connected),
            Some(Connection::Nand(pin_a, pin_b)) => {
                add_connected_pins(pin_a.clone(), all_connected);
                add_connected_pins(pin_b.clone(), all_connected);
            }
            None => {}
        }
    }
    let mut all_connected = HashSet::new();
    for output in outputs {
        add_connected_pins(output, &mut all_connected);
    }
    all_connected
}

#[cfg(test)]
mod test_all_connected_pins {
    use super::*;
    use crate::boolean_logic::TwoInOneOutGate;

    #[test]
    fn empty_set() {
        let outputs = Vec::new();
        let result = all_connected_pins(outputs);
        assert_eq!(result, HashSet::new());
    }

    #[test]
    fn single_pin() {
        let pin = Pin::new();
        let outputs = vec![pin.clone()];
        let result = all_connected_pins(outputs);
        let mut expected = HashSet::new();
        expected.insert(pin);
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_pair() {
        let pin_a = Pin::new();
        let pin_b = Pin::new();
        pin_b.feed_from(pin_a.clone());
        let outputs = vec![pin_b.clone()];
        let result = all_connected_pins(outputs);
        let mut expected = HashSet::new();
        expected.insert(pin_a);
        expected.insert(pin_b);
        assert_eq!(result, expected);
    }

    #[test]
    fn chain() {
        let mut expected = HashSet::new();
        let first_pin = Pin::new();
        expected.insert(first_pin.clone());
        let mut pin = first_pin.clone();
        for _ in 0..10 {
            let next_pin = Pin::new();
            expected.insert(next_pin.clone());
            pin.feed_from(next_pin.clone());
            pin = next_pin;
        }
        let outputs = vec![first_pin];
        let result = all_connected_pins(outputs);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_nand() {
        let mut expected = HashSet::new();
        let nand = TwoInOneOutGate::nand();
        expected.insert(nand.output.clone());
        expected.insert(nand.inputs[0].clone());
        expected.insert(nand.inputs[1].clone());
        let outputs = vec![nand.output];
        let result = all_connected_pins(outputs);
        assert_eq!(result, expected);
    }

    #[test]
    fn nand_tree() {
        let mut expected = HashSet::new();
        let nand_a = TwoInOneOutGate::nand();
        let nand_b = TwoInOneOutGate::nand();
        let nand_c = TwoInOneOutGate::nand();
        nand_a.inputs[0].feed_from(nand_b.output.clone());
        nand_a.inputs[1].feed_from(nand_c.output.clone());
        let outputs = vec![nand_a.output.clone()];
        let result = all_connected_pins(outputs);
        for nand in [nand_a, nand_b, nand_c] {
            expected.insert(nand.output.clone());
            expected.insert(nand.inputs[0].clone());
            expected.insert(nand.inputs[1].clone());
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn disconnected_components() {
        let nands = [
            TwoInOneOutGate::nand(),
            TwoInOneOutGate::nand(),
            TwoInOneOutGate::nand(),
            TwoInOneOutGate::nand(),
        ];
        nands[0].inputs[0].feed_from(nands[1].output.clone());
        nands[2].inputs[0].feed_from(nands[3].output.clone());
        let mut expected = HashSet::new();
        for nand in nands.iter() {
            expected.insert(nand.output.clone());
            expected.insert(nand.inputs[0].clone());
            expected.insert(nand.inputs[1].clone());
        }
        let outputs = vec![nands[0].output.clone(), nands[2].output.clone()];
        let result = all_connected_pins(outputs);
        assert_eq!(result, expected);
    }
}
