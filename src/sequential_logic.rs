struct BitRegister {
    pub input: bool,
    pub load: bool,
    value: bool,
}

impl BitRegister {
    fn new() -> Self {
        Self {
            input: false,
            value: false,
            load: false,
        }
    }
    fn get_value(&self) -> bool {
        self.value
    }
    fn tick(&mut self) {
        if self.load {
            self.value = self.input;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bit_initialization() {
        let bit = BitRegister::new();
        assert_eq!(bit.input, false);
        assert_eq!(bit.load, false);
        assert_eq!(bit.get_value(), false);
    }

    #[test]
    fn test_bit_doesnt_change_with_load_unset() {
        let mut bit = BitRegister::new();
        bit.input = true;
        assert_eq!(bit.get_value(), false);
        bit.tick();
        assert_eq!(bit.get_value(), false);
    }

    #[test]
    fn test_bit_doesnt_change_without_tick() {
        let mut bit = BitRegister::new();
        bit.load = true;
        bit.input = true;
        assert_eq!(bit.get_value(), false);
    }

    #[test]
    fn test_changing_bit() {
        let mut bit = BitRegister::new();
        bit.load = true;
        bit.input = true;
        bit.tick();
        assert_eq!(bit.get_value(), true);
        bit.tick();
        assert_eq!(bit.get_value(), true);
        bit.input = false;
        bit.tick();
        assert_eq!(bit.get_value(), false);
    }
}
