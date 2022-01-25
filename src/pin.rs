use std::cell::{Cell, RefCell};
use std::rc::Rc;

static mut PIN_COUNT: i32 = 0;

#[derive(Debug)]
enum Connection {
    Eq(Rc<Pin>),
    Nand(Rc<Pin>, Rc<Pin>),
}

#[derive(Debug, Default)]
pub struct Pin {
    debug_id: i32,
    pub value: Cell<bool>,
    connection: RefCell<Option<Connection>>,
}

impl Pin {
    pub fn new() -> Rc<Self> {
        unsafe {
            PIN_COUNT += 1;
            println!("create pin {}", PIN_COUNT);
            Rc::new(Self {
                debug_id: PIN_COUNT,
                value: Cell::new(false),
                connection: RefCell::new(None),
            })
        }
    }
    pub fn feed_from(&self, pin: Rc<Pin>) {
        let mut connection = self.connection.borrow_mut();
        if connection.as_ref().is_some() {
            panic!("pin is already connected");
        }
        connection.replace(Connection::Eq(pin));
    }
    pub fn nand_connect(&self, input_a: Rc<Pin>, input_b: Rc<Pin>) {
        self.connection
            .borrow_mut()
            .replace(Connection::Nand(input_a, input_b));
    }
    pub fn compute(&self) {
        println!("computing pin {}", self.debug_id);
        // TODO - this is an inefficient "pull" system - would be better
        // to do a toposort and then "push".
        let new_value = match self.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => {
                pin.compute();
                pin.value.get()
            }
            Some(Connection::Nand(pin_a, pin_b)) => {
                pin_a.compute();
                pin_b.compute();
                !(pin_a.value.get() && pin_b.value.get())
            }
            None => self.value.get(),
        };
        self.value.set(new_value);
    }
}

#[derive(Default, Debug)]
pub struct PinArray16 {
    pub pins: [Rc<Pin>; 16],
}

impl PinArray16 {
    pub fn new() -> Self {
        let mut pins: [Rc<Pin>; 16] = Default::default();
        for i in 0..16 {
            pins[i] = Pin::new();
        }
        Self { pins }
    }

    pub fn feed_from(&self, other: Self) {
        for (i, pin) in other.pins.into_iter().enumerate() {
            self.pins[i].feed_from(pin);
        }
    }

    pub fn compute(&self) {
        for pin in &self.pins {
            pin.compute();
        }
    }

    pub fn set_values(&self, values: [bool; 16]) {
        for i in 0..16 {
            self.pins[i].value.set(values[i]);
        }
    }

    pub fn clone(&self) -> Self {
        let mut pins: [Rc<Pin>; 16] = Default::default();
        for i in 0..16 {
            pins[i] = self.pins[i].clone();
        }
        Self { pins }
    }
}
