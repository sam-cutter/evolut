use bevy::prelude::*;

use evolut::simulation::{CreaturePlugin, FoodPlugin, SetupPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(CreaturePlugin)
        .add_plugins(FoodPlugin)
        .run();
}

// Next steps:
// 1) do some refactoring - make sure that code is where it should be. don't worry about commenting and documentation yet.
// 2) decide how food will work in the world
// 3) write the function which randomly spawns in food
// 4) adapt the vision functions to allow food
// 5) figure out how creatures will consume food
