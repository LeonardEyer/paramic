use std::f32::consts;
use super::parametric_equation::{EquationA, ParametricEquation};

pub trait Oscillator {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn set_frequency(&mut self, frequency: f32);
    fn sample(&mut self) -> f32;
}

pub struct SineOscillator {
    sample_rate: f32,
    phase: f32,
    frequency: f32,
}

impl SineOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            frequency: 440.0,
        }
    }
}

impl Oscillator for SineOscillator {

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn sample(&mut self) -> f32 {
        self.phase += self.frequency / self.sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        (self.phase * consts::TAU).sin()
    }
}

pub struct SquareOscillator {
    sample_rate: f32,
    phase: f32,
    frequency: f32,
}

impl SquareOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            frequency: 440.0,
        }
    }
}

impl Oscillator for SquareOscillator {

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn sample(&mut self) -> f32 {
        self.phase += self.frequency / self.sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
        if self.phase < 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

pub trait ParametricOscillator {
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn set_frequency(&mut self, frequency: f32);
    fn set_equation(&mut self, equation: EquationA);
    fn sample(&mut self) -> f32;
}

pub struct ParametricOscillatorA {
    sample_rate: f32,
    phase: f32,
    frequency: f32,
    equation: EquationA,
    period : f64,
}

impl ParametricOscillatorA {
    pub fn new(sample_rate: f32, equation: EquationA) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            frequency: 440.0,
            period: equation.get_period(),
            equation,
        }
    }
}

impl ParametricOscillator for ParametricOscillatorA {

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn set_equation(&mut self, equation: EquationA) {
        self.equation = equation;
    }

    fn sample(&mut self) -> f32 {
        self.phase += self.frequency / self.sample_rate;
        if self.phase > self.period as f32 {
            self.phase -= self.period as f32;
        }
        let (x, y) = self.equation.get_position(self.phase as f64);
        ((x.powi(2) + y.powi(2)).sqrt() - 1.0) as f32
    }
}