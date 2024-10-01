// TODO: Implement constructor for Connection
pub struct Connection {
    source: SourceNeuron,
    destination: DestinationNeuron,
    weight: f64,
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
