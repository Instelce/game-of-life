use bevy::prelude::*;

const CAMERA_SPEED: f32 = 25.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, move_camera);
    }
}


#[derive(Component)]
struct Camera;


fn spawn_camera(
    mut commands: Commands
) {
    // spawn camera
    commands.spawn((Camera2dBundle::default(), Camera, Name::new("Camera")));
}

fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let mut transform = camera.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += CAMERA_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= CAMERA_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += CAMERA_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= CAMERA_SPEED;
    }
}