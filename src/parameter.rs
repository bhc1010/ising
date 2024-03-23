#[derive(Debug)]
pub enum ParameterType {
    Temp,
    Coupling,
    MagMoment,
    MagFieldStrength,
}

#[derive(Debug)]
pub struct Parameter {
    value: u16,
    step: u16,
    bounds: (u16, u16),
    scaled_max: f64,
}

impl Parameter {
    pub fn new(value: u16, step: u16, bounds: (u16, u16), scaled_max: f64) -> Self {
        Self {
            value,
            step,
            bounds,
            scaled_max,
        }
    }

    pub fn increase_value(&mut self, value: u16) {
        if self.value <= self.bounds.1 - value {
            self.value += value;
        }
    }

    pub fn decrease_value(&mut self, value: u16) {
        if self.value >= self.bounds.0 + value {
            self.value -= value;
        }
    }

    pub fn normalized(&self) -> f64 {
        self.value as f64 / self.bounds.1 as f64
    }

    pub fn scaled(&self) -> f64 {
        self.normalized() * self.scaled_max
    }

    pub fn step(&self) -> &u16 {
        &self.step
    }
}