use bevy::prelude::*;

use evolut::simulation::creature::CreaturePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CreaturePlugin)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::WHITE))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
