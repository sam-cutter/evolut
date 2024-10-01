fn main() {
    println!("Hello, world!");
}

struct Connection {
    source: SourceNeuron,
    destination: DestinationNeuron,
    weight: f64,
}

enum SourceNeuron {
    Sensory(SensoryNeuron),
    Internal(InternalNeuron),
}

enum DestinationNeuron {
    Internal(InternalNeuron),
    Action(ActionNeuron),
}

enum SensoryNeuron {
    Age,
    Speed,
    AngularVelocity,
    LinesOfSight,
    StoredEnergy,
}

struct InternalNeuron;

enum ActionNeuron {
    Acceleration,
    AngularAcceleration,
}

trait Neuron {}

impl Neuron for SensoryNeuron {}

impl Neuron for InternalNeuron {}

impl Neuron for ActionNeuron {}
