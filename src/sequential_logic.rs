// #[derive(Default)]
// struct BitRegister {
//     pub input: bool,
//     pub load: bool,
//     value: bool,
// }

// impl BitRegister {
//     fn new() -> Self {
//         Self {
//             input: false,
//             value: false,
//             load: false,
//         }
//     }
//     fn get_value(&self) -> bool {
//         self.value
//     }
//     fn tick(&mut self) {
//         if self.load {
//             self.value = self.input;
//         }
//     }
// }

// #[derive(Default)]
// struct Register {
//     bit_registers: [BitRegister; 16],
// }

// impl Register {
//     fn new() -> Self {
//         Self {
//             bit_registers: Default::default(),
//         }
//     }
//     fn set_input(&mut self, input: [bool; 16]) {
//         for (reg_idx, register) in self.bit_registers.iter_mut().enumerate() {
//             register.input = input[reg_idx];
//         }
//     }
//     fn set_load(&mut self, value: bool) {
//         for register in &mut self.bit_registers {
//             register.load = value;
//         }
//     }
//     fn get_value(&self) -> [bool; 16] {
//         let v: Vec<bool> = self
//             .bit_registers
//             .iter()
//             .map(|reg| reg.get_value())
//             .collect();
//         v.try_into().unwrap()
//     }
//     fn tick(&mut self) {
//         for register in &mut self.bit_registers {
//             register.tick()
//         }
//     }
// }

// struct Ram8 {
//     registers: [Register; 8],
//     input: [bool; 16],
//     address: [bool; 3],
// }

// impl Ram8 {
//     fn new() -> Self {
//         Self {
//             registers: Default::default(),
//             input: [false; 16],
//             address: [false; 3],
//         }
//     }

//     fn tick(&mut self) {
//         for register in &mut self.registers {
//             register.tick();
//         }
//     }

//     fn set_load(&self, value: bool) {
//         for register in self.registers {
//             register.set_load(value);
//         }
//     }

//     fn get_value(&self, address: [bool; 3]) -> [bool; 16] {
//         // TODO - should really do this using combinational logic...
//         self.registers[]
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::utils::binary;

//     #[test]
//     fn test_bit_initialization() {
//         let bit = BitRegister::new();
//         assert_eq!(bit.input, false);
//         assert_eq!(bit.load, false);
//         assert_eq!(bit.get_value(), false);
//     }

//     #[test]
//     fn test_bit_doesnt_change_with_load_unset() {
//         let mut bit = BitRegister::new();
//         bit.input = true;
//         assert_eq!(bit.get_value(), false);
//         bit.tick();
//         assert_eq!(bit.get_value(), false);
//     }

//     #[test]
//     fn test_bit_doesnt_change_without_tick() {
//         let mut bit = BitRegister::new();
//         bit.load = true;
//         bit.input = true;
//         assert_eq!(bit.get_value(), false);
//     }

//     #[test]
//     fn test_changing_bit() {
//         let mut bit = BitRegister::new();
//         bit.load = true;
//         bit.input = true;
//         bit.tick();
//         assert_eq!(bit.get_value(), true);
//         bit.tick();
//         assert_eq!(bit.get_value(), true);
//         bit.input = false;
//         bit.tick();
//         assert_eq!(bit.get_value(), false);
//     }

//     #[test]
//     fn test_register() {
//         let mut reg = Register::new();
//         assert_eq!(reg.get_value(), [false; 16]);
//         reg.set_input([true; 16]);
//         assert_eq!(reg.get_value(), [false; 16]);
//         reg.tick();
//         assert_eq!(reg.get_value(), [false; 16]);
//         reg.set_load(true);
//         assert_eq!(reg.get_value(), [false; 16]);
//         reg.tick();
//         assert_eq!(reg.get_value(), [true; 16]);
//     }

//     #[test]
//     fn test_ram8_load_memory() {
//         let mut ram = Ram8::new();
//         ram.set_load(true);
//         ram.input = binary(123);
//         ram.address = [true, false, false];
//         ram.tick();
//         assert_eq!(ram.get_value([true, false, false]), binary(123));
//     }
// }
