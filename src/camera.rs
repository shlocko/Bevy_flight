use bevy::prelude::*;
use crate::setup::Move;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (setup_camera))
            .add_systems(Update, (update_camera));
    }
}

fn setup_camera(
    mut commands: Commands,
               ){
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2000.0, 20000.5, 50000.0).looking_at(Vec3::ZERO, Vec3::ZERO),
        ..default()
    });
}
fn update_camera(
    player: Query<&Transform, With<Move>>,
    mut camera: Query<(&mut Transform, &Camera), Without<Move>>
              ){
    let plane = player.single();
    for (mut cam, _) in &mut camera{
        cam.translation = plane.translation;
        cam.translation += Vec3{z: 10.0, y: 2.0, ..default()};
        *cam = cam.looking_at(plane.translation, Vec3::Y);
    }
}