use bevy::{
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::*,
};

pub struct Terrain;

impl Plugin for Terrain{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_terrain));
    }
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
                ){
    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.01, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.1), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });
    // Terrain
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/Mountains.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3{x: 10000.0, y: 10000.0, z: 10000.0}),
        ..default()
    });
}
