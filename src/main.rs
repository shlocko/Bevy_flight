use bevy::prelude::*;
use terrain::Terrain;
use setup::Setup;
use camera::CameraPlugin;
use physics::PhysicsPlugin;

mod setup;
mod terrain;
mod camera;
mod physics;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Setup, Terrain, CameraPlugin, PhysicsPlugin))
        .run();
}
