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
