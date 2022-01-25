use crate::pin::{Connection, Pin};
use std::collections::HashSet;
use std::rc::Rc;

fn topological_sort(all_pins: HashSet<Rc<Pin>>) -> Vec<Rc<Pin>> {
    let mut perm_marked = HashSet::new();
    let mut temp_marked = HashSet::new();
    let mut all_pins_iter = all_pins.into_iter();
    let mut result = Vec::new();

    fn visit(
        node: &Rc<Pin>,
        perm_marked: &mut HashSet<Rc<Pin>>,
        temp_marked: &mut HashSet<Rc<Pin>>,
        result: &mut Vec<Rc<Pin>>,
    ) {
        if perm_marked.contains(node) {
            return;
        }
        if temp_marked.contains(node) {
            panic!("not a dag");
        }

        temp_marked.insert(node.clone());

        match node.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => {
                visit(pin, perm_marked, temp_marked, result);
            }
            Some(Connection::Nand(pin_a, pin_b)) => {
                visit(pin_a, perm_marked, temp_marked, result);
                visit(pin_b, perm_marked, temp_marked, result);
            }
            None => {}
        }

        temp_marked.remove(node);
        perm_marked.insert(node.clone());
        result.push(node.clone());
    }

    while let Some(node) = all_pins_iter.next() {
        visit(&node, &mut perm_marked, &mut temp_marked, &mut result);
    }

    result
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
mod tests {
    use super::*;
    use crate::boolean_logic::TwoInOneOutGate;

    #[test]
    fn test_all_connected_pins_empty_set() {
        let outputs = Vec::new();
        let result = all_connected_pins(outputs);
        assert_eq!(result, HashSet::new());
    }

    #[test]
    fn test_all_connected_pins_single_pin() {
        let pin = Pin::new();
        let outputs = vec![pin.clone()];
        let result = all_connected_pins(outputs);
        let mut expected = HashSet::new();
        expected.insert(pin);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_connected_pins_simple_pair() {
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
    fn test_all_connected_pins_chain() {
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
    fn test_all_connected_pins_single_nand() {
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
    fn test_all_connected_pins_nand_tree() {
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
    fn test_all_connected_pins_disconnected_components() {
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
