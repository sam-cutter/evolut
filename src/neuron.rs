pub struct Connection<'a> {
    source: &'a SourceNeuron,
    destination: &'a DestinationNeuron,
    weight: f64,
}

impl<'a> Connection<'a> {
    pub fn new(source: &'a SourceNeuron, destination: &'a DestinationNeuron, weight: f64) -> Self {
        Self {
            source,
            destination,
            weight,
        }
    }
}

pub enum SourceNeuron {
    Sensory(SensoryNeuron),
    Internal(InternalNeuron),
}

pub enum DestinationNeuron {
    Internal(InternalNeuron),
    Action(ActionNeuron),
}

// TODO: Create structs for sensory variables
// TODO: Create a trait for sensory variables
// TODO: Implement trait for sensory variables
pub enum SensoryNeuron {
    Age,
    Speed,
    AngularVelocity,
    LinesOfSight,
    StoredEnergy,
}

pub struct InternalNeuron;

pub enum ActionNeuron {
    Acceleration,
    AngularAcceleration,
}

// TODO: Create function signatures for Neuron trait
// TODO: Implement Neuron trait for different Neuron types
pub trait Neuron {}

impl Neuron for SensoryNeuron {}

impl Neuron for InternalNeuron {}

impl Neuron for ActionNeuron {}
