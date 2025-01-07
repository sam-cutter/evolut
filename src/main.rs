use bevy::prelude::*;

use evolut::simulation::creature::CreaturePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CreaturePlugin)
        .run();
}
