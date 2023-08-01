use bevy::prelude::*;
use crate::physics::{step_physics, Body};

#[derive(Component)]
pub struct Airfoil{
    pub cl: f32,
    pub max_thrust: f32,
    pub cur_thrust: f32,
    pub cd: f32,
    pub ci: f32,
}

pub struct AirfoilPlugin;

impl Plugin for AirfoilPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, (apply_thrust.before(step_physics), apply_lift.before(step_physics)));
    }
}

fn apply_thrust(
    mut query: Query<(&mut Body, &mut Airfoil, &mut Transform)>
               ){
    for (mut body, airfoil, mut transform) in &mut query{
        let mut force = Vec3{z: -airfoil.max_thrust, ..default()};
        force = transform.rotation.mul_vec3(force);
        body.force += force;
    }
}
fn apply_lift(
    mut query: Query<(&mut Body, &mut Airfoil, &Transform)>
             ){
    for (mut body, airfoil, transform) in &mut query{
        let mass = body.mass;
        body.force += Vec3{y: 9.81*mass, ..default()};
    }
}