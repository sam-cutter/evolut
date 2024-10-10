use std::rc::Rc;

struct Connection {
    input: InputNeuron,
    weight: f64,
}

enum InputNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}

struct SensoryNeuron {
    input: SensoryInput,
}

pub enum SensoryInput {
    Age,
    Speed,
    AngularVelocity,
    LineOfSight(LineOfSight),
    StoredEnergy,
}

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

struct InternalNeuron {
    inputs: Vec<Connection>,
}
struct ActionNeuron {
    inputs: Vec<Connection>,
    output: ActionOutput,
}

enum ActionOutput {
    Acceleration,
    AngularAcceleration,
}

enum Neuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
    Action(Rc<ActionNeuron>),
}

struct Brain {
    neurons: Vec<Neuron>,
}
