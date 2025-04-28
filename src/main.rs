use bevy::prelude::*;

use evolut::simulation::{CreaturePlugin, FoodPlugin, SetupPlugin, SpatialIndexPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(CreaturePlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SpatialIndexPlugin)
        .run();
}
