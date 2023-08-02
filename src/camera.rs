use bevy::prelude::*;
use crate::{setup::{Player, setup}, physics::step_physics};

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (setup_camera))
            .add_systems(Update, (update_camera));
    }
}

pub fn setup_camera(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>
               ){
    // Camera
    /*let plane = player.single();
    for (mut cam, _) in &mut camera{
        let mut offset = plane.back();
        offset *= 12.0;
        offset.y += 2.0;
        cam.look_at(plane.translation, plane.up());
        cam.translation = plane.translation + offset;
    }*/

}
fn update_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>
              ){
    let plane = player.single();
    for (mut cam, _) in &mut camera{
        let mut offset = plane.back();
        println!("{}", plane.back());
        offset *= 12.0;
        offset.y += 2.0;
        //println!("{}", offset);
        /*cam.translation = plane.translation;
        cam.translation += offset;*/
        //cam.look_at(plane.translation, plane.up());
        let lerp_factor = 0.1;
        //cam.translation = plane.translation + offset;
        //println!("{}",cam.translation);
    }
}
