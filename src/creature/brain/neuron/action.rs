use super::super::connection::Connection;

pub struct ActionNeuron {
    inputs: Vec<Connection>,
    output: ActionOutput,
}

pub enum ActionOutput {
    Acceleration,
    AngularAcceleration,
}
