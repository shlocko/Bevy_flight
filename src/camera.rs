use bevy::prelude::*;
use crate::setup::Player;

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
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>
              ){
    let plane = player.single();
    for (mut cam, _) in &mut camera{
        let mut offset = Vec3{z: 10.0, y: 2.0, x: 0.0};
        offset = plane.rotation.mul_vec3(offset);
        println!("{}", offset);
        cam.translation = plane.translation;
        cam.translation += offset;
        *cam = cam.looking_at(plane.translation, Vec3::Y);
    }
}
