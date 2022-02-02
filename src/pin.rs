use std::cell::{Cell, RefCell};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

thread_local!(static PIN_COUNT: Cell<u32> = Cell::new(0));

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Connection {
    Eq(Rc<Pin>),
    Nand(Rc<Pin>, Rc<Pin>),
    FlipFlop(Rc<Pin>),
}

impl Connection {
    pub fn pins(&self) -> Vec<Rc<Pin>> {
        match self {
            Self::Eq(pin) => {
                vec![pin.clone()]
            }
            Self::Nand(pin_a, pin_b) => {
                vec![pin_a.clone(), pin_b.clone()]
            }
            Self::FlipFlop(pin) => {
                vec![pin.clone()]
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pin {
    debug_id: u32,
    pub value: Cell<bool>,
    pub connection: RefCell<Option<Connection>>,
}

impl Pin {
    pub fn new() -> Rc<Self> {
        let mut debug_id = 0;
        PIN_COUNT.with(|thread_id| {
            debug_id = thread_id.get();
            thread_id.set(debug_id + 1);
        });
        Rc::new(Self {
            debug_id,
            value: Cell::new(false),
            connection: RefCell::new(None),
        })
    }
    pub fn feed_from(&self, pin: &Rc<Pin>) {
        let mut connection = self.connection.borrow_mut();
        if let Some(_) = connection.replace(Connection::Eq(pin.clone())) {
            panic!("pin is already connected");
        }
    }
    pub fn nand_connect(&self, input_a: Rc<Pin>, input_b: Rc<Pin>) {
        self.connection
            .borrow_mut()
            .replace(Connection::Nand(input_a, input_b));
    }
    pub fn flipflop_connect(&self, input: Rc<Pin>) {
        self.connection
            .borrow_mut()
            .replace(Connection::FlipFlop(input));
    }
    pub fn compute(&self) {
        let new_value = match self.connection.borrow().as_ref() {
            Some(Connection::Eq(pin)) => pin.value.get(),
            Some(Connection::Nand(pin_a, pin_b)) => !(pin_a.value.get() && pin_b.value.get()),
            _ => self.value.get(),
        };
        self.value.set(new_value);
    }
}

impl Hash for Pin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.debug_id.hash(state);
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

    pub fn feed_from(&self, other: &Self) {
        for (i, pin) in other.pins.clone().into_iter().enumerate() {
            self.pins[i].feed_from(&pin);
        }
    }

    pub fn get_values(&self) -> Vec<bool> {
        self.pins.iter().map(|pin| pin.value.get()).collect()
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

pub enum OptimizedConnection {
    Eq(usize),
    Nand(usize, usize),
}

pub struct OptimizedPin {
    pub connection: Option<OptimizedConnection>,
    pub value: bool,
}

pub struct OptimizedFlipFlop {
    pub input: usize,
    pub output: usize,
}

pub struct OptimizedPinCollection {
    pub pins: Vec<OptimizedPin>,
    pub flipflops: Vec<OptimizedFlipFlop>,
    pub output_pins: Vec<OptimizedPin>,
}

impl OptimizedPinCollection {
    pub fn compute(&mut self) {
        for pin_idx in 0..self.pins.len() {
            match self.pins[pin_idx].connection {
                Some(OptimizedConnection::Eq(other_pin_idx)) => {
                    self.pins[pin_idx].value = self.pins[other_pin_idx].value
                }
                Some(OptimizedConnection::Nand(pin_a_idx, pin_b_idx)) => {
                    self.pins[pin_idx].value =
                        !(self.pins[pin_a_idx].value && self.pins[pin_b_idx].value);
                }
                None => {}
            }
        }
    }

    pub fn tick(&mut self) {
        for flipflop in self.flipflops.iter() {
            self.pins[flipflop.output].value = self.pins[flipflop.input].value;
        }
    }
}
