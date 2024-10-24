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
