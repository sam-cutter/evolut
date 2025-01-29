use std::{collections::HashMap, sync::Arc};

use super::{Activation, InternalNeuron};

/// The inputs to the neural network.
#[derive(Debug)]
pub struct SensoryNeuron {
    input: SensoryInput,
}

impl SensoryNeuron {
    /// Creates a new sensory neuron.
    pub fn new(sensory_neuron_id: u8) -> Self {
        let input = match sensory_neuron_id {
            0 => SensoryInput::Age,
            1 => SensoryInput::Speed,
            2 => SensoryInput::AngularVelocity,
            3..=11 => SensoryInput::LineOfSight(match sensory_neuron_id {
                3 => LineOfSight::LeftCreature,
                4 => LineOfSight::LeftFood,
                5 => LineOfSight::LeftObstacle,
                6 => LineOfSight::MiddleCreature,
                7 => LineOfSight::MiddleFood,
                8 => LineOfSight::MiddleObstacle,
                9 => LineOfSight::RightCreature,
                10 => LineOfSight::RightFood,
                11 => LineOfSight::RightObstacle,
                _ => unreachable!(),
            }),
            12 => SensoryInput::StoredEnergy,
            _ => unreachable!(),
        };

        Self { input }
    }

    /// Returns a reference to its input type.
    pub fn input(&self) -> &SensoryInput {
        &self.input
    }
}

impl Activation for SensoryNeuron {
    fn activation(
        &self,
        _internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f32>,
        sensory_inputs: &SensoryInputs,
    ) -> f32 {
        // TODO: compute the actual sensory input values.
        match self.input() {
            SensoryInput::Age => sensory_inputs.age,
            SensoryInput::AngularVelocity => sensory_inputs.angular_velocity,
            SensoryInput::LineOfSight(_) => 0.1,
            SensoryInput::Speed => sensory_inputs.speed,
            SensoryInput::StoredEnergy => sensory_inputs.stored_energy,
        }
    }
}

#[derive(Debug)]
pub enum SensoryInput {
    Age,
    Speed,
    AngularVelocity,
    LineOfSight(LineOfSight),
    StoredEnergy,
}

#[derive(Debug)]
pub enum LineOfSight {
    LeftCreature,
    LeftFood,
    LeftObstacle,
    MiddleCreature,
    MiddleFood,
    MiddleObstacle,
    RightCreature,
    RightFood,
    RightObstacle,
}

pub struct SensoryInputs {
    pub age: f32,
    pub speed: f32,
    pub angular_velocity: f32,
    pub lines_of_sight: LinesOfSight,
    pub stored_energy: f32,
}

#[derive(Default)]
pub struct LinesOfSight {
    pub left_creature: f32,
    pub left_food: f32,
    pub left_obstacle: f32,
    pub middle_creature: f32,
    pub middle_food: f32,
    pub middle_obstacle: f32,
    pub right_creature: f32,
    pub right_food: f32,
    pub right_obstacle: f32,
}
