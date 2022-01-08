use std::cell::Cell;

struct SourcePin {
    value: Cell<bool>,
}

impl SourcePin {
    fn new(value: bool) -> Self {
        Self {
            value: Cell::new(value),
        }
    }

    fn set_value(&self, value: bool) {
        self.value.set(value);
    }
}

impl SourcePin {
    fn get_value(&self) -> bool {
        self.value.get()
    }
}

struct NandOutPin<'a> {
    in_a: &'a SourcePin,
    in_b: &'a SourcePin,
}

impl NandOutPin<'_> {
    fn get_value(&self) -> bool {
        !(self.in_a.get_value() && self.in_b.get_value())
    }
}

impl<'a> NandOutPin<'a> {
    fn new(in_a: &'a SourcePin, in_b: &'a SourcePin) -> Self {
        Self { in_a, in_b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_pin() {
        let pin = SourcePin::new(true);
        assert_eq!(pin.get_value(), true);
        pin.set_value(false);
        assert_eq!(pin.get_value(), false);
    }

    #[test]
    fn test_nand() {
        let pin_a = SourcePin::new(true);
        let pin_b = SourcePin::new(true);
        let nand_out = NandOutPin::new(&pin_a, &pin_b);
        assert_eq!(nand_out.get_value(), false);
        pin_a.set_value(false);
        pin_b.set_value(false);
        assert_eq!(nand_out.get_value(), true);
    }
}
