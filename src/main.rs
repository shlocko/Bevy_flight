use airfoil::AirfoilPlugin;
use bevy::prelude::*;
use input::InputPlugin;
use terrain::Terrain;
use setup::Setup;
use camera::CameraPlugin;
use physics::PhysicsPlugin;

mod setup;
mod terrain;
mod camera;
mod physics;
mod airfoil;
mod input;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Setup, Terrain, CameraPlugin, PhysicsPlugin, AirfoilPlugin, InputPlugin))
        .run();
}
