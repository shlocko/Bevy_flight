use bevy::prelude::*;

use crate::{physics::Body, airfoil::Airfoil, camera::setup_camera};

pub struct Setup;

#[derive(Component)]
pub struct Player;

impl Plugin for Setup{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (setup.after(setup_camera)));
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
        ){
    let cam = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1000.0, 5000.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).id();
    let player = commands.spawn(SceneBundle {
        scene: asset_server.load("models/plane.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 1000.0, 0.0).with_scale(Vec3{x:0.005, y: 0.005, z:0.005}),
        ..Default::default()
    }).insert(Player)
    .insert(Body{
        mass: 800.0,
        velocity: Vec3::ZERO,
        force: Vec3::ZERO
    })
    .insert(Airfoil{
        cl: 0.0,
        cd: 0.0,
        ci: 0.0,
        max_thrust: 1000.0,
        cur_thrust: 1000.0
    }).id();
    commands.entity(player).push_children(&[cam]);
}
