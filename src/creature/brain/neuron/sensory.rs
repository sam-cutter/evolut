// TODO: Create structs for sensory variables
// TODO: Create a trait for sensory variables
// TODO: Implement trait for sensory variables
pub enum SensoryNeuron {
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
