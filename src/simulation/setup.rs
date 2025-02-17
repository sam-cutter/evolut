use bevy::{prelude::*, render::camera::ScalingMode};

use crate::simulation::FIXED_UPDATE_FREQUENCY;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.insert_resource(ClearColor(Color::WHITE));

        app.insert_resource(Time::<Fixed>::from_hz(FIXED_UPDATE_FREQUENCY));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.05,
            ..OrthographicProjection::default_2d()
        },
    ));
}
