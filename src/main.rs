use bevy::prelude::*;
use terrain::Terrain;
use setup::Setup;
use camera::CameraPlugin;

mod setup;
mod terrain;
mod camera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Setup, Terrain, CameraPlugin))
        .run();
}
