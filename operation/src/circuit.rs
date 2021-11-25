pub enum CircuitObject {
    Feeder(Feeder),
    Capture(Capture),
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Ready,
    Running,
    Disabled,
}

pub trait Circuit {
    fn get_id(&self) -> i64;
    fn init(&self) -> bool;
    fn tick(&self, delta_time: std::time::Duration);
    fn status(&self) -> Status;
    fn stop(&self);
}

pub struct Feeder {
    device_id: i64,
    sensor: Vec<super::sensor::Sensor>,
    status: Status,
}

impl Feeder {
    pub fn register(id: i64) -> Feeder {
        Feeder {
            device_id: id,
            sensor: vec!(),
            status: Status::Ready,
        }
    }
}

impl Circuit for Feeder {
    fn get_id(&self) -> i64 {
        return self.device_id;
    }

    fn init(&self) -> bool {
        return true;
    }

    fn tick(&self, delta_time: std::time::Duration) {
        println!("Tick tock: {0}", delta_time.as_nanos());
    }

    fn stop(&self) {

    }

    fn status(&self) -> Status {
        return self.status;
    }
}

pub struct Capture {
    device_id: i64,
    status: Status,
}

impl Capture {
    pub fn register(id: i64) -> Capture {
        Capture {
            device_id: id,
            status: Status::Ready,
        }
    }
}

impl Circuit for Capture {
    fn get_id(&self) -> i64 {
        return self.device_id;
    }

    fn init(&self) -> bool {
        return true;
    }

    fn tick(&self, delta_time: std::time::Duration) {
        println!("Tick tock: {0}", delta_time.as_nanos());
    }

    fn stop(&self) {

    }

    fn status(&self) -> Status {
        return self.status;
    }
}