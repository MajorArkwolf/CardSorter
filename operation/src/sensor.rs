pub enum SensorObject<T> {
    AnalogInput(AnalogInput<T>),
    AnalogOutput(AnalogOutput<T>),
    DigitalInput(DigitalInput<T>),
    DigitalOutput(DigitalOutput<T>)
}

pub struct Sensor {
    pub id: i64,
}

trait Analog {
    fn get_min<T>(self) -> T;
    fn get_max<T>(self) -> T;
    
}

trait GetSensor {
    fn get_value<T>(self) -> T;
}

trait SetSensor {
    fn set_value<T>(self, value: T);
}

pub struct AnalogInput<T> {
    sensor: Sensor,
    value: T,
    min: T,
    max: T,
}

pub struct AnalogOutput<T> {
    sensor: Sensor,
    last_value: T,
    min: T,
    max: T,
    command_sent: bool,
}

pub struct DigitalInput<T> {
    sensor: Sensor,
    value: T,
}

pub struct DigitalOutput<T> {
    sensor: Sensor,
    value: T,
    command_sent: bool, 
}

impl<T> AnalogInput<T> {
    fn new(sensor: Sensor, value: T, min: T, max: T) -> Self {
        return AnalogInput {sensor, value, min, max};
    }
    fn get_min(self) -> T {
        return self.min;
    }

    fn get_max(self) -> T {
        return self.max;
    }

    fn get_value(self) -> T {
        return self.value;
    }
}

impl<T> AnalogOutput<T> {
    fn new(sensor: Sensor, last_value: T, min: T, max: T) -> Self {
        return AnalogOutput {sensor, last_value, min, max, command_sent: true};
    }
    fn get_min(self) -> T {
        return self.min;
    }

    fn get_max(self) -> T {
        return self.max;
    }

    fn get_last_value(self) -> T {
        return self.last_value;
    }

    fn set_last_value(mut self, value: T) {
        self.last_value = value;
        self.command_sent = false;
    }
}

impl<T> DigitalInput<T> {
    fn new(sensor: Sensor, value: T) -> Self {
        return DigitalInput {sensor, value};
    }
}

impl<T> DigitalOutput<T> {
    fn new(sensor: Sensor, value: T) -> Self {
        return DigitalOutput {sensor, value, command_sent: false};
    }

    fn get_value(self) -> T {
        return self.value;
    }

    fn set_value(mut self, value: T) {
        self.command_sent = false;
        self.value = value;
    }
}