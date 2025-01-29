use bevy::prelude::*;

use evolut::simulation::{CreaturePlugin, SetupPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(CreaturePlugin)
        .run();
}

/*
Next steps:
1 DONE) Implement some sort of timer for executing creature decisions.
2 DONE) Implement stored energy.
3) Implement the sensory neuron values.
4) Document code.
*/
