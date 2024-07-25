extern crate alloc;

use alloc::vec::Vec;
use core::f32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub weights: Vec<f32>,
    pub bias: f32,
}

impl Neuron {
    pub fn new(weights: Vec<f32>, bias: f32) -> Self {
        Self { weights, bias }
    }

    /* fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + -x.exp())
    } */

    fn relu(x: f32) -> f32 {
        if x > 0.0 {
            x
        } else {
            0.0
        }
    }

    pub fn forward(&self, inputs: &[f32]) -> f32 {
        let mut sum = self.bias;
        for (i, w) in self.weights.iter().enumerate() {
            sum += w * inputs[i];
        }
        Self::relu(sum)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }

    pub fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        self.neurons.iter().map(|n| n.forward(inputs)).collect()
    }
}
