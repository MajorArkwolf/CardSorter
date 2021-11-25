use crate::circuit::Circuit;

pub struct OverSeer {
    next_id: i64,
    circuits: Vec<super::circuit::CircuitObject>,
    sensors: Vec<super::sensor::Sensor>,
}

impl OverSeer {
    fn register_device(&mut self) -> i64{
        let id = self.next_id;
        self.next_id += 1;
        return id;
    }

    pub fn init(&mut self) {
        let feeder = super::circuit::Feeder::register(self.register_device());
        self.circuits.push(super::circuit::CircuitObject::Feeder(feeder));
        self.next_id = self.next_id + 1;
        let capture = super::circuit::Capture::register(self.register_device());
        self.circuits.push(super::circuit::CircuitObject::Capture(capture));
        self.next_id = self.next_id + 1;
    }

    pub fn run(&self, delta_time: std::time::Duration) -> bool {
        for circuit in self.circuits.iter() {
            match circuit {
                super::circuit::CircuitObject::Feeder(output) => output.tick(delta_time),
                super::circuit::CircuitObject::Capture(output) => output.tick(delta_time),
            }
        }
        return true;
    }
}

impl Default for OverSeer {
    fn default() -> Self {
        OverSeer {next_id: 0, circuits: vec!(), sensors: vec!()}
    }
}