pub struct SensoryNeuron {
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
