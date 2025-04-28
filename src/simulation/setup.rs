use bevy::{prelude::*, render::camera::ScalingMode};

use crate::simulation::FIXED_UPDATE_FREQUENCY;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.insert_resource(ClearColor(Color::WHITE));

        app.insert_resource(Time::<Fixed>::from_hz(FIXED_UPDATE_FREQUENCY));

        app.add_systems(FixedUpdate, camera_movement_controls);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.4,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn camera_movement_controls(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut orthographic_projection) = query.single_mut();

    let mut vertical = 0.0;
    let mut horizontal = 0.0;
    let mut zoom = 0.0;

    const MOVE_AMOUNT: f32 = 0.1;
    const ZOOM_AMOUNT: f32 = 0.0001;

    if keyboard_input.pressed(KeyCode::KeyW) {
        vertical += MOVE_AMOUNT;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        vertical -= MOVE_AMOUNT;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        horizontal -= MOVE_AMOUNT;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        horizontal += MOVE_AMOUNT;
    }

    if keyboard_input.pressed(KeyCode::KeyQ) {
        zoom -= ZOOM_AMOUNT;
    }

    if keyboard_input.pressed(KeyCode::KeyE) {
        zoom += ZOOM_AMOUNT;
    }

    if orthographic_projection.scale + zoom < 0.1 {
        orthographic_projection.scale = 0.1;
    } else {
        orthographic_projection.scale += zoom;
    }

    transform.translation.y += vertical;
    transform.translation.x += horizontal;
}
