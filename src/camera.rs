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
        let mut offset = plane.back();
        offset *= 8.0;
        offset.y += 2.0;
        //println!("{}", offset);
        cam.translation = plane.translation;
        cam.translation += offset;
        cam.look_at(plane.translation, plane.up());
    }
}
